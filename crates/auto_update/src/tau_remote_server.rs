//! Tau's remote servers come from Tau's own GitHub releases, because Zed's
//! cloud publishes servers only for Zed's versions.

use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient};
use semver::Version;
use sha2::{Digest as _, Sha256};
use smol::io::{AsyncReadExt as _, AsyncWriteExt as _};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

const RELEASES_URL: &str = "https://github.com/gopherlian/tau/releases/download";

/// The release asset with Tau's remote server for a host, such as
/// `tau-remote-server-linux-x86_64.gz`.
fn asset_name(os: &str, arch: &str) -> String {
    format!("{}-{os}-{arch}.gz", paths::REMOTE_SERVER_BINARY_PREFIX)
}

fn asset_url(version: &Version, os: &str, arch: &str) -> String {
    format!("{RELEASES_URL}/tau-v{version}/{}", asset_name(os, arch))
}

/// Returns a gzipped remote server for the host: the one the Linux package
/// bundles next to the editor binary, an earlier download from `cache_dir`, or a
/// new download whose SHA-256 matches the one published with the release.
pub(crate) async fn remote_server_path(
    http_client: Arc<dyn HttpClient>,
    version: Option<Version>,
    os: &str,
    arch: &str,
    executable_dir: Option<&Path>,
    cache_dir: &Path,
) -> Result<PathBuf> {
    // The bundled server comes from the same release as the editor, so it
    // always matches this build's Tau version.
    if let Some(bundled) = executable_dir
        .map(|executable_dir| executable_dir.join(asset_name(os, arch)))
        .filter(|path| path.is_file())
    {
        log::info!("using the remote server bundled at {bundled:?}");
        return Ok(bundled);
    }

    anyhow::ensure!(
        os == "linux",
        "Tau publishes its remote server only for Linux hosts, and this host runs {os}."
    );
    let version = version.context(
        "This Tau build has no Tau version, so it cannot download a remote server that matches it. \
         Development builds build theirs from source when ZED_BUILD_REMOTE_SERVER is set.",
    )?;

    let path = cache_dir.join(format!("{version}.gz"));
    if smol::fs::metadata(&path).await.is_ok() {
        return Ok(path);
    }

    smol::fs::create_dir_all(cache_dir).await?;
    let url = asset_url(&version, os, arch);
    log::info!("downloading the remote server from {url}");
    download_verified(http_client, &url, &path).await?;

    if let Err(error) =
        super::cleanup_remote_server_cache(cache_dir, &path, super::REMOTE_SERVER_CACHE_LIMIT).await
    {
        log::warn!("Failed to clean up remote server cache in {cache_dir:?}: {error:#}");
    }

    Ok(path)
}

async fn download_verified(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    destination: &Path,
) -> Result<()> {
    let checksum_url = format!("{url}.sha256");
    let checksum_file = String::from_utf8(get(http_client.as_ref(), &checksum_url).await?)
        .with_context(|| format!("{checksum_url} is not text"))?;
    let expected = checksum_file
        .split_whitespace()
        .next()
        .filter(|checksum| {
            checksum.len() == 64 && checksum.chars().all(|char| char.is_ascii_hexdigit())
        })
        .with_context(|| format!("{checksum_url} does not start with a SHA-256 checksum"))?
        .to_ascii_lowercase();

    let mut response = http_client
        .get(url, AsyncBody::default(), true)
        .await
        .with_context(|| format!("downloading {url}"))?;
    anyhow::ensure!(
        response.status().is_success(),
        "downloading {url} failed with {}",
        response.status()
    );

    let directory = destination
        .parent()
        .with_context(|| format!("{destination:?} has no parent directory"))?;
    // The download lands in a temporary file that is deleted unless it is
    // verified, so a corrupt or tampered server is never cached.
    let temporary = tempfile::NamedTempFile::new_in(directory)?;
    let mut file = smol::fs::File::create(temporary.path()).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    loop {
        let read = response.body_mut().read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read]).await?;
    }
    file.flush().await?;
    drop(file);

    let actual = format!("{:x}", hasher.finalize());
    anyhow::ensure!(
        actual == expected,
        "The remote server downloaded from {url} does not match the SHA-256 published with it \
         (expected {expected}, got {actual}), so it was not installed."
    );
    temporary.persist(destination)?;
    Ok(())
}

