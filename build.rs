fn main() {
 println!("cargo:rerun-if-changed=native/sysinfo.c");
 cc::Build::new().file("native/sysinfo.c").compile("ua_sysinfo");
}