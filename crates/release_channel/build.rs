fn main() {
    println!("cargo::rustc-check-cfg=cfg(__do_not_set_zed_release_channel)");
    println!("cargo::rerun-if-env-changed=ZED_RELEASE_CHANNEL");
    println!("cargo::rerun-if-env-changed=TAU_VERSION");

    // Enabling this `cfg` will cause a runtime panic if the
    // `ZED_RELEASE_CHANNEL` env var is not also set. TLDR: don't set the `cfg`
    // directly, just set the env var (hence the name).
    if std::env::var("ZED_RELEASE_CHANNEL").is_ok() {
        println!("cargo::rustc-cfg=__do_not_set_zed_release_channel");
    }

    // An invalid Tau version fails the build instead of leaving the app
    // without one.
    if let Ok(tau_version) = std::env::var("TAU_VERSION") {
        if let Err(error) = semver::Version::parse(&tau_version) {
            panic!(
                "TAU_VERSION '{tau_version}' is not a semantic version such as 0.2.0 or 0.2.0-rc.1: {error}"
            );
        }
        println!("cargo::rustc-env=TAU_VERSION={tau_version}");
    }
}
