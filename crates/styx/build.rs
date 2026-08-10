fn main() {
    let is_distribution = std::env::var("CARGO_FEATURE_DISTRIBUTION").is_ok();
    if is_distribution {
        let server = std::env::var("SSHX_DEFAULT_SERVER").unwrap_or_default();
        if server.is_empty() {
            panic!("distribution build requires SSHX_DEFAULT_SERVER environment variable to be set");
        }
        println!("cargo:rustc-env=SSHX_DEFAULT_SERVER={server}");
    }
    println!("cargo:rerun-if-env-changed=SSHX_DEFAULT_SERVER");
    println!("cargo:rerun-if-changed=build.rs");
}
