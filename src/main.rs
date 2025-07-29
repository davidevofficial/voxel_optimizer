#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! [main.rs] is the frontend and manager of the persistance of data, built using egui,
//!it creates a native window to drop files and change settings to convert them using multithreading
//!to speed uo the process
mod vox_importer;
mod greedy_mesher;
mod vox_exporter;

//use rfd::FileDialog;
//use eframe;
use eframe::egui;

use std::fs::read;
use std::fs::write;
use std::io::BufRead;
use std::path::PathBuf;
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
    println!("WARNING: If it crashes without displaying a window it means that the src folder or the contents inside could not be found, make sure VoxelOptimizer.exe is in the same folder as the src folder and unzipped.

    For example:

    VoxelOptimizer/
    ├── voxeloptimizer.exe
    ├── voxeloptimizer_console_version.exe (Run this for debug... you are running it already)
    ├── voxeloptimizer.zip (Optional)
    └── src/
        ├── Icon.png
        └── options.txt

    ------------------------------------------------------------------------------------------------------------------    ");
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
    picked_file: Option<String>,
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
    custom_export_size: bool,
    sizex: f32,
    sizey: f32,
    sizez: f32,
    detailed_export_name: bool,
    uv_extra_precision: bool,
    //vox settings
    all_in_one_mesh: bool,
    transparency: bool,
    emission: bool,
    realistic_lightning: bool,
    roughness: bool,
    metallic: bool,
    refraction: bool,
    specular: bool,
    glass_creates_more_mesh:bool,
    export_invisible: bool,
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
                ui.label("Version is 2.0.10");
                ui.hyperlink_to("Check for updates",
                    "https://raw.githubusercontent.com/davidevofficial/voxel_optimizer/refs/heads/master/src/version.txt");


            });
        });
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui|{
        	ui.horizontal(|ui|{
		         if ui.button("Open file or drag and drop…").clicked() {
		             if let Some(path) = rfd::FileDialog::new().pick_file() {
		                 self.picked_file = Some(path.display().to_string());
		             }
		         }
		         if let Some(picked_file) = &self.picked_file {
		             ui.horizontal(|ui| {
		                 ui.label("Picked file:");
		                 ui.monospace(picked_file);
		             });
		         }
        	});
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
                        if self.picked_file.is_some(){
                        	let i = &PathBuf::from(&self.picked_file.clone().unwrap());
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
                let r = columns[0].checkbox(&mut self.cross, "Enable cross-overlapping optimization");
                if r.changed() {
                    change_options("cross", &self.cross.to_string());
                }
                let r = columns[0].checkbox(&mut self.monochrome, "Enable solid color faces to be one pixel on the texture map");
                if r.changed() {
                    change_options("monochrome", &self.monochrome.to_string());
                }
                let r = columns[0].checkbox(&mut self.pattern_matching, "Enable Pattern Matching");
                if r.changed() {
                    change_options("pattern_matching", &self.pattern_matching.to_string());
                }
                let r = columns[0].checkbox(&mut self.glass_creates_more_mesh, "Let Glass be more accurate (only for.vox)");
                if r.changed() {
                    change_options("glass_creates_more_mesh", &self.glass_creates_more_mesh.to_string());
                }
                columns[0].separator();
                //Export
                columns[0].hyperlink_to("Export Options","https://github.com/davidevofficial/voxel_optimizer/#export-options");
                let r = columns[0].checkbox(&mut self.manual_vt, "Enable manual setting of the precision levels?");
                if r.changed() {
                    change_options("manual_vt", &self.manual_vt.to_string());
                }
                if self.manual_vt {
                    let r = columns[0].add(egui::Slider::new(&mut self.vt_precisionnumber, 0..=15).text("Precision digits"));
                    if r.changed() {
                        change_options("vt_precisionnumber", &self.vt_precisionnumber.to_string());
                    }
                }else{
                    columns[0].label("");
                }
                columns[0].horizontal(|ui|{
                    ui.color_edit_button_rgb(&mut self.background_color);
                    ui.label("Select the background colour:");
                });
                columns[0].hyperlink_to("Coordinate system","https://github.com/davidevofficial/voxel_optimizer/assets/127616649/9c5fa9d9-6584-4475-af6d-90826c0d9a98");
                columns[0].with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui|{
                    let r = ui.checkbox(&mut self.y_is_up, "Y-up");
                    if r.changed() {
                        change_options("y_is_up", &self.y_is_up.to_string());
                    }
                    let r = ui.checkbox(&mut self.right_handed, "Right-Handed");
                    if r.changed() {
                        change_options("right_handed", &self.right_handed.to_string());
                    }
                });
                let r = columns[0].checkbox(&mut self.center_model_in_mesh, "Origin is center of the model");
                if r.changed() {
                    change_options("center_model_in_mesh", &self.center_model_in_mesh.to_string());
                }
                let r = columns[0].checkbox(&mut self.normals, "Enable normals on the final export");
                if r.changed() {
                    change_options("normals", &self.normals.to_string());
                }
                let r = columns[0].checkbox(&mut self.custom_export_size, "Enable custom export scale for the model");
                if r.changed() {
                    change_options("custom_export_size", &self.custom_export_size.to_string());
                }

                if self.custom_export_size{
                    columns[0].with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui|{
                        let r = ui.add(egui::Slider::new(&mut self.sizex, 0.001..=100.0));
                        if r.changed() {
                            change_options("sizex", &self.sizex.to_string());
                        }
                        let r = ui.add(egui::Slider::new(&mut self.sizey, 0.001..=100.0));
                        if r.changed() {
                            change_options("sizey", &self.sizey.to_string());
                        }
                        let r = ui.add(egui::Slider::new(&mut self.sizez, 0.001..=100.0));
                        if r.changed() {
                            change_options("sizez", &self.sizez.to_string());
                        }
                    });
                }

                columns[0].separator();

                //second column
                //Debug Option
                columns[1].separator();
                columns[1].hyperlink_to("Debug Option","https://github.com/davidevofficial/voxel_optimizer/#enable-uv-debug-mode");
                let r = columns[1].checkbox(&mut self.debug_uv_mode, "Enable uv debug mode");
                if r.changed() {
                    change_options("debug_uv_mode", &self.debug_uv_mode.to_string());
                }
                //PLY
                columns[1].separator();
                columns[1].hyperlink_to("Compatibility Options ","https://github.com/davidevofficial/voxel_optimizer/#compatibility-options");
                let r = columns[1].checkbox(&mut self.cull_optimization, "Enable de-cull optimization");
                if r.changed() {
                    change_options("cull_optimization", &self.cull_optimization.to_string());
                }
                let r = columns[1].checkbox(&mut self.uv_extra_precision, "Enable UV extra precision");
                if r.changed() {
                    change_options("uv_extra_precision", &self.uv_extra_precision.to_string());
                }


                //VOX
                columns[1].separator();
                columns[1].hyperlink_to(".vox specific Options","https://github.com/davidevofficial/voxel_optimizer/#vox-specific-options");
                let r = columns[1].checkbox(&mut self.all_in_one_mesh, "Enable all the meshes to be in one file");
                if r.changed() {
                    change_options("all_in_one_mesh", &self.all_in_one_mesh.to_string());
                }
                if self.all_in_one_mesh == false{
                    let r = columns[1].checkbox(&mut self.detailed_export_name, "Enable a more detailed export name");
                    if r.changed() {
                        change_options("detailed_export_name", &self.detailed_export_name.to_string());
                    }
                }
                let r = columns[1].checkbox(&mut self.export_invisible, "Export invisible objects");
                if r.changed() {
                    change_options("export_invisible", &self.export_invisible.to_string());
                }
                let r = columns[1].checkbox(&mut self.transparency, "Enable transparency");
                if r.changed() {
                    change_options("transparency", &self.transparency.to_string());
                }
                let r = columns[1].checkbox(&mut self.emission, "Enable the creation of an emission map");
                if r.changed() {
                    change_options("emission", &self.emission.to_string());
                }
                if self.emission{
	                let r = columns[1].checkbox(&mut self.realistic_lightning, "Enable realistic emissions");
	                if r.changed() {
	                    change_options("realistic_lightning", &self.realistic_lightning.to_string());
	                }
                }
                let r = columns[1].checkbox(&mut self.roughness, "Enable roughness to be in red channel of extra texture map");
                if r.changed() {
                    change_options("roughness", &self.roughness.to_string());
                }
                let r = columns[1].checkbox(&mut self.metallic, "Enable metal to be in green channel of extra texture map");
                if r.changed() {
                    change_options("metallic", &self.metallic.to_string());
                }
                let r = columns[1].checkbox(&mut self.refraction, "Enable index of refraction to be in blue channel of extra texture map");
                if r.changed() {
                    change_options("refraction", &self.refraction.to_string());
                }
                let r = columns[1].checkbox(&mut self.specular, "Enable specular to be in alpha channel of extra texture map");
                if r.changed() {
                    change_options("specular", &self.specular.to_string());
                }
                columns[1].separator();
            });

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

}
impl Default for MyApp{
    fn default() -> Self {
            let (sx, rx): (Sender<String>, Receiver<String>) = channel();
            let c = read("src/options.txt").unwrap();
            //Initialize default vars
            let mut monochrome = true;
            let mut pattern_matching = true;
            let mut is_texturesize_powerof2 = true;
            let mut texturemapping_invisiblefaces = true;
            let mut manual_vt = false;
            let mut vt_precisionnumber = 0;
            //let background_color = [0.0f32, 0.0, 0.0];
            let mut debug_uv_mode = false;
            let mut cross = true;

            let mut custom_export_size = false;
            let mut sizex: f32 = 1.0;
            let mut sizey: f32 = 1.0;
            let mut sizez: f32 = 1.0;
            let mut uv_extra_precision = false;
            // Extra precision for UV's
            let mut cull_optimization = true;
            let mut y_is_up = true;
            let mut right_handed = true;
            let mut center_model_in_mesh = true;
            let mut normals = true;
            // .vox settings
            let mut all_in_one_mesh = true;
            let mut detailed_export_name = true;
            let mut transparency = true;
            let mut roughness = true;
            let mut emission = true;
            let mut realistic_lightning = false;
            let mut metallic = true;
            let mut refraction = true;
            let mut specular = true;
            let mut glass_creates_more_mesh = true;
            let mut export_invisible = true;



            let mut i = 0;
            for line_result in c.lines(){
                i += 1;
                let line = line_result.expect("Failed Reading the file src/options.txt");
                if line.trim().is_empty(){
                    continue;
                }
                let colon_index = line.find(':');
                if colon_index.is_some(){
                    let parts = line.split_at(colon_index.unwrap());
                    match parts.0 {
                        "monochrome" => {monochrome = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "pattern_matching" => {pattern_matching = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "is_texturesize_powerof2" => {is_texturesize_powerof2 = parts.1[1..].parse::<bool>().expect("Type is not correct");}
                        "texturemapping_invisiblefaces" => {texturemapping_invisiblefaces = parts.1[1..].parse::<bool>().expect("Type is not correct");}
                        "manual_vt" => {manual_vt = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "vt_precisionnumber" => {vt_precisionnumber = parts.1[1..].parse::<u8>().expect("Type is not correct: {}");}
                        "debug_uv_mode" => {debug_uv_mode = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "cross" => {cross = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "cull_optimization" => {cull_optimization = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "y_is_up" => {y_is_up = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "all_in_one_mesh" => {all_in_one_mesh = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "center_model_in_mesh" => {center_model_in_mesh = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "transparency" => {transparency = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "emission" => {emission = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "metallic" => {metallic = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "roughness" => {roughness = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "refraction" => {refraction = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "specular" => {specular = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "normals" => {normals = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "glass_creates_more_mesh" => {glass_creates_more_mesh = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "right_handed" => {right_handed = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "custom_export_size" => {custom_export_size = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "sizex" => {sizex = parts.1[1..].parse::<f32>().expect("Type is not correct: {}");}
                        "sizey" => {sizey = parts.1[1..].parse::<f32>().expect("Type is not correct: {}");}
                        "sizez" => {sizez = parts.1[1..].parse::<f32>().expect("Type is not correct: {}");}
                        "uv_extra_precision" => {uv_extra_precision = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "detailed_export_name" => {detailed_export_name = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "export_invisible" => {export_invisible = parts.1[1..].parse::<bool>().expect("Type is not correct: {}");}
                        "realistic_lightning" => {realistic_lightning = parts.1[1..].parse::<bool>().expect("Type is not correct")}
                        _ => {
                            println!("Line has an unrecognized type \n line:{}",line);
                        }
                    }
                }else{
                    println!("Line {} is not of type XXX:Value \n line: {}", i, line);
                }

            }


        Self{
            sx,
            rx: Arc::new(Mutex::new(rx)),
            dropped_files: vec![],
            picked_path: None,
            picked_file: None,
            status: "".to_string(),
            requestrepaint: false,
            monochrome,
            pattern_matching,
            is_texturesize_powerof2,
            texturemapping_invisiblefaces,
            manual_vt,
            vt_precisionnumber,
            background_color: [0.0,0.0,0.0],
            debug_uv_mode,
            cross,
            cull_optimization,
            y_is_up,
            center_model_in_mesh,
            all_in_one_mesh,
            transparency,
            emission,
            roughness,
            metallic,
            refraction,
            specular,
            normals,
            glass_creates_more_mesh,
            right_handed,
            custom_export_size,
            sizex,
            sizey,
            sizez,
            uv_extra_precision,
            detailed_export_name,
            export_invisible,
            realistic_lightning,
        }
    }
}

use std::io::Write;

fn change_options(option: &str, value: &str){
    let filename = "src/options.txt";
    let c = read(filename).unwrap();
    let mut file = std::fs::File::create(format!("src/options.txt")).unwrap();
    let mut found = false;
    for line_result in c.lines(){
        let mut line = line_result.expect("Failed Reading the file src/options.txt");
        if line.trim().is_empty(){
            continue;}
        let colon_index = line.find(':');
        if colon_index.is_some(){
            let parts = line.split_at(colon_index.unwrap());
            if parts.0 == option {
                line = format!("{}:{}", option, value);
                found = true;
            }
        }
        let result = writeln!(&mut file,"{}",line);
        if result.is_err(){panic!("Failed to write to save file")}
    }
    if found == false{
        let result = writeln!(&mut file, "{}:{}", option, value);
        if result.is_err(){panic!("Failed to write to save file")}

    }
}
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
