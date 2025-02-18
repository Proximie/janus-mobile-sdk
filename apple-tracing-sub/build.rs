fn main() {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    cc::Build::new().file("ats_oslog.c").compile("ats_oslog");
}
