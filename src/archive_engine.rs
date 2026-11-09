use std::{collections::HashMap,fs::{self,File},io::{self,Read,Write},path::{Path,PathBuf}};
use flate2::{read::GzDecoder,write::GzEncoder,Compression};
use walkdir::WalkDir;
use zip::{ZipArchive,ZipWriter,write::SimpleFileOptions};

fn safe_name(path:&Path)->String{path.components().filter_map(|c|match c{std::path::Component::Normal(s)=>Some(s.to_string_lossy().to_string()),_=>None}).collect::<Vec<_>>().join("/")}
fn entries(inputs:&[PathBuf])->Result<Vec<(PathBuf,String,u64)>,String>{let mut v=Vec::new();for input in inputs{let base=if input.is_dir(){input.parent().unwrap_or(input)}else{input.parent().unwrap_or(Path::new("."))};if input.is_file(){v.push((input.clone(),safe_name(input.strip_prefix(base).unwrap_or(input)),input.metadata().map_err(|e|e.to_string())?.len()));}else{for e in WalkDir::new(input).follow_links(false){let e=e.map_err(|e|e.to_string())?;if e.file_type().is_file(){let p=e.path().to_path_buf();let n=p.metadata().map_err(|e|e.to_string())?.len();v.push((p.clone(),safe_name(p.strip_prefix(base).map_err(|e|e.to_string())?),n));}}}}Ok(v)}

fn hash_file(path:&Path,done:&mut u64,total:u64,progress:&mut impl FnMut(u64,u64))->Result<[u8;32],String>{let mut f=File::open(path).map_err(|e|e.to_string())?;let mut h=blake3::Hasher::new();let mut b=[0u8;1024*1024];loop{let n=f.read(&mut b).map_err(|e|e.to_string())?;if n==0{break}h.update(&b[..n]);*done+=n as u64;progress(*done,total);}Ok(*h.finalize().as_bytes())}
fn write_u32(w:&mut impl Write,n:u32)->io::Result<()>{w.write_all(&n.to_le_bytes())}
fn write_u64(w:&mut impl Write,n:u64)->io::Result<()>{w.write_all(&n.to_le_bytes())}
fn read_u32(r:&mut impl Read)->io::Result<u32>{let mut b=[0;4];r.read_exact(&mut b)?;Ok(u32::from_le_bytes(b))}
fn read_u64(r:&mut impl Read)->io::Result<u64>{let mut b=[0;8];r.read_exact(&mut b)?;Ok(u64::from_le_bytes(b))}
fn create_uap<F:FnMut(u64,u64)>(files:&[(PathBuf,String,u64)],out:&Path,level:u32,mut progress:F)->Result<(u64,u64),String>{
 let total=files.iter().map(|x|x.2).sum::<u64>();let mut done=0u64;let mut writer=File::create(out).map_err(|e|e.to_string())?;
 writer.write_all(b"UAP1").map_err(|e|e.to_string())?;write_u32(&mut writer,files.len() as u32).map_err(|e|e.to_string())?;
 let mut seen:HashMap<[u8;32],u32>=HashMap::new();let mut temp_index=0u64;
 for (index,(path,name,size)) in files.iter().enumerate(){
  let hash=hash_file(path,&mut done,total,&mut progress)?;
  let mut method=0u8;let mut reference=u32::MAX;let mut payload_path=None;let mut payload_len=*size;
  if let Some(prev)=seen.get(&hash){method=2;reference=*prev;payload_len=0;}
  else{
   seen.insert(hash,index as u32);
   let temp=std::env::temp_dir().join(format!("ultraarchiver-{}-{}.zst",std::process::id(),temp_index));temp_index+=1;
   let input=File::open(path).map_err(|e|e.to_string())?;let output=File::create(&temp).map_err(|e|e.to_string())?;
   let compressed=zstd::stream::copy_encode(input,output,(level.max(1).min(19)) as i32).map_err(|e|e.to_string())?;
   if compressed<*size{method=1;payload_len=compressed;payload_path=Some(temp);}else{let _=fs::remove_file(&temp);payload_path=Some(path.clone());payload_len=*size;}
  }
  let nb=name.as_bytes();write_u32(&mut writer,nb.len() as u32).map_err(|e|e.to_string())?;writer.write_all(nb).map_err(|e|e.to_string())?;
  write_u64(&mut writer,*size).map_err(|e|e.to_string())?;writer.write_all(&hash).map_err(|e|e.to_string())?;writer.write_all(&[method]).map_err(|e|e.to_string())?;
  write_u32(&mut writer,reference).map_err(|e|e.to_string())?;write_u64(&mut writer,payload_len).map_err(|e|e.to_string())?;
  if let Some(p)=payload_path{let mut f=File::open(&p).map_err(|e|e.to_string())?;io::copy(&mut f,&mut writer).map_err(|e|e.to_string())?;if p!=*path{let _=fs::remove_file(p);}}
 }
 writer.flush().map_err(|e|e.to_string())?;let output=writer.metadata().map_err(|e|e.to_string())?.len();Ok((total,output))
}

