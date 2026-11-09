#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use std::{path::{Path,PathBuf},process::Command,sync::mpsc,thread};
unsafe extern "C" { fn ua_windows_build() -> u32; }
struct App {
 files:Vec<PathBuf>, output:String, format:String, level:String,
 status:String, busy:bool, tx:mpsc::Sender<String>, rx:mpsc::Receiver<String>
}
fn desktop()->PathBuf { std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join("Desktop") }
fn engine()->PathBuf {
 let local=std::env::current_exe().unwrap_or_default().parent().unwrap_or(Path::new(".")).join("7z.exe");
 if local.exists(){local}else{PathBuf::from(r"C:\Program Files\7-Zip\7z.exe")}
}
fn run(args:Vec<String>,tx:mpsc::Sender<String>){
 thread::spawn(move||{
  let msg=match Command::new(engine()).args(args).output(){
   Ok(o) if o.status.success()=>"Операцію успішно завершено".to_string(),
   Ok(o)=>format!("Помилка 7-Zip: {}",String::from_utf8_lossy(&o.stderr)),
   Err(e)=>format!("Не вдалося запустити 7z.exe: {e}")
  }; let _=tx.send(msg);
 });
}
impl Default for App {
 fn default()->Self {let(tx,rx)=mpsc::channel();Self{files:vec![],output:desktop().join("Archive.7z").display().to_string(),format:"7z".into(),level:"9".into(),status:"Готово до роботи".into(),busy:false,tx,rx}}
}
impl eframe::App for App {
 fn update(&mut self,ctx:&egui::Context,_:&mut eframe::Frame){
  if let Ok(s)=self.rx.try_recv(){self.status=s;self.busy=false;}
  egui::TopBottomPanel::top("header").show(ctx,|ui|{ui.heading("UltraArchiver 2.0");ui.label("Архівування та розпакування • Windows 10 / 11");});
  egui::CentralPanel::default().show(ctx,|ui|{
   ui.horizontal(|ui|{
    ui.heading("Файли для архіву");
    if ui.button("＋ Додати файли").clicked(){if let Some(v)=rfd::FileDialog::new().pick_files(){self.files.extend(v)}}
    if ui.button("＋ Додати папку").clicked(){if let Some(v)=rfd::FileDialog::new().pick_folder(){self.files.push(v)}}
    if ui.button("Очистити").clicked(){self.files.clear();}
   });
   egui::ScrollArea::vertical().max_height(230.0).show(ui,|ui|{
    let mut del=None;for(i,p)in self.files.iter().enumerate(){ui.horizontal(|ui|{ui.label(p.display().to_string());if ui.small_button("✕").clicked(){del=Some(i)}});}
    if let Some(i)=del{self.files.remove(i);}
   });
   ui.separator();
   ui.horizontal(|ui|{
    ui.label("Формат архіву");
    egui::ComboBox::from_id_salt("format").selected_text(&self.format).show_ui(ui,|ui|for f in ["7z","zip","tar","wim","xz","gzip","bzip2"]{ui.selectable_value(&mut self.format,f.into(),f);});
    ui.label("Рівень стиснення");
    egui::ComboBox::from_id_salt("level").selected_text(&self.level).show_ui(ui,|ui|for(v,n)in[("0","Без стиснення"),("1","Швидкий"),("5","Збалансований"),("7","Сильний"),("9","Максимальний")]{ui.selectable_value(&mut self.level,v.into(),format!("{v} — {n}"));});
   });
   ui.horizontal(|ui|{ui.label("Зберегти як");ui.text_edit_singleline(&mut self.output);if ui.button("Огляд").clicked(){if let Some(p)=rfd::FileDialog::new().save_file(){self.output=p.display().to_string();}}});
   ui.horizontal(|ui|{
    if ui.add_enabled(!self.busy&&!self.files.is_empty(),egui::Button::new("Створити архів")).clicked(){
     let out=PathBuf::from(&self.output);
     if out.exists(){self.status="Цей файл уже існує. Виберіть інше ім’я.".into();}
     else{let mut a=vec!["a".into(),format!("-t{}",self.format),format!("-mx={}",self.level),out.display().to_string()];a.extend(self.files.iter().map(|p|p.display().to_string()));self.busy=true;self.status="Створення архіву…".into();run(a,self.tx.clone());}
    }
    if ui.add_enabled(!self.busy,egui::Button::new("Розпакувати архів")).clicked(){
     if let Some(a)=rfd::FileDialog::new().pick_file(){if let Some(d)=rfd::FileDialog::new().pick_folder(){self.busy=true;self.status="Розпакування…".into();run(vec!["x".into(),a.display().to_string(),format!("-o{}",d.display()),"-y".into()],self.tx.clone());}}
    }
   });
   ui.separator();ui.label(format!("Статус: {}",self.status));if self.busy{ui.add(egui::Spinner::new());}
  });
 }
}
fn main()->eframe::Result<()>{
 let _=unsafe{ua_windows_build()};
 let mut opt=eframe::NativeOptions::default();opt.viewport=egui::ViewportBuilder::default().with_inner_size([900.0,650.0]).with_min_inner_size([760.0,520.0]);
 eframe::run_native("UltraArchiver",opt,Box::new(|cc|{let mut s=(*cc.egui_ctx.style()).clone();s.visuals=egui::Visuals::dark();cc.egui_ctx.set_style(s);Ok(Box::<App>::default())}))
}
