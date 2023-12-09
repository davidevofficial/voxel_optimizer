
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::io::BufWriter;
use std::path::Path;
use crate::greedy_mesher::OptimizedCube;
use crate::MyApp;
use std::collections::HashMap;
use png;
use crunch::*;
#[derive(Debug)]
pub struct Obj{
    //meta-info
    pub name: String,
    pub export_folder: String,
    pub number_of_v_and_f: (i32, i32),
    //--number of v, vt and f
    pub faces: Vec<ObjF>,
    pub vertices: Vec<ObjV>,
    pub vertices_uvs: Vec<ObjVt>,
    pub texture_map: TextureMap,
    pub vt_precisionnumber: u8,
}
#[derive(Debug)]
pub struct TextureMap{
    pub w: usize,
    pub h: usize,
    colours: Vec<Rgb>, //I might consider using a tuple(u8, u8, u8) instead of a struct,
                        //however in this way I could implement an equality comparator for rgb struct
}
#[derive(PartialEq)]
pub enum equality{
    NO,
    ONE,
    TWO_90,
    TWO_180,
    TWO_270,
    THREE_X,
    THREE_Y,
}
impl TextureMap{
    fn is_equal(&self, t2: TextureMap, typeofequality: i32) -> equality{

        if typeofequality == 0{
            return equality::NO;
        } 
        let mut equality_one = equality::ONE;
        let mut equality_two180 = equality::TWO_180;
        let mut equality_two90 = equality::TWO_90;
        let mut equality_two270 = equality::TWO_270;
        let mut eq_threex = equality::THREE_X;
        let mut eq_threey = equality::THREE_Y;

        if typeofequality >= 1{
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len()-1{
                        if equality_one == equality::ONE && self.colours[x] != t2.colours[x]{
                            equality_one = equality::NO;
                        }
                    }
            }

        }
        if equality_one == equality::ONE{
            return equality_one;
        } 
        if typeofequality >= 2 {
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len()-1{
                            if equality_two180==equality::TWO_180 && self.colours[self.colours.len()-1-x] != t2.colours[x]{ 
                                equality_two180 = equality::NO; //180 (-x, -y)  
                            }
                    }
            } else if self.w == t2.h && self.h == t2.w {
                    for x in 0..self.colours.len(){
                        let w = self.w;
                        let h = self.h;
                        let mut m = x%h;
                        let i = m*w+(w-(x-m)/h)-1;  
                        
                        if equality_two90==equality::TWO_90 && self.colours[i] != t2.colours[x]{ 
                                equality_two90 = equality::NO; //90 (y, -x)  
                            }

                        let i = (h-1-m)*w+(x-m)/h;  
                        if equality_two270==equality::TWO_270 && self.colours[i] != t2.colours[x]{
                            equality_two270 = equality::NO;
                        } //270 (-y, x)
                    }
            }
        } 
        if equality_two180 == equality::TWO_180 {
            return equality_two180;
        }
        if equality_two90 == equality::TWO_90 {
            return equality_two90;
        }
        if equality_two270 == equality::TWO_180{
            return equality_two270;
        }
        if typeofequality >= 3{
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len(){
                        let w = self.w;
                        let h = self.h;
                        let mut m = x%h;
                        let i = (w-1-m)+(x-m)/w;
                        if eq_threex==equality::THREE_X && self.colours[i] != t2.colours[x]{ 
                                eq_threex = equality::NO; //x (mirror x axis)
                            }
                        let i = m+((h-1)-(x-m)/w)*w;
                        if eq_threey==equality::THREE_Y && self.colours[i] != t2.colours[x]{ 
                                eq_threey = equality::NO; //y (mirror y axis)
                            }
                            
                        }
                    
                }
            }
        
        if eq_threex == equality::THREE_X {
            return eq_threex;
        }
        if eq_threey == equality::THREE_Y {
            return eq_threey;
        }
        return equality::NO

    }
}
#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub struct Rgb{
    r: u8,
    g: u8,
    b: u8
}