pub fn create<F:FnMut(u64,u64)>(inputs:&[PathBuf],out:&Path,fmt:&str,level:u32,mut progress:F)->Result<(u64,u64),String>{
 if inputs.is_empty(){return Err("Немає файлів для архівування".into())}if out.exists(){return Err("Файл призначення вже існує".into())}
 let files=entries(inputs)?;if fmt=="uap"{return create_uap(&files,out,level,progress)}
 let total=files.iter().map(|x|x.2).sum::<u64>();let mut done=0u64;
 let result:Result<(),String>=match fmt{
 "zip"=>{let f=File::create(out).map_err(|e|e.to_string())?;let mut z=ZipWriter::new(f);let opt=SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).compression_level(Some(level.min(9) as i64));let mut buf=[0u8;1024*1024];for(p,name,_)in &files{z.start_file(name,opt).map_err(|e|e.to_string())?;let mut f=File::open(p).map_err(|e|e.to_string())?;loop{let n=f.read(&mut buf).map_err(|e|e.to_string())?;if n==0{break}z.write_all(&buf[..n]).map_err(|e|e.to_string())?;done+=n as u64;progress(done,total);}}z.finish().map_err(|e|e.to_string())?;Ok(())},
 "tar"|"tar.gz"=>{let f=File::create(out).map_err(|e|e.to_string())?;let writer:Box<dyn Write>=if fmt=="tar.gz"{Box::new(GzEncoder::new(f,Compression::new(level.min(9))))}else{Box::new(f)};let mut t=tar::Builder::new(writer);for(p,name,_)in &files{let mut f=File::open(p).map_err(|e|e.to_string())?;let mut h=tar::Header::new_gnu();h.set_path(name).map_err(|e|e.to_string())?;h.set_size(f.metadata().map_err(|e|e.to_string())?.len());h.set_mode(0o644);h.set_cksum();t.append(&h,&mut f).map_err(|e|e.to_string())?;done+=f.metadata().map_err(|e|e.to_string())?.len();progress(done,total);}t.finish().map_err(|e|e.to_string())?;Ok(())},
 _=>Err("Непідтримуваний формат".into())};result?;let output=out.metadata().map_err(|e|e.to_string())?.len();Ok((total,output))
}

