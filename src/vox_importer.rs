use std::{error, fmt};
use std::error::Error;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, Read};
use crate::vox_importer::vox_importer_errors::{NotAscii, NotPly};
//Ply reader without using external libraries

#[derive(Debug)]
pub enum vox_importer_errors{
    NotPly,
    NotAscii,
    NotEphtracy,
    Other(String),
}
impl std::fmt::Display for vox_importer_errors{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self{
            vox_importer_errors::NotPly => write!{f,"Not ply"},
            vox_importer_errors::NotAscii => write!{f,"Not ascii"},
            vox_importer_errors::NotEphtracy => write!{f,"Not Ephtracy"},
            vox_importer_errors::Other(ref s) => write!{f,"Other error:{}",s},
        }
    }
}
impl std::error::Error for vox_importer_errors{}
#[derive(Debug, Default)]
pub struct v{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ply{
    pub(crate) fn normalize_positions(mut self) -> Self{
        for va in 0..self.vertices.len(){
            self.vertices[va].x *= 10.0;
            self.vertices[va].y *= 10.0;
            self.vertices[va].z *= 10.0;
        }
    self
    }
}

#[derive(Debug, Default)]
pub struct f{
    pub(crate) vs: (i32, i32, i32, i32)
}

#[derive(Debug, Default)]
pub struct ply{
    //metadata
    ply_format: String,
    exported_by: String,
    pub number_of_v_and_f: (i32, i32),
    //vertices and faces
    pub vertices: Vec<v>,
    pub faces: Vec<f>
}
//Reads the ply files and returns the content as a string
//
//
pub(crate) fn is_valid_ply(ply_path: &std::path::PathBuf) -> bool{
    if ply_path.extension().unwrap() == std::ffi::OsStr::new("ply"){true}else{false}
}
pub fn read_ply(filepath: &String) -> Result<String, io::Error>{
    let mut output = String::new();
    File::open(filepath)?.read_to_string(&mut output)?;
    Ok(output)
}
//Parses the ply file and returns a list of vertices and faces as a list
//
//
pub fn parse_ply(content: &String) -> Result<ply, vox_importer_errors>{

    let mut ply: ply = ply::default();
    let ply_bytes = content.as_bytes();

    //ply check
    let result: Result<&[u8; 3], _> = ply_bytes[0..3].try_into();
    //println!("{:?}",result);
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != b"ply"{
                return Err(vox_importer_errors::NotPly);
                }
            }
            Err(_) => println!("Failed!"),
        }

    //ascii check
    let result: Result<&[u8; 16], _> = ply_bytes[5..0x15].try_into();
    match result{
        Ok(b) =>{
            if b != b"format ascii 1.0"{
                return Err(vox_importer_errors::NotAscii);
            } else { ply.ply_format = String::from("ascii 1.0") }
        }
        Err(_) => println!("Invalid!"),
    }
    //magicavoxel check
    let result: Result<&[u8; 32], _> = ply_bytes[0x17..0x37].try_into();
    match result{
        Ok(b) => {
            if b != b"comment : MagicaVoxel @ Ephtracy"{
                return Err(vox_importer_errors::NotEphtracy);
            } else { ply.exported_by = String::from("comment : Magicavoxel @ Ephtracy") }
        }
        Err(_) => println!("Error not made by Ephtracy's software"),
    }

    let nv_index = find_x_in_y(b"element vertex ", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap();
    let nv_newline_index = find_next_newline_after_index(&ply_bytes[nv_index..]).unwrap() +nv_index-1;
            let nf_index = find_x_in_y(b"element face ", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap();
    let nf_newline_index = find_next_newline_after_index(&ply_bytes[nf_index..]).unwrap() + nf_index-1;

    ply.number_of_v_and_f = (bytes_to_numeric::<i32>(&ply_bytes[(nv_index + 15)..nv_newline_index]).unwrap(),
                             bytes_to_numeric::<i32>(&ply_bytes[(nf_index + 13)..nf_newline_index]).unwrap());


    let mut start_index: usize = find_x_in_y(b"end_header", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap() + 12;
    let mut end_index: usize = 0;
    let mut vec_v: Vec<v> = Vec::new();
    for v in (0..ply.number_of_v_and_f.0){
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        println!("{:?}", &tokens);
        println!("{:?}", &v);
        start_index = end_index + 1;
        vec_v.push(v::default());
        //x, y, z value
        vec_v[v as usize].x = (bytes_to_numeric::<f32>(&tokens[0])).unwrap();
        vec_v[v as usize].y = (bytes_to_numeric::<f32>(&tokens[1])).unwrap();
        vec_v[v as usize].z = (bytes_to_numeric::<f32>(&tokens[2])).unwrap();
        //r, g, b value
        vec_v[v as usize].r = (bytes_to_numeric::<u8>(&tokens[3])).unwrap();
        vec_v[v as usize].g = (bytes_to_numeric::<u8>(&tokens[4])).unwrap();
        vec_v[v as usize].b = (bytes_to_numeric::<u8>(&tokens[5])).unwrap();
    }
    let mut vec_f: Vec<f> = Vec::new();
    for f in (0..ply.number_of_v_and_f.1){
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        println!("{:?}", &tokens);
        println!("{:?}", &f);
        start_index = end_index + 1;
        vec_f.push(f::default());
        //a, b, c, d indices
        vec_f[f as usize].vs.0 = (bytes_to_numeric::<i32>(&tokens[1])).unwrap();
        vec_f[f as usize].vs.1 = (bytes_to_numeric::<i32>(&tokens[2])).unwrap();
        vec_f[f as usize].vs.2 = (bytes_to_numeric::<i32>(&tokens[3])).unwrap();
        vec_f[f as usize].vs.3 = (bytes_to_numeric::<i32>(&tokens[4])).unwrap();


    }
    ply.faces = vec_f;
    ply.vertices = vec_v;
    Ok(ply)
}
fn bytes_to_numeric<T>(bytes: &[u8]) -> Option<T> where T:std::str::FromStr{
    if let Ok(str_value) = std::str::from_utf8(bytes){
        if let Ok(numeric_value) = str_value.parse::<T>(){
            return Some(numeric_value)
        }
    }
    None
}
fn find_x_in_y(x: &[u8], y: &[u8]) -> Option<usize> {
    for (index, window) in y.windows(x.len()).enumerate(){
        if window == x{
            return Some(index);
        }
    }
    None
}
fn split_into_words(input: &[u8]) -> Vec<&[u8]>{
    input.split(|&x| x==b' ').collect()
}
fn find_next_space_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b' ')}
fn find_next_newline_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b'\n')}
pub fn is_made_by_ephtracy(ply: ply) -> bool { if ply.exported_by == "comment : MagicaVoxel @ Ephtracy"{true} else {false}}