async fn get(http_client: &dyn HttpClient, url: &str) -> Result<Vec<u8>> {
    let mut response = http_client
        .get(url, AsyncBody::default(), true)
        .await
        .with_context(|| format!("downloading {url}"))?;
    anyhow::ensure!(
        response.status().is_success(),
        "downloading {url} failed with {}",
        response.status()
    );
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AutoUpdater;
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, Response};
    use release_channel::ReleaseChannel;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const SERVER: &[u8] = b"gzipped remote server";
    const ASSET_URL: &str = "https://github.com/gopherlian/tau/releases/download/tau-v0.2.0/tau-remote-server-linux-aarch64.gz";

    fn release_server(checksum: String, requests: Arc<AtomicUsize>) -> Arc<dyn HttpClient> {
        release_server_with_checksum_file(
            Some(format!("{checksum}  tau-remote-server-linux-aarch64.gz\n")),
            requests,
        )
    }

    fn release_server_with_checksum_file(
        checksum_file: Option<String>,
        requests: Arc<AtomicUsize>,
    ) -> Arc<dyn HttpClient> {
        FakeHttpClient::create(move |request| {
            let checksum_file = checksum_file.clone();
            let requests = requests.clone();
            async move {
                requests.fetch_add(1, Ordering::SeqCst);
                let url = request.uri().to_string();
                if url == ASSET_URL {
                    Ok(Response::builder().status(200).body(SERVER.into())?)
                } else if let Some(checksum_file) = checksum_file
                    && url == format!("{ASSET_URL}.sha256")
                {
                    Ok(Response::builder().status(200).body(checksum_file.into())?)
                } else {
                    Ok(Response::builder().status(404).body("".into())?)
                }
            }
        })
    }

    fn server_checksum() -> String {
        format!("{:x}", Sha256::digest(SERVER))
    }

    #[test]
    fn test_asset_url_points_at_tau_releases() {
        assert_eq!(
            asset_url(&Version::new(0, 2, 0), "linux", "aarch64"),
            ASSET_URL
        );
    }

    #[test]
    fn test_bundled_server_is_used_without_downloading() {
        let executable_dir = tempfile::tempdir().unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let bundled = executable_dir
            .path()
            .join("tau-remote-server-linux-x86_64.gz");
        std::fs::write(&bundled, SERVER).unwrap();
        let requests = Arc::new(AtomicUsize::new(0));

        let path = smol::block_on(remote_server_path(
            release_server(server_checksum(), requests.clone()),
            Some(Version::new(0, 2, 0)),
            "linux",
            "x86_64",
            Some(executable_dir.path()),
            cache_dir.path(),
        ))
        .unwrap();

        assert_eq!(path, bundled);
        assert_eq!(requests.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_download_is_verified_and_cached() {
        let executable_dir = tempfile::tempdir().unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));
        let http_client = release_server(server_checksum(), requests.clone());

        for _ in 0..2 {
            let path = smol::block_on(remote_server_path(
                http_client.clone(),
                Some(Version::new(0, 2, 0)),
                "linux",
                "aarch64",
                Some(executable_dir.path()),
                cache_dir.path(),
            ))
            .unwrap();

            assert_eq!(path, cache_dir.path().join("0.2.0.gz"));
            assert_eq!(std::fs::read(&path).unwrap(), SERVER);
        }
        assert_eq!(requests.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_checksum_mismatch_is_rejected() {
        let cache_dir = tempfile::tempdir().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));

        let error = smol::block_on(remote_server_path(
            release_server("0".repeat(64), requests),
            Some(Version::new(0, 2, 0)),
            "linux",
            "aarch64",
            None,
            cache_dir.path(),
        ))
        .unwrap_err();

        assert!(error.to_string().contains("SHA-256"), "{error}");
        assert_eq!(std::fs::read_dir(cache_dir.path()).unwrap().count(), 0);
    }

    #[test]
    fn test_missing_or_malformed_checksum_file_is_rejected() {
        for (checksum_file, message) in [
            (None, "404"),
            (
                Some("not a checksum\n".to_string()),
                "does not start with a SHA-256",
            ),
        ] {
            let cache_dir = tempfile::tempdir().unwrap();
            let requests = Arc::new(AtomicUsize::new(0));

            let error = smol::block_on(remote_server_path(
                release_server_with_checksum_file(checksum_file, requests.clone()),
                Some(Version::new(0, 2, 0)),
                "linux",
                "aarch64",
                None,
                cache_dir.path(),
            ))
            .unwrap_err();

            assert!(format!("{error:#}").contains(message), "{error:#}");
            assert_eq!(requests.load(Ordering::SeqCst), 1);
            assert_eq!(std::fs::read_dir(cache_dir.path()).unwrap().count(), 0);
        }
    }

    #[test]
    fn test_non_linux_host_is_rejected() {
        let cache_dir = tempfile::tempdir().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));

        let error = smol::block_on(remote_server_path(
            release_server(server_checksum(), requests.clone()),
            Some(Version::new(0, 2, 0)),
            "macos",
            "aarch64",
            None,
            cache_dir.path(),
        ))
        .unwrap_err();

        assert!(error.to_string().contains("only for Linux"), "{error}");
        assert_eq!(requests.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_download_needs_tau_version() {
        let cache_dir = tempfile::tempdir().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));

        let error = smol::block_on(remote_server_path(
            release_server(server_checksum(), requests.clone()),
            None,
            "linux",
            "aarch64",
            None,
            cache_dir.path(),
        ))
        .unwrap_err();

        assert!(error.to_string().contains("no Tau version"), "{error}");
        assert_eq!(requests.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn test_remote_hosts_never_download_the_server(cx: &mut TestAppContext) {
        let url = AutoUpdater::get_remote_server_release_url(
            ReleaseChannel::Stable,
            Some(Version::new(0, 2, 0)),
            "linux",
            "x86_64",
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        assert_eq!(url, None);
    }
}