fn safe_output(dest:&Path,name:&str)->Result<PathBuf,String>{let rel=Path::new(name);if rel.components().any(|c|!matches!(c,std::path::Component::Normal(_))){return Err("Небезпечний шлях усередині архіву".into())}Ok(dest.join(rel))}
fn extract_uap(src:&Path,dest:&Path)->Result<u64,String>{
 let mut r=File::open(src).map_err(|e|e.to_string())?;let mut magic=[0;4];r.read_exact(&mut magic).map_err(|e|e.to_string())?;if &magic!=b"UAP1"{return Err("Невідомий формат UltraPack".into())}
 let count=read_u32(&mut r).map_err(|e|e.to_string())?;if count>1_000_000{return Err("Занадто багато елементів".into())}
 fs::create_dir_all(dest).map_err(|e|e.to_string())?;let mut outputs:Vec<PathBuf>=Vec::new();let mut total=0u64;
 for _ in 0..count{
  let n=read_u32(&mut r).map_err(|e|e.to_string())? as usize;if n>1024*1024{return Err("Некоректна назва файлу".into())}
  let mut nb=vec![0;n];r.read_exact(&mut nb).map_err(|e|e.to_string())?;let name=String::from_utf8(nb).map_err(|e|e.to_string())?;
  let size=read_u64(&mut r).map_err(|e|e.to_string())?;let mut hash=[0;32];r.read_exact(&mut hash).map_err(|e|e.to_string())?;
  let mut mb=[0];r.read_exact(&mut mb).map_err(|e|e.to_string())?;let reference=read_u32(&mut r).map_err(|e|e.to_string())?;let payload=read_u64(&mut r).map_err(|e|e.to_string())?;
  let out=safe_output(dest,&name)?;if let Some(p)=out.parent(){fs::create_dir_all(p).map_err(|e|e.to_string())?;}
  match mb[0]{
   0=>{let mut o=File::create(&out).map_err(|e|e.to_string())?;io::copy(&mut r.take(payload),&mut o).map_err(|e|e.to_string())?;},
   1=>{let mut decoder=zstd::stream::read::Decoder::new(r.by_ref().take(payload)).map_err(|e|e.to_string())?;let mut o=File::create(&out).map_err(|e|e.to_string())?;io::copy(&mut decoder,&mut o).map_err(|e|e.to_string())?;},
   2=>{let p=outputs.get(reference as usize).ok_or("Некоректне посилання дедуплікації")?;fs::copy(p,&out).map_err(|e|e.to_string())?;},
   _=>return Err("Невідомий тип блока UltraPack".into())
  }
  let actual=fs::metadata(&out).map_err(|e|e.to_string())?.len();if actual!=size{return Err(format!("Перевірка розміру не пройдена: {name}"))}
  let mut f=File::open(&out).map_err(|e|e.to_string())?;let mut h=blake3::Hasher::new();let mut b=[0;1024*1024];loop{let n=f.read(&mut b).map_err(|e|e.to_string())?;if n==0{break}h.update(&b[..n]);}if h.finalize().as_bytes()!=&hash{return Err(format!("Контрольна сума не збігається: {name}"))}
  total+=actual;outputs.push(out);
 }
 Ok(total)
}

pub fn extract(src:&Path,dest:&Path)->Result<u64,String>{
 let ext=src.extension().and_then(|s|s.to_str()).unwrap_or("").to_ascii_lowercase();
 if ext=="uap"{return extract_uap(src,dest)}
 fs::create_dir_all(dest).map_err(|e|e.to_string())?;
 if ext=="zip"{let f=File::open(src).map_err(|e|e.to_string())?;let mut z=ZipArchive::new(f).map_err(|e|e.to_string())?;let mut total=0;for i in 0..z.len(){let mut file=z.by_index(i).map_err(|e|e.to_string())?;let Some(rel)=file.enclosed_name().map(|p|p.to_owned())else{continue};let out=dest.join(rel);if file.is_dir(){fs::create_dir_all(&out).map_err(|e|e.to_string())?;}else{if let Some(p)=out.parent(){fs::create_dir_all(p).map_err(|e|e.to_string())?;}let mut o=File::create(out).map_err(|e|e.to_string())?;total+=io::copy(&mut file,&mut o).map_err(|e|e.to_string())?;}}Ok(total)}
 else if src.to_string_lossy().to_ascii_lowercase().ends_with(".tar.gz")||ext=="tgz"{let f=File::open(src).map_err(|e|e.to_string())?;let mut a=tar::Archive::new(GzDecoder::new(f));a.unpack(dest).map_err(|e|e.to_string())?;Ok(0)}
 else if ext=="tar"{let f=File::open(src).map_err(|e|e.to_string())?;let mut a=tar::Archive::new(f);a.unpack(dest).map_err(|e|e.to_string())?;Ok(0)}
 else{Err("Підтримуються ZIP, TAR, TAR.GZ і UltraPack (.uap)".into())}
}
