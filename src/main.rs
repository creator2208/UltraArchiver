#![windows_subsystem = "windows"]
mod archive_engine;
use eframe::egui;
use std::{path::PathBuf,sync::mpsc,thread};
unsafe extern "C" { fn ua_windows_build() -> u32; }
struct App { files:Vec<PathBuf>, output:String, format:String, level:u32, status:String, busy:bool, tx:mpsc::Sender<String>, rx:mpsc::Receiver<String> }
fn desktop()->PathBuf { std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join("Desktop") }
fn run_create(files:Vec<PathBuf>,out:PathBuf,fmt:String,level:u32,tx:mpsc::Sender<String>){thread::spawn(move||{let r=archive_engine::create(&files,&out,&fmt,level);let _=tx.send(match r{Ok(())=>"Архів успішно створено".into(),Err(e)=>format!("Помилка: {e}")});});}
fn run_extract(src:PathBuf,dest:PathBuf,tx:mpsc::Sender<String>){thread::spawn(move||{let r=archive_engine::extract(&src,&dest);let _=tx.send(match r{Ok(())=>"Розпакування завершено".into(),Err(e)=>format!("Помилка: {e}")});});}
impl Default for App {fn default()->Self{let(tx,rx)=mpsc::channel();Self{files:vec![],output:desktop().join("Archive.zip").display().to_string(),format:"zip".into(),level:9,status:"Готово до роботи".into(),busy:false,tx,rx}}}
impl eframe::App for App {
 fn update(&mut self,ctx:&egui::Context,_:&mut eframe::Frame){
  if let Ok(s)=self.rx.try_recv(){self.status=s;self.busy=false;}
  egui::TopBottomPanel::top("top").show(ctx,|ui|{ui.horizontal(|ui|{ui.heading("🗜 UltraArchiver");ui.label("2.1 • Власний Rust Engine");ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{ui.label("Windows 10 / 11");});});});
  egui::SidePanel::left("side").resizable(false).default_width(155.0).show(ctx,|ui|{ui.add_space(12.0);ui.heading("UltraArchiver");ui.separator();ui.label("▣  Архівування");ui.label("▤  Розпакування");ui.label("⚙  Налаштування");ui.with_layout(egui::Layout::bottom_up(egui::Align::Min),|ui|{ui.label("Версія 2.1.0");});});
  egui::CentralPanel::default().show(ctx,|ui|{
   ui.heading("Створити архів");ui.label("Додайте файли або папки, які потрібно стиснути.");
   ui.horizontal(|ui|{if ui.button("＋ Додати файли").clicked(){if let Some(v)=rfd::FileDialog::new().pick_files(){self.files.extend(v)}}if ui.button("＋ Додати папку").clicked(){if let Some(v)=rfd::FileDialog::new().pick_folder(){self.files.push(v)}}if ui.button("Очистити").clicked(){self.files.clear();}});
   egui::Frame::group(ui.style()).show(ui,|ui|{egui::ScrollArea::vertical().max_height(220.0).show(ui,|ui|{let mut del=None;for(i,p)in self.files.iter().enumerate(){ui.horizontal(|ui|{ui.label("📄");ui.label(p.display().to_string());if ui.small_button("✕").clicked(){del=Some(i)}});}if let Some(i)=del{self.files.remove(i);}});});
   ui.add_space(8.0);ui.horizontal(|ui|{ui.label("Формат");egui::ComboBox::from_id_salt("fmt").selected_text(&self.format).show_ui(ui,|ui|{ui.selectable_value(&mut self.format,"zip".into(),"ZIP");ui.selectable_value(&mut self.format,"tar".into(),"TAR");ui.selectable_value(&mut self.format,"tar.gz".into(),"TAR.GZ");});ui.label("Стиснення");ui.add(egui::Slider::new(&mut self.level,0..=9).text("рівень"));});
   ui.horizontal(|ui|{ui.label("Файл архіву");ui.text_edit_singleline(&mut self.output);if ui.button("Огляд…").clicked(){if let Some(p)=rfd::FileDialog::new().save_file(){self.output=p.display().to_string();}}});
   ui.add_space(10.0);ui.horizontal(|ui|{if ui.add_enabled(!self.busy&&!self.files.is_empty(),egui::Button::new("Створити архів")).clicked(){let p=PathBuf::from(&self.output);if p.exists(){self.status="Файл уже існує — виберіть інше ім’я".into();}else{self.busy=true;self.status="Архівування…".into();run_create(self.files.clone(),p,self.format.clone(),self.level,self.tx.clone());}}
   if ui.add_enabled(!self.busy,egui::Button::new("Розпакувати…")).clicked(){if let Some(a)=rfd::FileDialog::new().pick_file(){if let Some(d)=rfd::FileDialog::new().pick_folder(){self.busy=true;self.status="Розпакування…".into();run_extract(a,d,self.tx.clone());}}}});
   ui.separator();ui.horizontal(|ui|{if self.busy{ui.spinner();}ui.label(&self.status);});
  });
 }
}
fn main()->eframe::Result<()>{let _=unsafe{ua_windows_build()};let mut app=App::default();let mut args=std::env::args_os().skip(1);if args.next().as_deref()==Some(std::ffi::OsStr::new("--add")){app.files.extend(args.map(PathBuf::from));}
 let mut opt=eframe::NativeOptions::default();opt.viewport=egui::ViewportBuilder::default().with_inner_size([960.0,680.0]).with_min_inner_size([800.0,560.0]);
 eframe::run_native("UltraArchiver 2.1",opt,Box::new(|cc|{let mut s=(*cc.egui_ctx.style()).clone();s.visuals=egui::Visuals::dark();s.visuals.widgets.noninteractive.bg_fill=egui::Color32::from_rgb(20,27,40);s.visuals.selection.bg_fill=egui::Color32::from_rgb(0,110,220);cc.egui_ctx.set_style(s);Ok(Box::new(app))}))
}
