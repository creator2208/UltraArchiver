use std::{fs::{self,File},path::{Path,PathBuf}};
use flate2::{read::GzDecoder,write::GzEncoder,Compression};
use walkdir::WalkDir;
use zip::{ZipArchive,ZipWriter,write::SimpleFileOptions};
fn safe_name(path:&Path)->String{path.components().filter_map(|c|match c{std::path::Component::Normal(s)=>Some(s.to_string_lossy().to_string()),_=>None}).collect::<Vec<_>>().join("/")}
pub fn create(inputs:&[PathBuf],out:&Path,fmt:&str,level:u32)->Result<(),String>{
 if inputs.is_empty(){return Err("Немає файлів для архівування".into())} if out.exists(){return Err("Файл призначення вже існує".into())}
 match fmt{
 "zip"=>{let f=File::create(out).map_err(|e|e.to_string())?;let mut z=ZipWriter::new(f);let opt=SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).compression_level(Some(level as i64));
 for input in inputs{let base=if input.is_dir(){input.parent().unwrap_or(input)}else{input.parent().unwrap_or(Path::new("."))};for ent in WalkDir::new(input).follow_links(false){let ent=ent.map_err(|e|e.to_string())?;let p=ent.path();if p.is_dir(){continue}let name=safe_name(p.strip_prefix(base).map_err(|e|e.to_string())?);z.start_file(name,opt).map_err(|e|e.to_string())?;let mut f=File::open(p).map_err(|e|e.to_string())?;std::io::copy(&mut f,&mut z).map_err(|e|e.to_string())?;}}
 z.finish().map_err(|e|e.to_string())?;Ok(())},
 "tar"|"tar.gz"=>{let f=File::create(out).map_err(|e|e.to_string())?;if fmt=="tar.gz"{let enc=GzEncoder::new(f,Compression::new(level));let mut t=tar::Builder::new(enc);for p in inputs{t.append_path_with_name(p,p.file_name().unwrap_or_default()).map_err(|e|e.to_string())?;}t.into_inner().map_err(|e|e.to_string())?.finish().map_err(|e|e.to_string())?;}else{let mut t=tar::Builder::new(f);for p in inputs{t.append_path_with_name(p,p.file_name().unwrap_or_default()).map_err(|e|e.to_string())?;}t.finish().map_err(|e|e.to_string())?;}Ok(())},
 _=>Err("Непідтримуваний формат".into())
 }
}
pub fn extract(src:&Path,dest:&Path)->Result<(),String>{
 fs::create_dir_all(dest).map_err(|e|e.to_string())?;let ext=src.extension().and_then(|s|s.to_str()).unwrap_or("").to_ascii_lowercase();
 if ext=="zip"{let f=File::open(src).map_err(|e|e.to_string())?;let mut z=ZipArchive::new(f).map_err(|e|e.to_string())?;for i in 0..z.len(){let mut file=z.by_index(i).map_err(|e|e.to_string())?;let Some(rel)=file.enclosed_name().map(|p|p.to_owned())else{continue};let out=dest.join(rel);if file.is_dir(){fs::create_dir_all(&out).map_err(|e|e.to_string())?;}else{if let Some(p)=out.parent(){fs::create_dir_all(p).map_err(|e|e.to_string())?;}let mut o=File::create(out).map_err(|e|e.to_string())?;std::io::copy(&mut file,&mut o).map_err(|e|e.to_string())?;}}Ok(())}
 else if src.to_string_lossy().to_ascii_lowercase().ends_with(".tar.gz")||ext=="tgz"{let f=File::open(src).map_err(|e|e.to_string())?;let dec=GzDecoder::new(f);let mut a=tar::Archive::new(dec);a.unpack(dest).map_err(|e|e.to_string())}
 else if ext=="tar"{let f=File::open(src).map_err(|e|e.to_string())?;let mut a=tar::Archive::new(f);a.unpack(dest).map_err(|e|e.to_string())}
 else{Err("Підтримуються ZIP, TAR і TAR.GZ".into())}
}
