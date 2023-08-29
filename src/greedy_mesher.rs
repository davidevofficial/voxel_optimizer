use std::path::PathBuf;
use crate::vox_importer::*;
use crate::texture_mapping::*;
use crate::uv_unwrapping::*;
use crate::{MyApp, vox_importer};

/*
END_PRODUCT
pub struct Obj{
    //metainfo
    //--number of v, vt and f
    faces: vec!<obj_f>,
    vertices: vec!<obj_v>,
    vertices_uvs: vec!<obj_vt>
}
pub struct TextureMap{
    w: i32,
    h: i32,
    colours: [[rgb;w];h] //I might consider using a tuple(u8, u8, u8) instead of a struct,
                            however in this way I could implement an equality comparator for rgb struct
}
pub struct rgb{
    r: u8,
    g: u8,
    b: u8
}
pub struct obj_f{
    //index_v|index_vt
    a = (i32, i32),
    b = (i32, i32),
    c = (i32, i32),
    d = (i32, i32)
}
pub struct obj_v{
    x: i32
    y: i32
    z: i32
}
pub struct obj_vt{
    u: f32,
    v: f32
}
END________
INTERMEDIARY_PRODUCT
pub struct cube{
    faces: Option<[cube_f;6]>, //(it was about to be outdated even before I uncommented this mess LOL DEATH_EMOJI)
    position: (f32, f32, f32),
    merged: bool
}
pub struct cube_f{
    dir: DIRECTION,
    colour: (u8,u8,u8),
    vertices: [v;4]
}
pub struct cube_v{
    x: i32,
    y: i32,
    z: i32
}
pub struct cube_vt{
    u: f32,
    v: f32
}
impl cube_f{
    fn from_vertices(a: v, b: v, c: v, d: v) -> cube_f{
    }
}
pub struct OptimizedCube{
    //___________w_|_h_|_d_|_
    dimensions: (u8, u8, u8)

    //used to evalyate the texture map of each face
    cubes: Vec!<cube>
    important_vertices: Vec!<cube_v>
}

 */
//pub(crate) fn convert(my_app: &mut MyApp, path: &std::path::PathBuf, monochrome: &bool, pattern_matching: &bool, is_texturesize_powerof2: &bool, texturemapping_invisiblefaces: &bool, manual_vt: &bool, vt_precisionnumber: &u8, background_color: [f32;3], debug_uv_mode: bool){
pub(crate) fn convert(my_app: &mut MyApp, path: PathBuf){
    let x= format!("{}{}",String::from("converting:"), path.to_string_lossy().to_string());
    my_app.sx.send(x);
    my_app.status = String::from("Reading...");
    let content = read_ply(&path.to_string_lossy().to_string());
    let ply:Result<ply, vox_importer::vox_importer_errors> = match content {
        Ok(content) => {
            println!("{}", content);
            let x = format!("{}{}" ,String::from("parsing:"), path.to_string_lossy().to_string());
            my_app.sx.send(x);
            parse_ply(&content)
            //my_app.status = "parsing" ; parse(content)
        },
        Err(error) => {
            println!("couldn't read!");
            let x = String::from(format!("Error while Reading!!! {}",error.to_string()));
            my_app.sx.send(x);

            return;
        }

    };
    if let Ok(ply) = &ply{
        let x = String::from(format!("Optimizing model with {} vertices and {} faces", &ply.number_of_v_and_f.0, &ply.number_of_v_and_f.1));
        my_app.sx.send(x);
        println!("{:?}", &ply);
    }
    if let Err(e) = &ply{
        let x = String::from(format!("Error while parsing!!! {}" ,e));
        my_app.sx.send(x);
        println!("{}", e);
    }
    //let ply: ply = parse_ply(&content);
    //check if made by ephtracy if true continue else return with my_app.status = "error: ply not exported by magicavoxel"

}