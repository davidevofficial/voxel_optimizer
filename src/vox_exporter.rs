
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::io::BufWriter;
use std::path::Path;
use crate::greedy_mesher::{OptimizedCube};
use crate::{vox_importer, MyApp};
use std::collections::HashMap;

use crunch::*;
#[derive(Debug)]
pub struct Obj{
    //meta-info
    pub name: String,
    pub export_folder: String,
    pub number_of_v_and_f: (i32, i32),
    //--number of v, vt and f
    pub is_vox: bool,
    pub faces: Vec<ObjF>,
    pub vertices: Vec<ObjV>,
    pub vertices_uvs: Vec<ObjVt>,
    pub vertices_normals: Vec<ObjVn>,
    pub texture_map: TextureMap,
    pub material_map: Option<MaterialMap>,
    pub materials: Option<Vec<vox_importer::Matl>>,
    ///allowed materials in order are 0: Albedo, 1: Alpha, 2: Rgb_e, 3: Roughness, 4: Metal, 5:Spec, 6: Ior (total len = 7) 
    pub allowed_materials: (bool, bool, bool, bool, bool, bool, bool),
    pub vt_precisionnumber: u8,
    pub y_is_up: bool,
    pub center_model: bool,
    pub background_color: Rgb,
}
#[derive(Copy, Debug, PartialEq)]
#[derive(Clone)]
#[derive(Default)]
pub struct Rgb{
    pub r: u8,
    pub g: u8,
    pub b: u8
}
#[derive(Debug, Clone)]
pub struct MaterialMap{
    pub w: usize,
    pub h: usize,
    pub id: Vec<u8>,
    pub materials: Vec<vox_importer::Matl>,
}
impl MaterialMap{
    fn from(w:usize, h:usize, materials: Vec<vox_importer::Matl>){

    }
}
#[derive(Debug, Clone)]
pub struct TextureMap{
    pub w: usize,
    pub h: usize,
    pub colours: Vec<Option<Rgb>>, //I might consider using a tuple(u8, u8, u8) instead of a struct,
                        //however in this way I could implement an equality comparator for rgb struct
}
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Equality{
    No,
    One,
    Two90,
    Two180,
    Two270,
    ThreeX,
    ThreeY,
}
impl TextureMap{
    fn is_equal(&self, t2: &TextureMap, typeofequality: i32) -> Equality{

        if typeofequality == 0 || self.colours.len() != t2.colours.len(){
            return Equality::No;
        } 
        //println!("comparing {:?} with {:?}",self, t2);
        let mut equality_one = true;
        /*
        let mut equality_one = true;
        let mut equality_two180 = equality::TWO_180;
        let mut equality_two90 = equality::TWO_90;
        let mut equality_two270 = equality::TWO_270;
        let mut eq_threex = equality::THREE_X;
        let mut eq_threey = equality::THREE_Y;
        */
        if typeofequality >= 1 && self.w == t2.w && self.h == t2.h {
                for x in 0..self.colours.len(){
                    if equality_one == true && self.colours[x] != t2.colours[x]{
                        equality_one = false;
                    }
                }
        }
        if equality_one{
            //println!("{:?}", "it's a match! One");
            return Equality::One;
        } 
        if typeofequality >= 2 {
            if self.w == t2.h && self.h == t2.w{
                let t1 = self.rotate();
                if t1.is_equal(t2, 1) != Equality::No{return Equality::Two90}
            }
            if self.w == t2.w && self.h == t2.h{
                let t1 = self.rotate().rotate();
                if t1.is_equal(t2, 1) != Equality::No{return Equality::Two180}
            }
            if self.w == t2.h && self.h == t2.w{
                let t1 = self.rotate().rotate().rotate();
                if t1.is_equal(t2, 1) != Equality::No{return Equality::Two270}
            }
            /*
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len(){
                            if equality_two180==equality::TWO_180 && self.colours[self.colours.len()-1-x] != t2.colours[x]{ 
                                equality_two180 = equality::NO; //180 (-x, -y)  
                            }
                    }
            } else if self.w == t2.h && self.h == t2.w {
                    for x in 0..self.colours.len(){
                        let w = self.w;
                        let h = self.h;
                        let m = x%h;
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
            */
        } 
        /*
        if equality_two180 == equality::TWO_180 {
            println!("{:?}", "it's a match! Two180");
            return equality::TWO_180;
        }
        if equality_two90 == equality::TWO_90 {
            println!("{:?}", "it's a match! Two90");
            return equality::TWO_90;
        }
        if equality_two270 == equality::TWO_270{
            println!("{:?}", "it's a match! Two270");
            return equality::TWO_270;
        }
        */
        if typeofequality >= 3 && self.w == t2.w && self.h == t2.h{
            if self.flipx().is_equal(t2, 1) != Equality::No{return Equality::ThreeX}
            if self.flipy().is_equal(t2, 1) != Equality::No{return Equality::ThreeY}
        }
            
        /*
        if eq_threex == equality::THREE_X {
            println!("{:?}", "it's a match! Threex");
            return equality::THREE_X;

        }
        if eq_threey == equality::THREE_Y {
            println!("{:?}", "it's a match! Threey");
            return equality::THREE_Y;
        }
        return equality::NO
        */
        Equality::No //return

    }
    //_______________________________________x___y____
    fn scalar_to_coordinates(w:i32, i:i32)->(i32,i32){
        ((i%w),((i-(i%w))/w)) //return
    }
    //__________________________________x____y__________index______
    fn coordinates_to_scalar(w:i32, xy:(i32,i32))->i32{
        xy.0+(xy.1*w) //return
    }
    fn rotate(&self)->TextureMap{
        let mut buffer1 = Vec::new();
        let h = self.w;
        let w = self.h;
        for x in 0..self.colours.len(){
            let i = TextureMap::scalar_to_coordinates(self.w as i32, x as i32);
            let ii = TextureMap::coordinates_to_scalar(w as i32, ((w as i32 -1 - i.1), i.0));
            buffer1.push(self.colours[ii as usize]);
        }
        TextureMap{
            w,
            h,
            colours: buffer1,
        }
    }
    fn flipx(&self)->TextureMap{
        let w = self.w;
        let h = self.h;
        let mut buffer1 = Vec::new();
        for x in 0..self.colours.len(){
            let i = TextureMap::scalar_to_coordinates(self.w as i32, x as i32);
            let ii = TextureMap::coordinates_to_scalar(self.w as i32, (self.w as i32 - 1 - i.0, i.1));
            buffer1.push(self.colours[ii as usize]);
        }
        TextureMap{
            w,
            h,
            colours: buffer1,
        }
    }
    fn flipy(&self)->TextureMap{
        let w = self.w;
        let h = self.h;
        let mut buffer1 = Vec::new();
        for x in 0..self.colours.len(){
            let i = TextureMap::scalar_to_coordinates(self.w as i32, x as i32);
            let ii = TextureMap::coordinates_to_scalar(self.w as i32, (i.0, self.h as i32 - 1 - i.1));
            buffer1.push(self.colours[ii as usize]);
        }
        TextureMap{
            w,
            h,
            colours: buffer1,
        }
    }
    fn is_texture_some(&self)->bool{
        for pixel in 0..self.colours.len(){
            if self.colours[pixel].is_some(){
                return true;
            }
        }
        false //return
    }
}

