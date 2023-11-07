
use std::fs;
use crate::greedy_mesher::OptimizedCube;
use crate::MyApp;

pub struct Obj{
    //meta-info
    name: String,
    number_of_v_and_f: (i32, i32),
    //--number of v, vt and f
    faces: Vec<ObjF>,
    vertices: Vec<ObjV>,
    vertices_uvs: Vec<ObjVt>,
    texture_map: TextureMap
}
pub struct TextureMap{
    w: usize,
    h: usize,
    colours: Vec<Vec<Rgb>>, //I might consider using a tuple(u8, u8, u8) instead of a struct,
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
                for y in 0..self.colours.len(){
                    for x in 0..self.colours.len(){
                        if equality_one == equality::ONE && self.colours[y][x] != t2.colours[y][x]{
                            equality_one = equality::NO;
                        }
                    }
                }
            }

        }
        if equality_one == equality::ONE{
            return equality_one;
        } 
        if typeofequality >= 2 {
            if self.w == t2.w && self.h == t2.h{
                for y in 0..self.colours.len(){
                    for x in 0..self.colours.len(){
                            
                            if equality_two180==equality::TWO_180 && self.colours[(self.w-y) as usize][(self.h-x)as usize] != t2.colours[y][x]{ 
                                equality_two180 = equality::NO; //180 (-x, -y)  
                            }
                    }
                }
            } else if self.w == t2.h && self.h == t2.w {
                for y in 0..self.colours.len(){
                    for x in 0..self.colours.len(){
                        if equality_two90==equality::TWO_90 && self.colours[(self.w-x) as usize][y] != t2.colours[y][x]{ 
                                equality_two90 = equality::NO; //90 (y, -x)  
                            }
                        
                        if equality_two270==equality::TWO_270 && self.colours[x][(self.h-y) as usize] != t2.colours[y][x]{
                            equality_two270 = equality::NO;
                        } //270 (-y, x)
                    }
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
                for y in 0..self.colours.len(){
                    for x in 0..self.colours.len(){
                        if eq_threex==equality::THREE_X && self.colours[y][(self.w-x) as usize] != t2.colours[y][x]{ 
                                eq_threex = equality::NO; //x (mirror x axis)
                            }
                        if eq_threey==equality::THREE_Y && self.colours[(self.h-y) as usize][x] != t2.colours[y][x]{ 
                                eq_threey = equality::NO; //y (mirror y axis)
                            }
                            
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
pub struct Rgb{
    r: u8,
    g: u8,
    b: u8
}

//todo()! -> implement an HashMap (obj_v, index_v) and an HashMap (obj_vt, index_vt)
pub struct ObjF{
    //index_v|index_vt
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32)
}
pub struct ObjV{
    x: i32,
    y: i32,
    z: i32
}
pub struct ObjVt{
    u: f32,
    v: f32
}
impl Obj{
    fn from_optimized_cubes(my_app: &MyApp, name: String, vector_of_optimized_cubes: Vec<&OptimizedCube>) -> Obj{
        if my_app.debug_uv_mode{
            //mtl will be the same but png is going to be a 2x2 of pink and black and there are going to be 4 vt's in the whole obj
            todo!()
        } else{
            //set up metadata, vertices, texture vertices, faces, textures
            todo!()
        }
    }
    fn write_mtl(&self){
        let x = format!("#@DL2023 - w:{:?}, h:{:?}\nnewmtl x\nmap_Kd {:?}.png", self.texture_map.w, self.texture_map.h, self.name);
        //todo!() -> write this to file
        todo!()
    }
    fn write_obj(&self){

        const WATERMARK: &[u8; 45] = b"#created with MagicaVoxel and VoxelOptimizer ";
        let nv = format!("v:{:?}, f:{:?}", self.number_of_v_and_f.0, self.number_of_v_and_f.1);
        let watermark = format!("{:?}{:?}", WATERMARK, nv);
        let name = self.name;
        let oname = format!("o {:?}", name);
        let mtllibname = format!("mtllib {:?}.mtl", name);
        const USEMTL: String = format!("usemtl x");
        todo!()
    }
    fn write_png(&self){
        todo!()
    }
    fn export_all(&self){
        self.write_obj();
        self.write_mtl();
        self.write_png();
    }
}
