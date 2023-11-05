use ndarray::Array2;
use std::fs;
use crate::greedy_mesher::OptimizedCube;
use crate::MyApp;

pub struct Obj{
    //meta-info
    name: String,
    number_of_v_and_f: (i32, i32),
    //--number of v, vt and f
    faces: Vec<obj_f>,
    vertices: Vec<obj_v>,
    vertices_uvs: Vec<obj_vt>,
    texture_map: TextureMap
}
pub struct TextureMap{
    w: i32,
    h: i32,
    colours: Array2<rgb> //I might consider using a tuple(u8, u8, u8) instead of a struct,
                        //however in this way I could implement an equality comparator for rgb struct
}
pub struct rgb{
    r: u8,
    g: u8,
    b: u8
}

//todo()! -> implement an HashMap (obj_v, index_v) and an HashMap (obj_vt, index_vt)
pub struct obj_f{
    //index_v|index_vt
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32)
}
pub struct obj_v{
    x: i32,
    y: i32,
    z: i32
}
pub struct obj_vt{
    u: f32,
    v: f32
}
impl Obj{
    fn from_optimized_cubes(my_app: &MyApp, name: String, vector_of_optimized_cubes: Vec<&OptimizedCube>, debug: bool) -> Obj{
        if my_app.debug_uv_mode{
            //mtl will be the same but png is going to be a 2x2 of pink and black and there are going to be 4 vt's in the whole obj
            todo!()
        } else{
            //set up metadata, vertices, texture vertices, faces, textures
            todo!()
        }
    }
    fn write_mtl(&self){
        todo!()
    }
    fn write_obj(&self){
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