impl Rgb{
    fn from(rgb:Option<(u8,u8,u8)>, ba:(u8,u8,u8)) -> Rgb{
        if let Some(x) = rgb{
            return Rgb{r:x.0, g: x.1, b: x.2}
        }
        Rgb{r:ba.0, g:ba.1, b:ba.2}
    }
}
//todo()! -> implement an HashMap (obj_v, index_v) and an HashMap (obj_vt, index_vt)
#[derive(Debug)]
pub struct ObjF{
    // index_v|index_vt
    // (x,y,z),(u,v)
    a: (i32, i32, i32),
    b: (i32, i32, i32),
    c: (i32, i32, i32),
    d: (i32, i32, i32),
}
///Vertices of a .obj file
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
            x,
            y,
            z,
        }
    }
}

#[derive(Default,Debug,Clone,Copy)]
pub struct ObjVt{
    u: i32,
    v: i32
}
#[derive(Debug,Default,Clone,Copy)]
pub struct ObjVn{
    nx: i32,
    ny: i32,
    nz: i32,
}
/// Adds two xyz tuples together.
///[Rust Book](https://doc.rust-lang.org/book/)
///See also [`ObjV`]
/// # Arguments
///
/// * `a: (i32,i32,i32)` - The first tuple.
/// * `b: (i32,i32,i32)` - The second tuple.
///
/// # Returns
///
/// * The sum of `a` and `b` as a tuple 'c'.
///
/// # Examples
///
/// ```
/// let result = add((2,2,2), (3,2,1));
/// assert_eq!(result, (5,4,3));
/// ```
fn add_two_tuples(a: (i32, i32, i32), b:(i32,i32,i32))->(i32,i32,i32){return (a.0+b.0, a.1+b.1, a.2+b.2)}
impl Obj{
    pub fn from_optimized_cubes(path: PathBuf,my_app: &MyApp, opcubes: &Vec<OptimizedCube>, is_vox:bool, materials: Option<Vec<vox_importer::Matl>>) -> Obj{
        //println!("my_app.vt_precisionnumber:{:?}", my_app.vt_precisionnumber);

        let x = if !is_vox{path.file_name().unwrap().to_str().unwrap().to_string().trim_end_matches(".ply").to_string()}
                        else{path.file_name().unwrap().to_str().unwrap().to_string().trim_end_matches(".vox").to_string()};
        let y = x.replace(' ', "");
        let xx = my_app.picked_path.clone().unwrap().to_string();
        let yy = xx.replace("\\", "/");
        let allowed_materials = {
            (true,
            my_app.transparency,
            my_app.emission,
            my_app.roughness,
            my_app.metal,
            my_app.refraction,
            my_app.specular)
        };
        let mut obj = Obj {
            name: y,
            export_folder: yy,
            number_of_v_and_f: (0,0),
            faces: Vec::new(),
            vertices: Vec::new(),
            vertices_uvs: Vec::new(),
            texture_map: TextureMap{w:0, h:0, colours:Vec::new()},
            vt_precisionnumber: my_app.vt_precisionnumber,
            y_is_up: my_app.y_is_up,
            center_model: my_app.center_model_in_mesh,
            background_color: Rgb{r:(my_app.background_color[0]*255.0) as u8
                                 ,g:(my_app.background_color[1]*255.0) as u8
                                 ,b:(my_app.background_color[2]*255.0) as u8},
            is_vox,
            material_map: None,
            materials,
            allowed_materials,
            vertices_normals: Vec::new(),

        };
        //If normals then write normals
        if my_app.normals && !my_app.y_is_up{
            //Top
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: 1 });
            //Bottom
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: -1 });
            //Forward
            obj.vertices_normals.push(ObjVn { nx: 0, ny: -1, nz: 0 });
            //Backwards
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 1, nz: 0 });
            //Right
            obj.vertices_normals.push(ObjVn { nx: 1, ny: 0, nz: 0 });
            //Left
            obj.vertices_normals.push(ObjVn { nx: -1, ny: 0, nz: 0 });
        }else if my_app.normals && my_app.y_is_up{
            //Top
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 1, nz: 0 });
            //Bottom
            obj.vertices_normals.push(ObjVn { nx: 0, ny: -1, nz: 0 });
            //Forward
            obj.vertices_normals.push(ObjVn { nx: -1, ny: 0, nz: 0 });
            //Backwards
            obj.vertices_normals.push(ObjVn { nx: 1, ny: 0, nz: 0 });
            //Right
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: 1 });
            //Left
            obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: -1 });
        }

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
        obj.number_of_v_and_f.0 = temp_v.len() as i32;
        for _x in 0..obj.number_of_v_and_f.0{
            obj.vertices.push(ObjV::default());
        }
        for (k,v) in &temp_v{           
            obj.vertices[*v-1] = ObjV::from_xyz(k.0, k.1, k.2);
        }
        //push the vertices into the list (there is nothing more we can do)
        //println!("{:?}", temp_v.len());
        //println!("{:?}", temp_v); 
        

        for x in 0..opcubes.len(){
                let a = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                let b = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                let c = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                let d = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                let e = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let f = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let g = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                let h = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);

                //face 1 top
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,1),
                    b:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,1),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,1),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,1),
                });
                //face 2 bottom
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,2),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,2),
                    c:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,2),
                    d:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,2),
                });
                //face 3 left
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,6),
                    b:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,6),
                    c:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,6),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,6),
                });
                //face 4 right
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,5),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,5),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,5),
                    d:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,5),
                });
                //face 5 front
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,3),
                    b:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,3),
                    c:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,3),
                    d:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,3),
                });
                //face 6 back
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,4),
                    b:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,4),
                    c:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,4),
                    d:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,4),
                });
        }
        obj.number_of_v_and_f.1 = obj.faces.len() as i32;
        let mut tid: Vec<(Option<i32>, Equality)> = Vec::new();
        let mut unique_tid: Vec<TextureMap> = Vec::new();
        let mut temp_vt: HashMap<(i32,i32),i32> = HashMap::new();
        let mut positions = Vec::new();
        //println!("{:?}",obj);

        if my_app.debug_uv_mode{
            
            obj.vertices_uvs.push(ObjVt{u:0, v:0});
            obj.vertices_uvs.push(ObjVt{u:0, v:2});
            obj.vertices_uvs.push(ObjVt{u:2, v:2});
            obj.vertices_uvs.push(ObjVt{u:2, v:0});
            obj.texture_map = TextureMap{w:2, h:2,colours:[Some(Rgb{r:255,g:0,b:255}),Some(Rgb{r:0,g:0,b:0}),Some(Rgb{r:0,g:0,b:0}),Some(Rgb{r:255,g:0,b:255})].to_vec()};
            for x in 0..obj.faces.len(){
                obj.faces[x].a.1=1;
                obj.faces[x].b.1=2;
                obj.faces[x].c.1=3;
                obj.faces[x].d.1=4;
            }
            //mtl will be the same but png is going to be a 2x2 of pink and black and there are going to be 4 vt's in the whole obj
            return obj;
        } 
        //set up textures
        for x in 0..opcubes.len(){
            for t in 0..6{
                //println!("opcubes[{:?}].textures[{:?}] = {:?}", x,t,opcubes[x].textures[t]);
                //what to do if texture is empty or all of the same colour?
                let is_texture_some = opcubes[x].textures[t].is_texture_some();
                let mut is_all_same_colour = true;
                //let mut pixels = 
                for pixel in 0..opcubes[x].textures[t].colours.len(){
                    //pixels.push(opcubes[x].textures[t].colours[pixel]); 
                    //check the existence of each pixel
                    if is_texture_some{
                        //if this setting is true
                        if my_app.monochrome{
                            // and if it is not the first pixel
                            if pixel > 0{
                                //if the next pixel is not equal to the last
                                let previousp = &opcubes[x].textures[t].colours[pixel - 1].clone();
                                let currentp = &opcubes[x].textures[t].colours[pixel].clone();
                                if previousp.is_some() && 
                                   currentp.is_some() &&
                                   previousp.unwrap() != currentp.unwrap() {
                                    //texture isn't all of the same colour
                                    is_all_same_colour = false;
                                }
                                
                            }
                        }
                    }
                }
                //println!("is texture some = {:?}", is_texture_some);
                //println!("is all same color = {:?}", is_all_same_colour);
                if !is_texture_some{
                    tid.push((None, Equality::No));
                } else if is_texture_some{
                    //the texture is going to depend on if it is a single colour or more
                    let mut tex = opcubes[x].textures[t].clone();
                    if is_all_same_colour{
                        let c = obj.background_color;
                        let mut i = 0;  
                        while opcubes[x].textures[t].colours[i].is_none(){
                            i+=1;
                        }      
                        tex = TextureMap{w:1,h:1, colours:[opcubes[x].textures[t].colours[i]].to_vec()}
                    }
                    //println!("{:?}", tex);

                    if unique_tid.is_empty(){
                        //if all the texture is of a colour just push that colour
                        unique_tid.push(tex);
                        //tid.push((Some(((x*6)+t) as i32), equality::NO));
                        tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))
                        
                    //if there is a unique texture already
                    }else {
                        //if it is just one colour check if that colour exists already
                        if my_app.pattern_matching == 0{
                            unique_tid.push(tex.clone());
                            tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))
                        }else{
                            let mut equ = Equality::No;
                            let mut ii = 0;
                            for i in 0..unique_tid.len(){

                                if equ == Equality::No{
                                    match tex.is_equal(&unique_tid[i], my_app.pattern_matching){
                                        Equality::No =>{}
                                        Equality::One =>{ii=i as i32;equ = Equality::One;}
                                        Equality::Two90 =>{ii=i as i32;equ = Equality::Two90;}
                                        Equality::Two180 =>{ii=i as i32;equ = Equality::Two180;}
                                        Equality::Two270 =>{ii=i as i32;equ = Equality::Two270;}
                                        Equality::ThreeX =>{ii=i as i32;equ = Equality::ThreeX;}
                                        Equality::ThreeY =>{ii=i as i32;equ = Equality::ThreeY;}
                                    }
                                }
                                //println!("opcubes[{:?}].textures[{:?}] equality::{:?} unique_tid[{:?}]",x,t,equ,i);
                            }
                            //println!("opcubes[{:?}].textures[{:?}] equality::{:?} with the rest of the textures",x,t,equ);
                            if equ == Equality::No{
                                unique_tid.push(tex.clone());
                                tid.push((Some((unique_tid.len()-1) as i32), Equality::No));
                            }
                            if equ != Equality::No {
                                //println!("pushing tid: {:?}", ii);
                                //println!("lenght before: {:?}", tid.len());
                                tid.push((Some(ii), equ));
                                //println!("lenght after: {:?}", tid.len());

                            }
                        }
                        
                    }
                }
            } 
        }
        //println!("unique tid .len()={:?}", unique_tid.len());
        //println!("tid .len()={:?}", tid.len());
        let mut items = Vec::new();
        for x in 0..unique_tid.len(){
            items.push(crunch::Item::new(x, unique_tid[x].w, unique_tid[x].h, crunch::Rotation::None));
            positions.push((0,0));
            
            //println!("tid[{:?}] = {:?}",x, tid[x]);
            //println!("{:?}", unique_tid[x].colours);
            //println!("{:?}x{:?}", unique_tid[x].w, unique_tid[x].h);

        }
        println!("{:?} unique textures", unique_tid.len());
        let mut container = crunch::Rect::of_size(1, 1);
        while pack(container, items.clone()).is_err(){
            container.w *= 2;
            container.h *= 2;
            if container.w > 100000{
                panic!();
            }
        }
        let packed = match pack(container, items) {
            Ok(all_packed) => {all_packed},
            Err(some_packed) => {println!("{:?}", "some packed");some_packed},
        };
        let mut finaltexture = TextureMap{w:container.w, h:container.h, colours:Vec::new()};
        for _x in 0..finaltexture.w*finaltexture.h{
            finaltexture.colours.push(Some(obj.background_color));
        }
        for item in &packed {
            positions[item.data] = (item.rect.x, item.rect.y);
            for p in 0..unique_tid[item.data].colours.len(){
                let pp = TextureMap::scalar_to_coordinates(unique_tid[item.data].w as i32,p as i32);
                let ppp = (pp.0+item.rect.x as i32, pp.1+item.rect.y as i32);
                let i =TextureMap::coordinates_to_scalar(container.w as i32, ppp);
                finaltexture.colours[i as usize] = unique_tid[item.data].colours[p];
            } 
            let a = (item.rect.x as i32, item.rect.y as i32);
            let b = (item.rect.x as i32, (item.rect.y+item.rect.h) as i32);
            let c = ((item.rect.x+item.rect.w) as i32, (item.rect.y+item.rect.h) as i32);
            let d = ((item.rect.x+item.rect.w) as i32, item.rect.y as i32);
            if let Some(_pat) = temp_vt.get(&a) {
            }else {
                let l = temp_vt.len() as i32;
                temp_vt.insert(a, l+1);
            } 
            if let Some(_pat) = temp_vt.get(&b) {
            }else {
                let l = temp_vt.len() as i32;
                temp_vt.insert(b, l+1);
            } 
            if let Some(_pat) = temp_vt.get(&c) {
            }else {
                let l = temp_vt.len() as i32;
                temp_vt.insert(c, l+1);
            } 
            if let Some(_pat) = temp_vt.get(&d) {
            }else {
                let l = temp_vt.len() as i32;
                temp_vt.insert(d, l+1);
            } 
            //println!("data: {:?}, x: {:?}, y: {:?}", item.data, item.rect.x, item.rect.y);
        }
        for x in 0..obj.faces.len(){
            let mut aa = 1;
            let mut bb = 1;
            let mut cc = 1;
            let mut dd = 1;
            if tid[x].0.is_some(){
                let a = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32,
                (unique_tid[tid[x].0.unwrap() as usize].h)as i32 + positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                let b = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32 + (unique_tid[tid[x].0.unwrap() as usize].w)as i32,
                (unique_tid[tid[x].0.unwrap() as usize].h)as i32 + positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                let c = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32 + (unique_tid[tid[x].0.unwrap() as usize].w)as i32,
                                     positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                let d = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32,
                                     positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();

                match tid[x].1 {
                    Equality::No => {
                        aa = *a;
                        bb = *b;
                        cc = *c;
                        dd = *d;
                    }Equality::One => {
                        aa = *a;
                        bb = *b;
                        cc = *c;
                        dd = *d;
                    }Equality::Two90=> {
                        aa = *b;
                        bb = *c;
                        cc = *d;
                        dd = *a;
                    }Equality::Two180=> {
                        aa = *c;
                        bb = *d;
                        cc = *a;
                        dd = *b;
                    }Equality::Two270=> {
                        dd = *c;
                        cc = *b;
                        bb = *a;
                        aa = *d;
                    }Equality::ThreeX=> {
                        dd = *c;
                        cc = *d;
                        bb = *a;
                        aa = *b;
                    }Equality::ThreeY=> {
                        dd = *a;
                        cc = *b;
                        bb = *c;
                        aa = *d;
                    
                    }
                }
            }
            obj.faces[x].a.1 = aa;
            obj.faces[x].b.1 = bb;
            obj.faces[x].c.1 = cc;
            obj.faces[x].d.1 = dd;
        }
        //VT LIST______
        for _x in 0..temp_vt.len(){
            obj.vertices_uvs.push(ObjVt::default());
        }
        for (k,v) in &temp_vt{           
            obj.vertices_uvs[(*v as usize)-1] = ObjVt{u: k.0, v: k.1};
        }
        obj.texture_map = finaltexture;
        //dbg!(&my_app.normals, &obj.vertices_normals);
        obj //return
    }
    ///[.obj and .mtl file specs][https://www.wikiwand.com/en/Wavefront_.obj_file#Physically-based_Rendering]
    ///
    ///
    fn write_mtl(&self){

        let transparency = if self.allowed_materials.1 &&self.is_vox{format!("\nTr 0.001\nmap_d -imfchain l {}.png",self.name)}
                                      else{"".to_owned()};
        let emission = if self.allowed_materials.2 &&self.is_vox{format!("\nmap_Ke {}_emit.png",self.name)
                                }else{"".to_owned()};
        let roughness = if self.allowed_materials.3 &&self.is_vox{format!("\nmap_Pr {}_extra.png -imfchain r",self.name)
                                }else{"".to_owned()};
        let metallic = if self.allowed_materials.4 && self.is_vox{format!("\nmap_Pm {}_extra.png -imfchain g",self.name)
                                }else{"".to_owned()};
        let specular = if self.allowed_materials.5 &&self.is_vox{format!("\nmap_Ns {}_extra.png -imfchain b",self.name)
                                }else{"".to_owned()};
        let ior = if self.allowed_materials.6 && self.is_vox{format!("\nmap_Ni {}_extra.png -imfchain l",self.name)
                                }else{"".to_owned()};
        let x = format!("#@DL2023 - w:{:?}, h:{:?}\nnewmtl x{}\nmap_Kd {}.png{}{}{}{}{}",
                                self.texture_map.w,
                                self.texture_map.h,
                                transparency,
                                self.name,
                                emission, roughness, metallic, specular, ior);
        //todo!() -> write this to file
        //println!("export/{}.mtl",self.name);
        let mut mtl_file = File::create(format!("{}/{}.mtl",self.export_folder,self.name)).expect("creation failed");
                mtl_file.write_all(x.as_bytes()).expect("write failed");
    }
    fn write_obj(&mut self, shape:(i32,i32,i32),lowest_coordinates:(i32,i32,i32)){

        //Meta data
        let w = "#created with MagicaVoxel and VoxelOptimizer ";
        let nv = format!("v:{} - f:{}\n", self.number_of_v_and_f.0, self.number_of_v_and_f.1);
        let watermark = format!("{} - {}", w, nv);
        let name = &self.name;
        let oname = format!("o {}\n", name);
        let mtllibname = format!("mtllib {}.mtl\n", name);
        let usemtl = "usemtl x\n".to_string();
        let mut obj_file = File::create(format!("{}/{}.obj",self.export_folder,self.name)).unwrap();
        obj_file.write_all(watermark.as_bytes()).expect("write failed");
        obj_file.write_all(oname.as_bytes()).expect("write failed");
        obj_file.write_all(mtllibname.as_bytes()).expect("write failed");
        if !self.vertices_normals.is_empty(){
            for v in 0..self.vertices_normals.len(){
                //let vn = &format!("{}vn {} {} {}\n",vn, v.nx, v.ny, v.nz);
                writeln!(&mut obj_file,  "vn {} {} {}",
                    &self.vertices_normals[v].nx,
                    &self.vertices_normals[v].ny,
                    &self.vertices_normals[v].nz);
            }
        }
        //obj_file.write_all(vn.as_bytes()).expect("write failed");
        //write vertices
        //let mut list_of_v = String::new();
        for v in 0..self.number_of_v_and_f.0{
            //is model center (0,0,0)?
            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;

            if self.center_model{
                let center = (lowest_coordinates.0 as f32+(shape.0 as f32/2.0),
                    lowest_coordinates.1 as f32+(shape.1 as f32/2.0),
                    lowest_coordinates.2 as f32+(shape.2 as f32/2.0));
                x = self.vertices[v as usize].x as f32 - center.0;
                y = self.vertices[v as usize].y as f32 - center.1;
                z = self.vertices[v as usize].z as f32 - center.2;
            }else {
                x = self.vertices[v as usize].x as f32;
                y = self.vertices[v as usize].y as f32;
                z = self.vertices[v as usize].z as f32;
            }
            //is y the up vector?
            if self.y_is_up{
                //list_of_v = format!("{}v {:?} {:?} {:?}\n",list_of_v,y,z,x);
                writeln!(&mut obj_file, "v {:?} {:?} {:?}",y,z,x);
            }else {
                //list_of_v = format!("{}v {:?} {:?} {:?}\n",list_of_v,x,y,z);
                writeln!(&mut obj_file, "v {:?} {:?} {:?}",x,y,z);


            }
            
        }

        //obj_file.write(list_of_v.as_bytes()).expect("write failed");
        //write vt
        let mut list_of_vt = String::new();
        if self.vt_precisionnumber == 0 && self.texture_map.w!=2 && self.texture_map.h!=2{
            if self.texture_map.w == 1{
                self.vt_precisionnumber = 0;
            }
            if self.texture_map.w < 10{
                self.vt_precisionnumber = 3;
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
        //println!("writing vt's with: {:?} digits", self.vt_precisionnumber);
        for vt in 0..self.vertices_uvs.len(){
            let x = self.vt_precisionnumber as usize;
            let u = format!("{:.*}", x, (self.vertices_uvs[vt].u as f32/self.texture_map.w as f32) as f32);
            let v = format!("{:.*}", x, 1.0-(self.vertices_uvs[vt].v as f32/self.texture_map.h as f32) as f32);
            list_of_vt = format!("{}vt {u} {v}\n",list_of_vt);
        }
        obj_file.write_all(list_of_vt.as_bytes()).expect("write failed");
        //write usemtl
        obj_file.write_all(usemtl.as_bytes()).expect("write failed");
        //write faces
        if !self.vertices_normals.is_empty(){
            for f in 0..self.number_of_v_and_f.1{
                let face = &self.faces[f as usize];
                writeln!(&mut obj_file, "f {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{}"
                ,face.a.0,face.a.1,face.a.2
                ,face.b.0,face.b.1,face.b.2
                ,face.c.0,face.c.1,face.c.2
                ,face.d.0,face.d.1,face.d.2);
                
            }
            
        }else{
            for f in 0..self.number_of_v_and_f.1{
            writeln!(&mut obj_file, "f {}/{} {}/{} {}/{} {}/{}"
                ,self.faces[f as usize].a.0,self.faces[f as usize].a.1
                ,self.faces[f as usize].b.0,self.faces[f as usize].b.1
                ,self.faces[f as usize].c.0,self.faces[f as usize].c.1
                ,self.faces[f as usize].d.0,self.faces[f as usize].d.1);
            }
        }
    }
    fn write_png(&self){
        if self.is_vox{
        let map = self.material_map.clone().unwrap();
        let file = File::create(format!("{}/{}.png",self.export_folder,self.name)).unwrap();
        let file_e = if self.allowed_materials.2{Some(File::create(
            format!("{}/{}_emit.png",self.export_folder,self.name)).unwrap())
        }else{None};
        let file_o = if self.allowed_materials.2{Some(File::create(
            format!("{}/{}_extra.png",self.export_folder,self.name)).unwrap())
        }else{None};
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w,map.w as u32, map.h as u32);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Best); 
        encoder.set_color(png::ColorType::Rgb);

        //Emission png
        let mut writer_e = None;
        if self.allowed_materials.2{
            let mut encoder_e = png::Encoder::new(BufWriter::new(file_e.unwrap()),map.w as u32, map.h as u32);
            encoder_e.set_depth(png::BitDepth::Eight);
            encoder_e.set_compression(png::Compression::Best); 
            encoder_e.set_color(png::ColorType::Rgb);
            writer_e = Some(encoder_e.write_header().unwrap());
        }
        //The other RGBA .png
        let mut writer_o = None;
        if self.allowed_materials.3||self.allowed_materials.4||self.allowed_materials.5||self.allowed_materials.6{
            let mut encoder_o = png::Encoder::new(BufWriter::new(file_o.unwrap()),map.w as u32, map.h as u32);
            encoder_o.set_depth(png::BitDepth::Eight);
            encoder_o.set_compression(png::Compression::Best); 
            encoder_o.set_color(png::ColorType::Rgba);
            writer_o = Some(encoder_o.write_header().unwrap());
        }

        //If there is transparency do a RGBA png
        if self.allowed_materials.1{
            encoder.set_color(png::ColorType::Rgba);
            let mut writer = encoder.write_header().unwrap();
            let mut data = Vec::new(); // An array containing an RGBA sequence
            let mut data_e = Vec::new();
            let mut data_o = Vec::new();
            for x in 0..map.id.len(){
                let m = &map.materials[map.id[x]as usize];
                data.push(m.rgb.r);
                data.push(m.rgb.g);
                data.push(m.rgb.b);
                data.push((m.transparent*255.0) as u8);
                data_e.push(m.rgb_e.unwrap().r);
                data_e.push(m.rgb_e.unwrap().g);
                data_e.push(m.rgb_e.unwrap().b);
                data_o.push((m.roughness*self.allowed_materials.3 as i32 as f32*255.0) as u8);
                data_o.push((m.metallic*self.allowed_materials.4 as i32 as f32*255.0) as u8);
                data_o.push((m.ior/2.0*self.allowed_materials.5 as i32 as f32*255.0) as u8);
                data_o.push((m.specular*self.allowed_materials.6 as i32 as f32*255.0) as u8);
            }
            //Albedo + Transparency map
            writer.write_image_data(&data).unwrap();
            //Emission map
            if writer_e.is_some(){
                writer_e.unwrap().write_image_data(&data_e).unwrap();
            }
            //Extra map
            if writer_o.is_some(){
                writer_o.unwrap().write_image_data(&data_o).unwrap();
            }
        //If no transparency then write a RGB map
        }else{
            let mut writer = encoder.write_header().unwrap();
            let mut data = Vec::new(); // An array containing an RGBA sequence
            let mut data_e = Vec::new();
            let mut data_o = Vec::new();
            for x in 0..map.id.len(){
                let m = &map.materials[map.id[x]as usize];
                data.push(m.rgb.r);
                data.push(m.rgb.g);
                data.push(m.rgb.b);
                //data.push((m.transparent*255) as u8);
                data_e.push(m.rgb_e.unwrap().r);
                data_e.push(m.rgb_e.unwrap().g);
                data_e.push(m.rgb_e.unwrap().b);
                data_o.push((m.roughness*self.allowed_materials.3 as i32 as f32*255.0) as u8);
                data_o.push((m.metallic*self.allowed_materials.4 as i32 as f32*255.0) as u8);
                data_o.push((m.ior/2.0*self.allowed_materials.5 as i32 as f32*255.0) as u8);
                data_o.push((m.specular*self.allowed_materials.6 as i32 as f32*255.0) as u8);
            }
            //Albedo Map
            writer.write_image_data(&data).unwrap();
            //Emission Map
            if writer_e.is_some(){
                writer_e.unwrap().write_image_data(&data_e).unwrap();
            }
            //Extra Map
            if writer_o.is_some(){
                writer_o.unwrap().write_image_data(&data_o).unwrap();
            }

        }

        }else{
        let file = File::create(format!("{}/{}.png",self.export_folder,self.name)).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.texture_map.w as u32, self.texture_map.h as u32); 
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Best);
        let mut writer = encoder.write_header().unwrap();
        let mut data = Vec::new(); // An array containing an RGB sequence
        for x in 0..self.texture_map.colours.len(){
            let p = self.texture_map.colours[x];
            if p.is_some(){
                data.push(p.unwrap().r);
                data.push(p.unwrap().g);
                data.push(p.unwrap().b);
            }else{
                data.push(self.background_color.r);
                data.push(self.background_color.g);
                data.push(self.background_color.b);
            }
        }
        writer.write_image_data(&data).unwrap();
        }
    }
    pub fn export_all(&mut self, shape:(i32,i32,i32),lowest_coordinates:(i32,i32,i32)){
        self.write_obj(shape, lowest_coordinates);
        self.write_mtl();
        self.write_png();
        /*
        tokio::try_join!(
            self.write_obj(shape, lowest_coordinates),
            self.write_mtl(),
            self.write_png()
        )
        .expect("Failed to export");
        */
    }
}
