use crate::vox_importer::*;
use crate::texture_mapping::*;
use crate::uv_unwrapping::*;
use crate::{MyApp, vox_importer};

//pub(crate) fn convert(my_app: &mut MyApp, path: &std::path::PathBuf, monochrome: &bool, pattern_matching: &bool, is_texturesize_powerof2: &bool, texturemapping_invisiblefaces: &bool, manual_vt: &bool, vt_precisionnumber: &u8, background_color: [f32;3], debug_uv_mode: bool){
pub(crate) fn convert(my_app: &mut MyApp, path: &std::path::PathBuf){
    my_app.status=format!("{}{}",String::from("converting:"), path.to_string_lossy().to_string());
    my_app.status = String::from("Reading...");
    let content = read_ply(&path.to_string_lossy().to_string());
    let ply:Result<ply, vox_importer::vox_importer_errors> = match content {
        Ok(content) => {
            println!("{}", content);
            my_app.status =format!("{}{}" ,String::from("parsing:"), path.to_string_lossy().to_string());
            parse_ply(&content)
            //my_app.status = "parsing" ; parse(content)
        },
        Err(error) => {
            println!("couldn't read!");
            my_app.status = String::from(format!("Error while Reading!!! {}",error.to_string()));
            return;
        }

    };
    if let Ok(ply) = &ply{
        println!("{:?}", &ply);
    }
    if let Err(e) = &ply{
        my_app.status = String::from(format!("Error while parsing!!! {}" ,e));
        println!("{}", e);
    }
    //let ply: ply = parse_ply(&content);
    //check if made by ephtracy if true continue else return with my_app.status = "error: ply not exported by magicavoxel"

}