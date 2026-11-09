fn main() {
 println!("cargo:rerun-if-changed=native/sysinfo.c");
 println!("cargo:rerun-if-changed=assets/ultraarchiver.ico");
 cc::Build::new().file("native/sysinfo.c").compile("ua_sysinfo");
 if std::env::var("CARGO_CFG_TARGET_OS").as_deref()==Ok("windows") {
  let mut res=winres::WindowsResource::new();
  res.set_icon("assets/ultraarchiver.ico");
  res.set("ProductName","UltraArchiver");
  res.set("FileDescription","UltraArchiver - Archive Manager");
  res.compile().expect("Unable to embed UltraArchiver application icon");
 }
}
