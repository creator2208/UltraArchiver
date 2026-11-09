#![windows_subsystem = "windows"]
mod archive_engine;
use eframe::egui;
use std::{path::PathBuf,sync::mpsc,thread};
unsafe extern "C" { fn ua_windows_build() -> u32; }
struct App { files:Vec<PathBuf>, output:String, format:String, level:u32, status:String, busy:bool, tx:mpsc::Sender<String>, rx:mpsc::Receiver<String>, page:usize }
fn desktop()->PathBuf { std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join("Desktop") }
fn run_create(files:Vec<PathBuf>,out:PathBuf,fmt:String,level:u32,tx:mpsc::Sender<String>){thread::spawn(move||{let r=archive_engine::create(&files,&out,&fmt,level);let _=tx.send(match r{Ok(())=>"Архів успішно створено".into(),Err(e)=>format!("Помилка: {e}")});});}
fn run_extract(src:PathBuf,dest:PathBuf,tx:mpsc::Sender<String>){thread::spawn(move||{let r=archive_engine::extract(&src,&dest);let _=tx.send(match r{Ok(())=>"Розпакування завершено".into(),Err(e)=>format!("Помилка: {e}")});});}
impl Default for App {fn default()->Self{let(tx,rx)=mpsc::channel();Self{files:vec![],output:desktop().join("Archive.zip").display().to_string(),format:"zip".into(),level:9,status:"Готово до роботи".into(),busy:false,tx,rx,page:0}}}
impl App {
 fn card(ui:&mut egui::Ui,title:&str,body:&str){egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(25,34,51)).rounding(egui::Rounding::same(12.0)).inner_margin(12.0).show(ui,|ui|{ui.label(egui::RichText::new(title).strong().color(egui::Color32::from_rgb(100,183,255)));ui.add_space(4.0);ui.label(body);});}
 fn action_button(ui:&mut egui::Ui,label:&str)->bool{ui.add(egui::Button::new(egui::RichText::new(label).strong()).fill(egui::Color32::from_rgb(20,105,220)).rounding(egui::Rounding::same(7.0))).clicked()}
}
impl eframe::App for App {
 fn update(&mut self,ctx:&egui::Context,_:&mut eframe::Frame){
  if let Ok(s)=self.rx.try_recv(){self.status=s;self.busy=false;}
  let dropped=ctx.input(|i|i.raw.dropped_files.clone());for f in dropped{if let Some(p)=f.path{self.files.push(p)}}
  egui::TopBottomPanel::top("top").exact_height(48.0).show(ctx,|ui|{ui.horizontal_centered(|ui|{ui.label(egui::RichText::new("▰").size(26.0).color(egui::Color32::from_rgb(48,153,255)));ui.label(egui::RichText::new("UltraArchiver").size(21.0).strong());ui.label(egui::RichText::new("2.2  •  Швидко · Потужно · Безпечно").color(egui::Color32::from_gray(155)));ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{ui.label("● Система готова");});});});
  egui::SidePanel::left("navigation").resizable(false).exact_width(178.0).show(ctx,|ui|{
   ui.add_space(14.0);ui.vertical_centered(|ui|{ui.label(egui::RichText::new("▰").size(48.0).color(egui::Color32::from_rgb(38,144,255)));ui.label(egui::RichText::new("ULTRA").size(18.0).strong());ui.label(egui::RichText::new("ARCHIVER").size(11.0).color(egui::Color32::from_gray(150)));});
   ui.add_space(22.0);
   for (i,label) in ["⌂  Головна","▣  Архіви","◫  Розпакувати","⚙  Налаштування"].iter().enumerate(){let selected=self.page==i;let b=egui::Button::new(egui::RichText::new(*label).color(if selected{egui::Color32::WHITE}else{egui::Color32::from_gray(175)})).fill(if selected{egui::Color32::from_rgb(22,91,164)}else{egui::Color32::TRANSPARENT}).rounding(egui::Rounding::same(7.0));if ui.add_sized([ui.available_width(),36.0],b).clicked(){self.page=i;}}
   ui.with_layout(egui::Layout::bottom_up(egui::Align::Min),|ui|{ui.separator();ui.label(egui::RichText::new("Версія 2.2.0 • Rust Engine").size(10.0).color(egui::Color32::from_gray(125)));});
  });
  egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::from_rgb(13,19,30)).inner_margin(18.0)).show(ctx,|ui|{
   ui.horizontal(|ui|{ui.heading(if self.page==0{"Вітаємо в UltraArchiver"}else if self.page==1{"Створення архіву"}else if self.page==2{"Розпакування"}else{"Налаштування"});ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{ui.label(egui::RichText::new("WINDOWS 10 / 11").size(10.0).color(egui::Color32::from_gray(135)));});});
   ui.label(egui::RichText::new("Архівування нового покоління — просто, швидко та надійно.").color(egui::Color32::from_gray(155)));
   ui.add_space(14.0);
   ui.columns(3,|cols|{Self::card(&mut cols[0],"⚡ Швидкість","Багатопотокове опрацювання файлів");Self::card(&mut cols[1],"🔒 Приватність","Архіви створюються локально на вашому ПК");Self::card(&mut cols[2],"🧩 Формати","ZIP · TAR · TAR.GZ — вбудований рушій");});
   ui.add_space(18.0);
   ui.horizontal(|ui|{ui.heading("Підтримувані формати");ui.label(egui::RichText::new("Вбудований рушій").small().color(egui::Color32::from_gray(130)));});
   ui.horizontal_wrapped(|ui|{for (name,ok) in [("ZIP",true),("TAR",true),("TGZ",true),("7Z",false),("RAR",false),("GZIP",false),("XZ",false),("BZ2",false),("WIM",false)]{let color=if ok{egui::Color32::from_rgb(19,104,184)}else{egui::Color32::from_rgb(45,52,64)};ui.label(egui::RichText::new(format!(" {} ",name)).strong().color(if ok{egui::Color32::WHITE}else{egui::Color32::from_gray(135)}).background_color(color));}});
   ui.add_space(18.0);
   ui.horizontal(|ui|{ui.heading("Файли для архіву");ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{if ui.button("Очистити").clicked(){self.files.clear();}if ui.button("＋ Додати папку").clicked(){if let Some(p)=rfd::FileDialog::new().pick_folder(){self.files.push(p)}}if ui.button("＋ Додати файли").clicked(){if let Some(v)=rfd::FileDialog::new().pick_files(){self.files.extend(v)}}});});
   egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(19,27,41)).rounding(egui::Rounding::same(10.0)).inner_margin(10.0).show(ui,|ui|{
    if self.files.is_empty(){ui.vertical_centered(|ui|{ui.add_space(14.0);ui.label(egui::RichText::new("⇧").size(34.0).color(egui::Color32::from_rgb(52,151,245)));ui.label(egui::RichText::new("Перетягніть файли сюди").strong());ui.label(egui::RichText::new("або скористайтеся кнопками додавання").small().color(egui::Color32::from_gray(140)));ui.add_space(14.0);});}
    else{egui::ScrollArea::vertical().max_height(110.0).show(ui,|ui|{let mut del=None;for(i,p)in self.files.iter().enumerate(){ui.horizontal(|ui|{ui.label("▤");ui.label(p.file_name().unwrap_or_default().to_string_lossy());ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{if ui.small_button("✕").clicked(){del=Some(i)}});});}if let Some(i)=del{self.files.remove(i);}});}
   });
   ui.add_space(12.0);
   ui.columns(2,|c|{
    c[0].label("Формат архіву");egui::ComboBox::from_id_salt("format").selected_text(self.format.to_uppercase()).show_ui(&mut c[0],|ui|{ui.selectable_value(&mut self.format,"zip".into(),"ZIP");ui.selectable_value(&mut self.format,"tar".into(),"TAR");ui.selectable_value(&mut self.format,"tar.gz".into(),"TAR.GZ");});
    c[1].label(format!("Рівень стиснення: {}",self.level));c[1].add(egui::Slider::new(&mut self.level,0..=9).show_value(false));
   });
   ui.add_space(8.0);ui.horizontal(|ui|{ui.label("Зберегти як");ui.text_edit_singleline(&mut self.output);if ui.button("Огляд…").clicked(){if let Some(p)=rfd::FileDialog::new().save_file(){self.output=p.display().to_string();}}});
   ui.add_space(12.0);ui.horizontal(|ui|{
    if ui.add_enabled(!self.busy&&!self.files.is_empty(),egui::Button::new(egui::RichText::new("  Створити архів  ").strong()).fill(egui::Color32::from_rgb(20,111,224))).clicked(){let p=PathBuf::from(&self.output);if p.exists(){self.status="Файл уже існує — виберіть інше ім’я".into();}else{self.busy=true;self.status="Архівування…".into();run_create(self.files.clone(),p,self.format.clone(),self.level,self.tx.clone());}}
    if ui.add_enabled(!self.busy,egui::Button::new("Розпакувати архів…")).clicked(){if let Some(a)=rfd::FileDialog::new().pick_file(){if let Some(d)=rfd::FileDialog::new().pick_folder(){self.busy=true;self.status="Розпакування…".into();run_extract(a,d,self.tx.clone());}}}
   });
   ui.add_space(10.0);ui.horizontal(|ui|{if self.busy{ui.spinner();}ui.label(&self.status);});
  });
 }
}
fn main()->eframe::Result<()>{let _=unsafe{ua_windows_build()};let mut app=App::default();let mut args=std::env::args_os().skip(1);if args.next().as_deref()==Some(std::ffi::OsStr::new("--add")){app.files.extend(args.map(PathBuf::from));}
 let mut opt=eframe::NativeOptions::default();opt.viewport=egui::ViewportBuilder::default().with_inner_size([1080.0,760.0]).with_min_inner_size([900.0,650.0]);
 eframe::run_native("UltraArchiver 2.2",opt,Box::new(|cc|{let mut s=(*cc.egui_ctx.style()).clone();s.visuals=egui::Visuals::dark();s.visuals.window_fill=egui::Color32::from_rgb(13,19,30);s.visuals.panel_fill=egui::Color32::from_rgb(16,24,37);s.visuals.widgets.noninteractive.bg_fill=egui::Color32::from_rgb(24,34,51);s.visuals.selection.bg_fill=egui::Color32::from_rgb(0,110,220);cc.egui_ctx.set_style(s);Ok(Box::new(app))}))
}
