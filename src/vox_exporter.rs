
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
    pub y_is_up: bool,
    pub center_model: bool,
    pub background_color: Rgb,
}
#[derive(Copy, Debug, PartialEq)]
#[derive(Clone)]
pub struct Rgb{
    pub r: u8,
    pub g: u8,
    pub b: u8
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
    fn is_equal(&self, t2: &TextureMap, typeofequality: i32) -> equality{

        if typeofequality == 0 || self.colours.len() != t2.colours.len(){
            return equality::NO;
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
        if typeofequality >= 1{
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len(){
                        if equality_one == true && self.colours[x] != t2.colours[x]{
                            equality_one = false;
                        }
                    }
            }

        }
        if equality_one == true{
            //println!("{:?}", "it's a match! One");
            return equality::ONE;
        } 
        if typeofequality >= 2 {
            if self.w == t2.h && self.h == t2.w{
                let t1 = self.rotate();
                if t1.is_equal(t2, 1) != equality::NO{return equality::TWO_90}
            }
            if self.w == t2.w && self.h == t2.h{
                let t1 = self.rotate().rotate();
                if t1.is_equal(t2, 1) != equality::NO{return equality::TWO_180}
            }
            if self.w == t2.h && self.h == t2.w{
                let t1 = self.rotate().rotate().rotate();
                if t1.is_equal(t2, 1) != equality::NO{return equality::TWO_270}
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
        if typeofequality >= 3{
            if self.w == t2.w && self.h == t2.h{
                if self.flipx().is_equal(t2, 1) != equality::NO{return equality::THREE_X}
                if self.flipy().is_equal(t2, 1) != equality::NO{return equality::THREE_Y}
            }
            /*
            if self.w == t2.w && self.h == t2.h{
                    for x in 0..self.colours.len(){
                        let w = self.w;
                        let h = self.h;
                        let m = x%h;
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
                */
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
        return equality::NO

    }
    //_______________________________________x___y____
    fn scalar_to_coordinates(w:i32, i:i32)->(i32,i32){
        return ((i%w)as i32,((i-(i%w))/w) as i32);
    }
    //__________________________________x____y__________index______
    fn coordinates_to_scalar(w:i32, xy:(i32,i32))->i32{
        return xy.0+(xy.1*w);
    }
    fn rotate(&self)->TextureMap{
        let mut buffer1 = Vec::new();
        let h = self.w;
        let w = self.h;
        for x in 0..self.colours.len(){
            let i = TextureMap::scalar_to_coordinates(self.w as i32, x as i32);
            let ii = TextureMap::coordinates_to_scalar(w as i32, ((w as i32 -1 - i.1) as i32, i.0));
            buffer1.push(self.colours[ii as usize]);
        }
        TextureMap{
            w:w,
            h:h,
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
            w:w,
            h:h,
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
            w:w,
            h:h,
            colours: buffer1,
        }
    }
    fn is_texture_some(&self)->bool{
        for pixel in 0..self.colours.len(){
            if self.colours[pixel].is_some(){
                return true;
            }
        }
        return false;
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
    u: i32,
    v: i32
}
fn add_two_tuples(a: (i32, i32, i32), b:(i32,i32,i32))->(i32,i32,i32){return (a.0+b.0, a.1+b.1, a.2+b.2)}
impl Obj{
    pub fn from_optimized_cubes(path: PathBuf,my_app: &MyApp, opcubes: &Vec<OptimizedCube>) -> Obj{
        //println!("my_app.vt_precisionnumber:{:?}", my_app.vt_precisionnumber);
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
            y_is_up: my_app.y_is_up,
            center_model: my_app.center_model_in_mesh,
            background_color: Rgb{r:(my_app.background_color[0]*255.0) as u8
                                 ,g:(my_app.background_color[1]*255.0) as u8
                                 ,b:(my_app.background_color[2]*255.0) as u8},
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
        obj.number_of_v_and_f.0 = temp_v.len() as i32;
        for _x in 0..obj.number_of_v_and_f.0{
            obj.vertices.push(ObjV::default());
        }
        for (k,v) in &temp_v{           
            obj.vertices[(*v as usize)-1] = ObjV::from_xyz(k.0, k.1, k.2);
        }
        //push the vertices into the list (there is nothing more we can do)
        //println!("{:?}", temp_v.len());
        //println!("{:?}", temp_v); 
        

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
        let mut tid: Vec<(Option<i32>, equality)> = Vec::new();
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
                                if previousp.is_some() && currentp.is_some(){
                                    if previousp.unwrap() != currentp.unwrap(){
                                        //texture isn't all of the same colour
                                        is_all_same_colour = false;
                                    }
                                }
                                
                            }
                        }
                    }
                }
                //println!("is texture some = {:?}", is_texture_some);
                //println!("is all same color = {:?}", is_all_same_colour);
                if !is_texture_some{
                    tid.push((None, equality::NO));
                } else if is_texture_some{
                    //the texture is going to depend on if it is a single colour or more
                    let mut tex = opcubes[x].textures[t].clone();
                    if is_all_same_colour{
                        let mut c = obj.background_color;
                        let mut i = 0;  
                        while opcubes[x].textures[t].colours[i].is_some() == false{
                            i+=1;
                        }      
                        tex = TextureMap{w:1,h:1, colours:[opcubes[x].textures[t].colours[i]].to_vec()}
                    }
                    //println!("{:?}", tex);

                    if unique_tid.len()==0{
                        //if all the texture is of a colour just push that colour
                        unique_tid.push(tex);
                        //tid.push((Some(((x*6)+t) as i32), equality::NO));
                        tid.push((Some((unique_tid.len() - 1) as i32), equality::NO))
                        
                    //if there is a unique texture already
                    }else {
                        //if it is just one colour check if that colour exists already
                        if my_app.pattern_matching == 0{
                            unique_tid.push(tex.clone());
                            tid.push((Some((unique_tid.len() - 1) as i32), equality::NO))
                        }else{
                            let mut equ = equality::NO;
                            let mut ii = 0;
                            for i in 0..unique_tid.len(){

                                if equ == equality::NO{
                                    match tex.is_equal(&unique_tid[i], my_app.pattern_matching){
                                        equality::NO =>{}
                                        equality::ONE =>{ii=i as i32;equ = equality::ONE;}
                                        equality::TWO_90 =>{ii=i as i32;equ = equality::TWO_90;}
                                        equality::TWO_180 =>{ii=i as i32;equ = equality::TWO_180;}
                                        equality::TWO_270 =>{ii=i as i32;equ = equality::TWO_270;}
                                        equality::THREE_X =>{ii=i as i32;equ = equality::THREE_X;}
                                        equality::THREE_Y =>{ii=i as i32;equ = equality::THREE_Y;}
                                    }
                                }
                                //println!("opcubes[{:?}].textures[{:?}] equality::{:?} unique_tid[{:?}]",x,t,equ,i);
                            }
                            //println!("opcubes[{:?}].textures[{:?}] equality::{:?} with the rest of the textures",x,t,equ);
                            if equ == equality::NO{
                                unique_tid.push(tex.clone());
                                tid.push((Some((unique_tid.len()-1) as i32), equality::NO));
                            }
                            if equ != equality::NO {
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
                    equality::NO => {
                        aa = *a;
                        bb = *b;
                        cc = *c;
                        dd = *d;
                    }equality::ONE => {
                        aa = *a;
                        bb = *b;
                        cc = *c;
                        dd = *d;
                    }equality::TWO_90=> {
                        aa = *b;
                        bb = *c;
                        cc = *d;
                        dd = *a;
                    }equality::TWO_180=> {
                        aa = *c;
                        bb = *d;
                        cc = *a;
                        dd = *b;
                    }equality::TWO_270=> {
                        dd = *c;
                        cc = *b;
                        bb = *a;
                        aa = *d;
                    }equality::THREE_X=> {
                        dd = *c;
                        cc = *d;
                        bb = *a;
                        aa = *b;
                    }equality::THREE_Y=> {
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
        return obj;

    }
    fn write_mtl(&self){
        let x = format!("#@DL2023 - w:{:?}, h:{:?}\nnewmtl x\nmap_Kd {}.png", self.texture_map.w, self.texture_map.h, self.name);
        //todo!() -> write this to file
        //println!("export/{}.mtl",self.name);
        let mut mtl_file = File::create(format!("{}/{}.mtl",self.export_folder,self.name)).expect("creation failed");
                mtl_file.write(x.as_bytes()).expect("write failed");
    }
    fn write_obj(&mut self, shape:(i32,i32,i32),lowest_coordinates:(i32,i32,i32)){
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
                self.vt_precisionnumber = 2;
            } else if self.texture_map.w < 100{
                self.vt_precisionnumber = 3;
            } else if self.texture_map.w < 1000{
                self.vt_precisionnumber = 4;
            } else if self.texture_map.w < 10000{
                self.vt_precisionnumber = 5;
            }else if self.texture_map.w < 100000{
                self.vt_precisionnumber = 6;
            }
        }
        //println!("writing vt's with: {:?} digits", self.vt_precisionnumber);
        for vt in 0..self.vertices_uvs.len(){
            let x = self.vt_precisionnumber as usize;
            let u = format!("{:.*}", x, (self.vertices_uvs[vt].u as f32/self.texture_map.w as f32) as f32);
            let v = format!("{:.*}", x, 1.0-(self.vertices_uvs[vt].v as f32/self.texture_map.h as f32) as f32);
            list_of_vt = format!("{}vt {u} {v}\n",list_of_vt);
        }
        obj_file.write(list_of_vt.as_bytes()).expect("write failed");
        //write usemtl
        obj_file.write(USEMTL.as_bytes()).expect("write failed");
        //write faces
        //let mut list_of_f = String::new();
        for f in 0..self.number_of_v_and_f.1{
            /*
            list_of_f = format!("{}f {}/{} {}/{} {}/{} {}/{}\n",
                list_of_f,self.faces[f as usize].a.0,self.faces[f as usize].a.1
                ,self.faces[f as usize].b.0,self.faces[f as usize].b.1
                ,self.faces[f as usize].c.0,self.faces[f as usize].c.1
                ,self.faces[f as usize].d.0,self.faces[f as usize].d.1);
                */
            writeln!(&mut obj_file, "f {}/{} {}/{} {}/{} {}/{}"
                ,self.faces[f as usize].a.0,self.faces[f as usize].a.1
                ,self.faces[f as usize].b.0,self.faces[f as usize].b.1
                ,self.faces[f as usize].c.0,self.faces[f as usize].c.1
                ,self.faces[f as usize].d.0,self.faces[f as usize].d.1);
        }
        //obj_file.write(list_of_f.as_bytes()).expect("write failed");



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
    pub fn export_all(&mut self, shape:(i32,i32,i32),lowest_coordinates:(i32,i32,i32)){
        self.write_obj(shape, lowest_coordinates);
        self.write_mtl();
        self.write_png();
    }
}
