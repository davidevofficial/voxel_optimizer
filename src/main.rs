#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! [main.rs] is the frontend and manager of the persistance of data, built using egui, 
//!it creates a native window to drop files and change settings to convert them using multithreading
//!to speed uo the process
mod vox_importer;
mod greedy_mesher;
mod vox_exporter;

//use rfd::FileDialog;
//use eframe;
use eframe::{egui};

use std::fs::read;
use std::fs::write;
use std::sync::{Arc, Mutex};
use std::thread;
//use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use eframe::egui::FontId;
use eframe::egui::RichText;
use crate::vox_importer::{is_valid_ply,is_vox};


/// Initiates the native window and calls the [`update`] method every frame. 
fn main() -> Result<(), eframe::Error> {

    println!("Hello, world!");
    
    //icon
    println!("WARNING: If it crashes right now it means that the src folder could not be found,
        make sure VoxelOptimizer.exe is in the same folder as the src folder");
    let bytes_png = read("src/icon.png").unwrap();
    let icon: eframe::IconData = eframe::IconData::try_from_png_bytes(&bytes_png).unwrap();
    /*let icon: eframe::IconData = eframe::IconData::from(IconData {
        rgba: vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255],
        width: 2,
        height: 2,
    });
     */
    let options = eframe::NativeOptions{
        drag_and_drop_support: true,
        initial_window_pos: Some(egui::pos2(300.0,100.0)),
        initial_window_size: Some(egui::vec2(1050.0, 600.0)),
        run_and_return: false,
        icon_data: Some(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Voxel optimizer | Davidevofficial",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
///Saves the data needed to run the app
/// # Contains
///
/// * Sender and Receiver (to send and receive status data from the processes)
/// * List of dropped_files and picked export path
/// * All the setttings for converting and optimize
#[derive(Clone, Debug)]
struct MyApp {
    sx: Sender<String>,
    rx: Arc<Mutex<Receiver<String>>>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    pub status: String,
    pub requestrepaint: bool,
    //settings
    monochrome: bool,
    pattern_matching: bool,
    is_texturesize_powerof2: bool,
    texturemapping_invisiblefaces: bool,
    manual_vt: bool,
    vt_precisionnumber: u8,
    background_color: [f32;3],
    debug_uv_mode: bool,
    cross: bool,
    cull_optimization: bool,
    y_is_up: bool,
    right_handed: bool,
    center_model_in_mesh: bool,
    normals: bool,
    //vox settings
    all_in_one_mesh: bool,
    transparency: bool,
    emission: bool,
    roughness: bool,
    metal: bool,
    refraction: bool,
    specular: bool,
    glass_creates_more_mesh:bool,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("title").show(ctx, |ui|{
            ui.vertical_centered(|ui|{
                ui.label(RichText::new("Voxel Optimizer").font(FontId::proportional(40.0)));
                ui.label(RichText::new("@davidevofficial - 2023").font(FontId::proportional(9.0)));
                ui.separator();
                ui.label("First change the settings and Drag-and-drop files onto the window.Click the convert button to convert them into an optimized .obj file.
                         send me an email at: davidevufficial@gmail.com or For more help check the documentation here:");
                ui.hyperlink_to("Github", "https://github.com/davidevofficial/voxel_optimizer/");
            });
        });
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui|{
            ui.horizontal(|ui|{
                //ui.label("Drag-and-drop files onto the window to import, click the button below to choose the export directory!");
                if ui.button("Click this button to choose the output directory!").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
                if let Some(picked_path) = &self.picked_path {
                    ui.horizontal(|ui| {
                        ui.label("The output folder is: ");
                        ui.monospace(picked_path);
                    });
                }//ui.label("Drag-and-drop files onto the window to import, click the button below to choose the export directory!");

            });
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");
                    egui::ScrollArea::vertical().id_source("Scroll1").max_height(200.0).show(ui,|ui|{
                        for file in &self.dropped_files {
                            let mut info = if let Some(path) = &file.path {
                                path.display().to_string()
                            } else if !file.name.is_empty() {
                                file.name.clone()
                            } else {
                                "???".to_owned()
                            };
                            if let Some(bytes) = &file.bytes {
                                use std::fmt::Write as _;
                                write!(info, " ({} bytes)", bytes.len()).ok();
                            }
                            ui.label(info);
                        }
                    });

                });
            }
            ui.horizontal(|ui|{
                if ui.button("Convert...").clicked() {
                    if self.picked_path.is_some() {
                        for i in &from_files_to_paths(self.dropped_files.clone()) {
                            if is_valid_ply(i) {
                                //println!("valid!");
                                self.status = format!("{}{}", String::from("Loading:"), i.to_string_lossy());
                                let i_clone = i.clone();
                                let mut my_app_clone = self.clone();
                                thread::spawn(move ||{
                                  greedy_mesher::convert(&mut my_app_clone, i_clone);
                                });
                                //thread::sleep(Duration::from_millis((2000/self.dropped_files.len()).try_into().unwrap()));
                                //greedy_mesher::convert(self, i);
                            } else if is_vox(i) {
                                self.status = format!("{}{}", String::from("Loading:"), i.to_string_lossy());
                                let i_clone = i.clone();
                                let mut my_app_clone = self.clone();
                                thread::spawn(move ||{
                                  greedy_mesher::convert_vox(&mut my_app_clone, i_clone);
                                });
                            }else{
                                println!("invalid!");
                                self.status = String::from("Invalid file/files!!!");
                            }
                        }
                    } else {
                        self.status = String::from("It is necessary to select an output folder, click the button above to do that! And if you haven't already drop the files onto the window")
                    }
                }
                    ui.label(&self.status);

            });

                /*
                if vox_importer::is_valid_ply(from_files_to_paths(self.dropped_files){
                    println!("Valid!");
                } else {
                    println!("Invalid!");
                }
                */
                //if  not ok then red label

        });
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.columns(2, |columns|{
            ui.columns(2, |columns|{
                //First column
                //Algorithm
                columns[0].separator();
                columns[0].hyperlink_to("Algorithm Options","https://github.com/davidevofficial/voxel_optimizer/#algorithm-options");
                columns[0].checkbox(&mut self.cross, "Enable cross-overlapping optimization");
                columns[0].checkbox(&mut self.monochrome, "Enable solid color faces to be one pixel on the texture map");
                columns[0].checkbox(&mut self.pattern_matching, "Enable Pattern Matching");
                columns[0].checkbox(&mut self.glass_creates_more_mesh, "Let Glass be more accurate (only for.vox)");
                columns[0].separator();
                //Export
                columns[0].hyperlink_to("Export Options","https://github.com/davidevofficial/voxel_optimizer/#export-options");
                columns[0].checkbox(&mut self.manual_vt, "Enable manual setting of the precision levels?");
                if self.manual_vt {
                    columns[0].add(egui::Slider::new(&mut self.vt_precisionnumber, 0..=15).text("Precision digits"));
                }else{
                    columns[0].label("");
                }
                columns[0].horizontal(|ui|{
                    ui.color_edit_button_rgb(&mut self.background_color);
                    ui.label("Select the background colour:");
                });
                columns[0].hyperlink_to("Coordinate system","https://github.com/davidevofficial/voxel_optimizer/assets/127616649/9c5fa9d9-6584-4475-af6d-90826c0d9a98");
                columns[0].with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui|{
                    ui.checkbox(&mut self.y_is_up, "Y-up");
                    ui.checkbox(&mut self.right_handed, "Right-Handed");
                });
                columns[0].checkbox(&mut self.center_model_in_mesh, "Origin is center of the model");
                columns[0].checkbox(&mut self.normals, "Enable normals on the final export");
                columns[0].separator();

                //second column
                //Debug Option
                columns[1].separator();
                columns[1].hyperlink_to("Debug Option","https://github.com/davidevofficial/voxel_optimizer/#enable-uv-debug-mode");
                columns[1].checkbox(&mut self.debug_uv_mode, "Enable uv debug mode");
                //PLY
                columns[1].separator();
                columns[1].hyperlink_to(".ply compatibility Option ","https://github.com/davidevofficial/voxel_optimizer/#ply-compatibility-options");
                columns[1].checkbox(&mut self.cull_optimization, "Enable de-cull optimization");
                columns[1].label("");

                //VOX
                columns[1].separator();
                columns[1].hyperlink_to(".vox specific Options","https://github.com/davidevofficial/voxel_optimizer/#vox-specific-options");
                columns[1].checkbox(&mut self.all_in_one_mesh, "Enable all the meshes to be in one file");
                columns[1].checkbox(&mut self.transparency, "Enable transparency");
                columns[1].checkbox(&mut self.emission, "Enable the creation of an emission map");
                columns[1].checkbox(&mut self.roughness, "Enable roughness to be in red channel of extra texture map");
                columns[1].checkbox(&mut self.metal, "Enable metal to be in green channel of extra texture map");
                columns[1].checkbox(&mut self.refraction, "Enable index of refraction to be in blue channel of extra texture map");
                columns[1].checkbox(&mut self.specular, "Enable specular to be in alpha channel of extra texture map");
                columns[1].separator();
            });
                //first column
                // Show dropped files (if any):
                //second column
                //ui.checkbox(&mut self.is_texturesize_powerof2, "Should the texture width and height both be a power of 2?");
                //ui.checkbox(&mut self.texturemapping_invisiblefaces, "Should invisible faces be on the texture map?");
                


        });
        preview_files_being_dropped(ctx);
        if !self.manual_vt{
            self.vt_precisionnumber = 0;
        }
        self.update_status();
        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() { self.dropped_files = i.raw.dropped_files.clone(); }
        });
        //save
        let mut b: Option<String> = None;
        if self.vt_precisionnumber < 10{b = Some(String::from("0"))}
        let c = format!("{},{},{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}"
                        , (self.monochrome as i32)
                        , self.pattern_matching as i32
                        , (self.manual_vt as i32)
                        , if b.is_some(){b.unwrap()}else{String::new()}
                        , (self.vt_precisionnumber as i32)
                        , (self.is_texturesize_powerof2 as i32)
                        , (self.texturemapping_invisiblefaces as i32)
                        , (self.cross as i32)
                        , (self.cull_optimization as i32)
                        , (self.y_is_up as i32)
                        , (self.center_model_in_mesh as i32)
                        , (self.all_in_one_mesh as i32)
                        , (self.transparency as i32)
                        , (self.emission as i32)
                        , (self.roughness as i32)
                        , (self.metal as i32)
                        , (self.refraction as i32)
                        , (self.specular as i32)
                        , (self.normals as i32)
                        , (self.glass_creates_more_mesh as i32)
                        , (self.right_handed as i32)
                        );
        write("src/options.txt", c).unwrap();
        //thread::sleep(Duration::from_millis(10));
        if self.requestrepaint{
            ctx.request_repaint();
            self.requestrepaint = false;
        }
        //ctx.request_repaint()
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>){panic!()}
}
impl MyApp {
    ///Receives the message from rx and
    ///
    ///* updates the status bar
    ///* Asks for a repaint (otherwise it would get stuck the status bar with no repaints)
    fn update_status(&mut self) {
        match self.rx.lock().expect("REASON").try_recv() {
            Ok(message) => {
                self.status = message;
                self.requestrepaint = true;
            }
            Err(_) => (),
        }
    }
    /*
    fn sav(&self){
        let c = format!("{},{},{},{},{},{},{}"
                        , (self.monochrome as i32).to_string()
                        , self.pattern_matching.to_string()
                        , (self.manual_vt as i32).to_string()
                        , (self.vt_precisionnumber as i32).to_string()
                        , (self.is_texturesize_powerof2 as i32).to_string()
                        , (self.texturemapping_invisiblefaces as i32).to_string()
                        , (self.cross as i32).to_string());
        write("src/options.txt", c).unwrap();
    }
    */

}
impl Default for MyApp{
    fn default() -> Self {
            let (sx, rx): (Sender<String>, Receiver<String>) = channel();
            let c = read("src/options.txt").unwrap();
            let m = c[0] == b'1';
            let fortyeight: u8 = 48; // '0' u8 representation in ascii
            let p = c[2] == b'1';
            let m_vt = c[4] == b'1';
            let vt_n = if c[6] == b'1' {10 + c[7]-fortyeight}else{c[7]-fortyeight};
            let tn_s = c[9] == b'1';
            let tx_f = c[11] == b'1';
            let cro = c[13] == b'1';
            let cu_o = c[15] == b'1';
            let y_up = c[17] == b'1';
            let cmm= c[19] == b'1';
            let all_in_one_mesh = c[21] == b'1'; 
            let transparency = c[23] == b'1'; 
            let emission = c[25] == b'1'; 
            let roughness = c[27] == b'1'; 
            let metal = c[29] == b'1'; 
            let refraction = c[31] == b'1';
            let specular = c[33] == b'1';  
            let normals = c[35] == b'1';
            let glass_creates_more_mesh = c[37] == b'1';
            let right_handed = c[39] == b'1';

        Self{
            sx,
            rx: Arc::new(Mutex::new(rx)),
            dropped_files: vec![],
            picked_path: None,
            status: "".to_string(),
            requestrepaint: false,
            monochrome: m,
            pattern_matching: p,
            is_texturesize_powerof2: tn_s,
            texturemapping_invisiblefaces: tx_f,
            manual_vt: m_vt,
            vt_precisionnumber: vt_n,
            background_color: [0.0,0.0,0.0],
            debug_uv_mode: false,
            cross: cro,
            cull_optimization: cu_o,
            y_is_up:y_up,
            center_model_in_mesh: cmm,
            all_in_one_mesh,
            transparency,
            emission,
            roughness,
            metal,
            refraction,
            specular,
            normals,
            glass_creates_more_mesh,
            right_handed,
        }
    }
}
/*


    fn load(){
        let c = read("src/options.txt").unwrap();
        self.monochrome = if c[0] == b'1' {true}else{false};
        let forty-eight: u8 = 48; // '0' u8 representation in ascii
        self.pattern_matching = (c[2] - &forty-eight) as i32;
        self.manual_vt = if c[4] == b'1' {true}else{false};
        self.vt_precisionnumber = if c[6] == b'1' {10 + c[7]-&fourtyeight}else{c[7]-&fourtyeight};
    }
*/
///Creates the semi-transparent black window for visualizing what you are dropping into the application
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });
        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
    }
///Convert an egui::DroppedFile to a std::path::PathBuf
fn from_files_to_paths(droppedfiles: Vec<egui::DroppedFile>) -> Vec<std::path::PathBuf>{
    let mut v: Vec<std::path::PathBuf> = vec![];
    for file in droppedfiles {if let Some(path) = file.path {
        v.push(path);
    }};
    v
}
///Same as [`from_files_to_paths`] but accepts only a string as an argument
fn from_string_to_path(pickedpath: String) -> Vec<std::path::PathBuf>{
    let v = vec![std::path::PathBuf::from(pickedpath)];
    v
    /*
    let mut v = vec![];
    v.push(std::path::PathBuf::from(pickedpath));
    v
    */
}

