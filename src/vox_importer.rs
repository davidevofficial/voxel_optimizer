use std::error;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
//Ply reader without using external libraries
struct v{
    x: i32,
    y: i32,
    z: i32,
    r: u8,
    g: u8,
    b: u8
}
struct f{
    vs: Vec<v>,
}
struct ply{
    //metadata
    ply_format: String,
    exported_by: String,
    number_of_v_and_f: (i32, i32),
    //vertices and faces
    vertices: Vec<v>,
    faces: Vec<f>
}
//Reads the ply files and returns the content as a string
//
//
fn is_valid_ply(ply_path: std::path::PathBuf) -> bool{
    if ply_path.extension().unwrap() == std::ffi::OsStr::new("ply"){true}else{false}
}
fn read_ply(filepath: String) -> Result<String, io::Error>{
    let mut output = String::new();
    File::open(filepath)?.read_to_string(&mut output)?;
    Ok(output)
}
//Parses the ply file and returns a list of vertices and faces as a list
//
//
fn parse_ply(content: &String) -> Option<ply>{
    todo!();
}