//todo()! -> implement an HashMap (obj_v, index_v) and an HashMap (obj_vt, index_vt)
#[derive(Debug)]
pub struct ObjF{
    // index_v|index_vt
    // (x,y,z),(u,v)
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32),
}
#[derive(Default)]
#[derive(Debug)]
pub struct ObjV{
    x: i32,
    y: i32,
    z: i32
}
impl ObjV{
    fn from_xyz(x:i32, y:i32, z:i32) -> ObjV{
        ObjV{
            x:x,
            y:y,
            z:z,
        }
    }
}
#[derive(Default)]
#[derive(Debug)]
pub struct ObjVt{
    u: f32,
    v: f32
}
fn add_two_tuples(a: (i32, i32, i32), b:(i32,i32,i32))->(i32,i32,i32){return (a.0+b.0, a.1+b.1, a.2+b.2)}
impl Obj{
    pub fn from_optimized_cubes(path: PathBuf,my_app: &MyApp, opcubes: &Vec<OptimizedCube>) -> Obj{
        println!("my_app.vt_precisionnumber:{:?}", my_app.vt_precisionnumber);
        let x =path.file_name().unwrap().to_str().unwrap().to_string().trim_end_matches(".ply").to_string();
        let y = x.replace(" ", "");
        let xx = my_app.picked_path.clone().unwrap().to_string();
        let yy = xx.replace("\\", "/");
        let mut obj = Obj {
            name: y,
            export_folder: yy,
            number_of_v_and_f: (0,0),
            faces: Vec::new(),
            vertices: Vec::new(),
            vertices_uvs: Vec::new(),
            texture_map: TextureMap{w:0, h:0, colours:Vec::new()},
            vt_precisionnumber: my_app.vt_precisionnumber,
        };

        let mut temp_v = HashMap::new();
        let mut n = 0;
        for i in 0..opcubes.len(){

            // 0x 0y 0z
            if let Some(_pat) = temp_v.get(&opcubes[i].starting_position) {
                n += 1;
            }else {
                temp_v.insert(opcubes[i].starting_position, i*8+1-n);
            } 
            
            // 1x 0y 0z
            let pos = add_two_tuples(opcubes[i].starting_position,(opcubes[i].dimensions.0 as i32,0,0));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+2-n);
            }
            // 1x 1y 0z
            let pos = add_two_tuples(opcubes[i].starting_position
                ,(opcubes[i].dimensions.0 as i32,opcubes[i].dimensions.1 as i32,0));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+3-n);
            }       
            // 0x 1y 0z
            let pos = add_two_tuples(opcubes[i].starting_position
                ,(0,opcubes[i].dimensions.1 as i32,0));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+4-n);
            }
            // 0x 0y 1z
           let pos = add_two_tuples(opcubes[i].starting_position
                ,(0,0,opcubes[i].dimensions.2 as i32));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+5-n);
            }
            // 1x 0y 1z
            let pos = add_two_tuples(opcubes[i].starting_position
                ,(opcubes[i].dimensions.0 as i32,0,opcubes[i].dimensions.2 as i32));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+6-n);
            }
            // 1x 1y 1z
            let pos = add_two_tuples(opcubes[i].starting_position
                ,(opcubes[i].dimensions.0 as i32,opcubes[i].dimensions.1 as i32,opcubes[i].dimensions.2 as i32));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+7-n);
            }
            
            // 0x 1y 1z
            let pos = add_two_tuples(opcubes[i].starting_position
                ,(0,opcubes[i].dimensions.1 as i32,opcubes[i].dimensions.2 as i32));
            if let Some(_pat) = temp_v.get(&pos) {
                n += 1;
            }else {
                temp_v.insert(pos, i*8+8-n);
            }  


        }

        //push the vertices into the list (there is nothing more we can do)
        //println!("{:?}", temp_v.len());
        //println!("{:?}", temp_v);

        obj.number_of_v_and_f.0 = temp_v.len() as i32;
        for _x in 0..obj.number_of_v_and_f.0{
            obj.vertices.push(ObjV::default());
        }
        for (k,v) in &temp_v{           
            obj.vertices[(*v as usize)-1] = ObjV::from_xyz(k.0, k.1, k.2);
        }
        

        for x in 0..opcubes.len(){
                let a = ObjV::from_xyz(opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                let b = ObjV::from_xyz(opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                let c = ObjV::from_xyz(opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                let d = ObjV::from_xyz(opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                let e = ObjV::from_xyz(opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let f = ObjV::from_xyz(opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let g = ObjV::from_xyz(opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let h = ObjV::from_xyz(opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);

                //face 1 top
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0),
                });
                //face 2 bottom
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0),
                });
                //face 3 left
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0),
                });
                //face 4 right
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0),
                });
                //face 5 front
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0),
                });
                //face 6 back
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0),
                    b:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0),
                    c:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0),
                    d:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0),
                });
        }
        obj.number_of_v_and_f.1 = obj.faces.len() as i32;
        let mut vec_tid: Vec<(i32, equality)> = Vec::new();
        //println!("{:?}",obj);

        if my_app.debug_uv_mode{
            
            obj.vertices_uvs.push(ObjVt{u:0.0, v:0.0});
            obj.vertices_uvs.push(ObjVt{u:0.0, v:2.0});
            obj.vertices_uvs.push(ObjVt{u:2.0, v:2.0});
            obj.vertices_uvs.push(ObjVt{u:2.0, v:0.0});
            obj.texture_map = TextureMap{w:2, h:2,colours:[Rgb{r:255,g:0,b:255},Rgb{r:0,g:0,b:0},Rgb{r:0,g:0,b:0},Rgb{r:255,g:0,b:255}].to_vec()};
            for x in 0..obj.faces.len(){
                obj.faces[x].a.1=1;
                obj.faces[x].b.1=2;
                obj.faces[x].c.1=3;
                obj.faces[x].d.1=4;
            }
            //mtl will be the same but png is going to be a 2x2 of pink and black and there are going to be 4 vt's in the whole obj
            return obj;
        } 
        //set up metadata, vertices, texture vertices, faces, textures
        todo!() 

    }
    fn write_mtl(&self){
        let x = format!("#@DL2023 - w:{:?}, h:{:?}\nnewmtl x\nmap_Kd {}.png", self.texture_map.w, self.texture_map.h, self.name);
        //todo!() -> write this to file
        //println!("export/{}.mtl",self.name);
        let mut mtl_file = File::create(format!("{}/{}.mtl",self.export_folder,self.name)).expect("creation failed");
                mtl_file.write(x.as_bytes()).expect("write failed");
    }
    fn write_obj(&mut self){
        let w = "#created with MagicaVoxel and VoxelOptimizer ";
        let nv = format!("v:{} - f:{}\n", self.number_of_v_and_f.0, self.number_of_v_and_f.1);
        let watermark = format!("{} - {}", w, nv);
        let name = &self.name;
        let oname = format!("o {}\n", name);
        let mtllibname = format!("mtllib {}.mtl\n", name);
        let USEMTL = format!("usemtl x\n");
        let mut obj_file = File::create(format!("{}/{}.obj",self.export_folder,self.name)).unwrap();

        obj_file.write(watermark.as_bytes()).expect("write failed");
        obj_file.write(oname.as_bytes()).expect("write failed");
        obj_file.write(mtllibname.as_bytes()).expect("write failed");
        //write vertices
        let mut list_of_v = String::new();
        for v in 0..self.number_of_v_and_f.0{
            list_of_v = format!("{}v {:?} {:?} {:?}\n",
                list_of_v,self.vertices[v as usize].x,self.vertices[v as usize].y,self.vertices[v as usize].z);
        }
        obj_file.write(list_of_v.as_bytes()).expect("write failed");
        //write vt
        let mut list_of_vt = String::new();
        if self.vt_precisionnumber == 0 && self.texture_map.w!=2 && self.texture_map.h!=2{
            if self.texture_map.w < 10{
                self.vt_precisionnumber = 2;
            } else if self.texture_map.w < 100{
                self.vt_precisionnumber = 4;
            } else if self.texture_map.w < 1000{
                self.vt_precisionnumber = 5;
            } else if self.texture_map.w < 10000{
                self.vt_precisionnumber = 6;
            }else if self.texture_map.w < 100000{
                self.vt_precisionnumber = 7;
            }
        }
        println!("writing vt's with: {:?} digits", self.vt_precisionnumber);
        for vt in 0..self.vertices_uvs.len(){
            let x = self.vt_precisionnumber as usize;
            let u = format!("{:.*}", x, (self.vertices_uvs[vt].u /self.texture_map.w as f32));
            let v = format!("{:.*}", x, (self.vertices_uvs[vt].v /self.texture_map.h as f32));
            list_of_vt = format!("{}vt {u} {v}\n",list_of_vt);
        }
        obj_file.write(list_of_vt.as_bytes()).expect("write failed");
        //write usemtl
        obj_file.write(USEMTL.as_bytes()).expect("write failed");
        //write faces
        let mut list_of_f = String::new();
        for f in 0..self.number_of_v_and_f.1{
            list_of_f = format!("{}f {}/{} {}/{} {}/{} {}/{}\n",
                list_of_f,self.faces[f as usize].a.0,self.faces[f as usize].a.1
                ,self.faces[f as usize].b.0,self.faces[f as usize].b.1
                ,self.faces[f as usize].c.0,self.faces[f as usize].c.1
                ,self.faces[f as usize].d.0,self.faces[f as usize].d.1);
        }
        obj_file.write(list_of_f.as_bytes()).expect("write failed");



    }
    fn write_png(&self){
        let file = File::create(format!("{}/{}.png",self.export_folder,self.name)).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.texture_map.w as u32, self.texture_map.h as u32); 
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut data = Vec::new(); // An array containing an RGB sequence
        for x in 0..self.texture_map.colours.len(){
            data.push(self.texture_map.colours[x].r);
            data.push(self.texture_map.colours[x].g);
            data.push(self.texture_map.colours[x].b);
        }
        writer.write_image_data(&data).unwrap();
    }
    pub fn export_all(&mut self){
        self.write_obj();
        self.write_mtl();
        self.write_png();
    }
}
