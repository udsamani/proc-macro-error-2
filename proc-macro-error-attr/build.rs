fn main() {
    println!("cargo:rustc-check-cfg=cfg(always_assert_unwind)");
    if version_check::is_max_version("1.36.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=always_assert_unwind");
    }
}
