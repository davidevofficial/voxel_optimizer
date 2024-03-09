use std::{error, fmt};
use std::error::Error;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, Read};
use crate::vox_importer::vox_importer_errors::{NotAscii, NotPly};
use crate::vox_exporter::Rgb;
//Ply reader without using external libraries

#[derive(Debug)]
pub enum vox_importer_errors{
    NotPly,
    NotAscii,
    NotEphtracy,
    NotVox,
    NotVersion200,
    Other(String),
}
impl std::fmt::Display for vox_importer_errors{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self{
            vox_importer_errors::NotPly => write!{f,"Not ply"},
            vox_importer_errors::NotAscii => write!{f,"Not ascii"},
            vox_importer_errors::NotEphtracy => write!{f,"Not Ephtracy"},
            vox_importer_errors::NotVox => write!{f,"Not Vox"},
            vox_importer_errors::NotVersion200 => write!{f,"Not Version 200"},
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
            self.vertices[va].x = (self.vertices[va].x*10.0).round();
            self.vertices[va].y = (self.vertices[va].y*10.0).round();
            self.vertices[va].z = (self.vertices[va].z*10.0).round();
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

#[derive(Debug, Default)]
pub struct Vox{
    //metadata
    number_of_models: usize,
    vox_version: usize,
    //cubes
    pub chunks:Vec<Chunks>,
    pub colours: Vec<Rgb>,
    pub materials: Vec<Matl>,
    pub tree: VoxTree
}
#[derive(Debug, Default)]
pub struct VoxTree{
    //vector of nodes ordered by their ID like node[0] is the node with id 0, node[5] is the node with id 5
    nodes: Vec<Node>,
}
#[derive(Debug, Default)]
pub struct Trn{
    size_in_bytes: u16,
    node_id: u8,
    //_name, _hidden 
    attributes: Dict,
    child_node_id: u8,
    layer: u8,
    n_of_frames: u8,
    //_r, _t, _f
    properties: Dict,
}
impl Trn{
    pub fn from_bytes(bytes: Vec<&u8>)->Trn{
        let bytesize = *(bytes[0])as u16+(256**(bytes[1])as u16)as u16;
        let id = bytes[8];
        let attributes_n = bytes[12];
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        let childid = bytes[16+size_of_dict];
        let layer = bytes[20+size_of_dict];
        let n_of_frames = bytes[24 + size_of_dict];
        if n_of_frames > &1{
            panic!("More than one frame! No animations allowed");
        }
        let attributes_n = bytes[28 + size_of_dict];
        let mut size_of_dict2 = 0;
        let mut dict2 = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for _x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[28+size_of_dict+size_of_dict2];
                size_of_dict2 += v_string1.buffer_size as usize;
                for x in 32+size_of_dict+size_of_dict2..32+size_of_dict+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[28+size_of_dict];
                size_of_dict2 += v_string2.buffer_size as usize;
                for x in 32+size_of_dict+size_of_dict2..32+size_of_dict+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict2.key_values.push((v_string1,v_string2))
            }
        }
        Trn{
            size_in_bytes:bytesize,
            node_id:*id,
            attributes:dict,
            child_node_id:*childid,
            layer:*layer,
            n_of_frames:*n_of_frames,
            properties: dict2
        }
    }
}
#[derive(Debug, Default)]
pub struct Grp{
    size_in_bytes: u16,
    node_id: u8,
    //_name, _hidden 
    attributes: Dict,
    n_of_children: u8,
    childlren_node_id: Vec<u8>,
}
impl Grp{
    pub fn from_bytes(bytes: Vec<&u8>)->Grp{
        let bytesize = *(bytes[0])as u16+(256**(bytes[1])as u16)as u16;
        let id = bytes[8];
        let attributes_n = bytes[12];
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        let n_of_children = bytes[16+size_of_dict];
        let mut childid = Vec::new();
        for n in 0..*n_of_children{
            childid.push(*bytes[16+size_of_dict+4+4*n as usize]);
        }
        //let childid = bytes[16+size_of_dict];
        
        Grp{
            size_in_bytes:bytesize,
            node_id:*id,
            attributes:dict,
            n_of_children:*n_of_children,
            childlren_node_id: childid,
        }
    }
}
#[derive(Debug, Default)]
pub struct Shp{
    size_in_bytes: u16,
    node_id: u8,
    //_name, _hidden ?
    attributes: Dict,
    n_of_models: u8,
    model_ids: Vec<u8>,
}
impl Shp{
    pub fn from_bytes(bytes: Vec<&u8>)->Shp{
        let bytesize = *(bytes[0])as u16+(256**(bytes[1])as u16)as u16;
        let id = bytes[8];
        let attributes_n = bytes[12];
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        let n_of_models = bytes[16+size_of_dict];
        let mut modelsid = Vec::new();
        for n in 0..*n_of_models{
            modelsid.push(*bytes[16+size_of_dict+4+4*n as usize]);
        }
        //let childid = bytes[16+size_of_dict];
        
        Shp{
            size_in_bytes:bytesize,
            node_id:*id,
            attributes:dict,
            n_of_models:*n_of_models,
            model_ids: modelsid,
        }
    }
}
#[derive(Debug, Default)]
pub struct Dict{
    n_of_key_values: u8,
    key_values: Vec<(VoxString, VoxString)>,
}
#[derive(Debug, Default)]
pub struct VoxString{
    buffer_size: u8,
    content: Vec<u8>,
}
#[derive(Debug)]
pub enum Node{
    TRN(Trn),
    GRP(Grp),
    SHP(Shp),
}
impl Default for Node{
    fn default() -> Self{
        Node::TRN(Trn::default())
    }
}
#[derive(Debug, Default)]
pub struct Chunks{
    pub id: u32,
    pub position: (u32,u32,u32),
    pub size: (u8, u8, u8),
    pub xyzi: Vec<VoxCubes>,
}
#[derive(Debug, Default)]
pub struct VoxCubes{
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub i: u8,
}
impl VoxCubes{
    pub fn from(x:u8,y:u8,z:u8,i:u8)->VoxCubes{return VoxCubes{x:x,y:y,z:z,i:i};}
}
#[derive(Debug, Default)]
pub struct Matl{
    //albedo
    pub rgb: Rgb,
    pub transparent: f32,
    //roughness map
    pub roughness: f32,
    //refraction map
    pub ior: f32,
    //metallic map
    pub specular: f32,
    pub metallic: f32,
    //emission map
    pub rgb_e: Rgb,
    /*
    pub emission: f32,
    pub ldr: f32,
    pub flux: u8,
    //colour on the emission map = (((rgb as (f32, f32, f32)* emission * ldr )/ 5.0) * (flux+1)) as u8 
    */
}
//Reads the ply files and returns the content as a string
//
//
pub(crate) fn is_valid_ply(ply_path: &std::path::PathBuf) -> bool{
    if ply_path.extension().unwrap() == std::ffi::OsStr::new("ply"){true}else{false}
}
pub(crate) fn is_vox(vox_path: &std::path::PathBuf) -> bool{
    if vox_path.extension().unwrap() == std::ffi::OsStr::new("vox"){true}else{false}
}
//pub fn is_valid_vox()
pub fn read_file(filepath: &String) -> Result<String, io::Error>{
    let mut output = String::new();
    File::open(filepath)?.read_to_string(&mut output)?;
    Ok(output)
}
//Parses the ply file and returns a list of vertices and faces as a list
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
    for v in 0..ply.number_of_v_and_f.0 {
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        //println!("{:?}", &tokens);
        //println!("{:?}", &v);
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
    for f in 0..ply.number_of_v_and_f.1 {
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        //println!("{:?}", &tokens);
        //println!("{:?}", &f);
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
pub fn parse_vox(content: &String) -> Result<Vox, vox_importer_errors>{
    let mut vox: Vox = Vox::default();
    let vox_bytes = content.as_bytes();
    //vox check
    let result: Result<&[u8; 4], _> = vox_bytes[0..4].try_into();
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != b"VOX "{
                return Err(vox_importer_errors::NotVox);
                }
            }
            Err(_) => println!("Failed!"),
        }
    let result: Result<u8, _> = vox_bytes[4].try_into();
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != 200{
                return Err(vox_importer_errors::NotVersion200);
                }
            }
            Err(_) => println!("Failed!"),
        }
    vox.vox_version = 200;
    //while find_x_in_y(x)
    //__________________S
    if vox_bytes[20] != 53{
        return Err(vox_importer_errors::Other("No models in the .vox file".to_string()));
    }
    let mut size_index = 20;
    while vox_bytes[size_index] == 53{
        let mut chunk = Chunks::default();
        chunk.size.0 = vox_bytes[size_index+12];
        chunk.size.1 = vox_bytes[size_index+16];
        chunk.size.2 = vox_bytes[size_index+20];
        let product = chunk.size.0*chunk.size.1*chunk.size.2;
        for voxel in 0..product{
        //________________Size->z, 3by -> 4bytes(XYZI) -> 12 Size -> +n (.n)->voxel
            let x = vox_bytes[size_index + 20 +3+ 4 + 12 + 1+4*voxel as usize];
            let y = vox_bytes[size_index + 20 +3+ 4 + 12 + 2+4*voxel as usize];
            let z = vox_bytes[size_index + 20 +3+ 4 + 12 + 3+4*voxel as usize];
            let i = vox_bytes[size_index + 20 +3+ 4 + 12 + 4+4*voxel as usize];
            chunk.xyzi.push(VoxCubes::from(x,y,z,i));
        }
        size_index = size_index + 20 +3+ 4 + 12 + 4+4*(product as usize-1)+1;
        vox.chunks.push(chunk)
    }
    vox.number_of_models = vox.chunks.len();
    let mut buf = 0;
    let mut nodes = Vec::new();
    while vox_bytes[size_index+1]==110{
        buf = vox_bytes[size_index+5] as usize+256*vox_bytes[size_index+6] as usize;
        let mut b = Vec::new();
        for x in 0..buf+9{
            b.push(&vox_bytes[size_index+5+x as usize]);
        }
        match vox_bytes[size_index+2] {
            //S hape
            53 => nodes.push(Node::SHP(Shp::from_bytes(b.clone()))),
            //G roup
            47 => nodes.push(Node::GRP(Grp::from_bytes(b.clone()))),
            //T ransform
            54 =>nodes.push(Node::TRN(Trn::from_bytes(b.clone()))),
            _ => return Err(vox_importer_errors::Other(".vox file nXXX is invalid".to_string())),
        }
        size_index+=b.len();
    }
    //RGBA find_RGBA_in_(allthefile)
    let rgba_index = find_x_in_y(&[52,47,42,41], &vox_bytes);
    if rgba_index.is_none(){return Err(vox_importer_errors::Other(".vox file is corrupted (NO RGBA TAG)".to_string()))}
    let mut palette = Vec::new();
    palette.push(Rgb{r:0, g:0, b:0});
    for x in 0..256{
        let r = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize];
        let g = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize+1];
        let b = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize+2];
        palette.push(Rgb{r:r,g:g,b:b});
    }
    //MATL just after RGBA so it should be easy
    //do it only if at least one of the vox setting is enabled
    //let MATL index = rgba_index + 1036
    //for x in 0..257{}
    //find _type in the next 30bytes, find _rough in the same 30 bytes, ecc...

    todo!()
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
fn find_next_x(bytes: &[u8], x: &[u8]) -> Option<usize>{bytes.windows(x.len()).position(|window| window == x)}
fn find_next_space_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b' ')}
fn find_next_newline_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b'\n')}
pub fn is_made_by_ephtracy(ply: ply) -> bool { if ply.exported_by == "comment : MagicaVoxel @ Ephtracy"{true} else {false}}