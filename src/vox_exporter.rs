
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::io::BufWriter;
use std::path::Path;
use crate::greedy_mesher::{OptimizedCube, OptimizedVox};
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
    //pub is_vox: bool,
    pub faces: Vec<ObjF>,
    pub vertices: Vec<ObjV>,
    pub vertices_uvs: Vec<ObjVt>,
    pub vertices_normals: Vec<ObjVn>,
    pub texture_map: Option<TextureMap>,
    pub material_map: Option<MaterialMap>,
    pub materials: Option<Vec<vox_importer::Matl>>,
    ///allowed materials in order are 0: Albedo, 1: Alpha, 2: Rgb_e, 3: Roughness, 4: Metal, 5:Spec, 6: Ior (total len = 7)
    pub allowed_materials: (bool, bool, bool, bool, bool, bool, bool),
    pub vt_precisionnumber: u8,
    pub y_is_up: bool,
    pub right_handed: bool,
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
    pub id: Vec<Vec<u8>>,
    pub materials: Vec<vox_importer::Matl>,
}
impl PartialEq for MaterialMap{
    fn eq(&self, rhs: &Self) -> bool{
        if self.w != rhs.w && self.h != rhs.h{
            return false;
        }
        for y in 0..self.h{
            for x in 0..self.w{
                if self.id[y][x] != rhs.id[y][x]{
                    return false;
                }
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl MaterialMap{
    fn is_equal(&self, t2: &Self, pattern_matching: bool) -> Equality{

        if !pattern_matching || self.w*self.h != t2.w*t2.h{
            return Equality::No;
        }
        let matrices = [
        ((1,0),(0,1)),
        ((-1,0),(0,1)),
        ((-1,0),(0,-1)),
        ((1,0),(0,-1)),
        ((0,1),(1,0)),
        ((0,1),(-1,0)),
        ((0,-1),(-1,0)),
        ((0,-1),(1,0)),];
        for matrix in matrices.iter(){
            if *self == t2.apply_rotation_matrix(*matrix){
                return Equality::Yes(matrix.0,matrix.1);
            }
        }
        Equality::No //return

    }
    fn apply_rotation_matrix(&self, r:((i32,i32),(i32,i32)))->MaterialMap{
        let mut buffer = Vec::new();
        let unit_vector = bidimensional_column_x_matrix((1,1), r.clone());
        let (w,h) = (r.0.0*self.w as i32+r.0.1*self.h as i32,r.1.0*self.w as i32+r.1.1*self.h as i32);
        let w = w.abs();
        let h = h.abs();
        for y in 0..h{
            buffer.push(Vec::new());
            for x in 0..w{
                buffer[y as usize].push(0);
            }
        }
        for y in 0..self.h{
            for x in 0..self.w{
                let mut xy = (r.0.0*x as i32+r.0.1*y as i32,r.1.0*x as i32+r.1.1*y as i32);
                if unit_vector.0 <0{xy.0+=w-1;}
                if unit_vector.1<0{xy.1+=h-1;}
                buffer[xy.1 as usize][xy.0 as usize]=self.id[y as usize][x as usize];
            }
        }
        MaterialMap { w: w as usize, h: h as usize, id: buffer, materials: self.materials.clone() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureMap{
    pub w: usize,
    pub h: usize,
    pub colours: Vec<Vec<Option<Rgb>>>, //I might consider using a tuple(u8, u8, u8) instead of a struct,
                        //however in this way I could implement an equality comparator for rgb struct
}
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Equality{
    No,
    Yes((i32,i32),(i32,i32)),
}
impl TextureMap{
    fn is_equal(&self, t2: &Self, pattern_matching: bool) -> Equality{

        if !pattern_matching || self.w*self.h != t2.w*t2.h{
            return Equality::No;
        }
        let matrices = [
        ((1,0),(0,1)),
        ((-1,0),(0,1)),
        ((-1,0),(0,-1)),
        ((1,0),(0,-1)),
        ((0,1),(1,0)),
        ((0,1),(-1,0)),
        ((0,-1),(-1,0)),
        ((0,-1),(1,0)),];
        for matrix in matrices.iter(){
            if *self == t2.apply_rotation_matrix(*matrix){
                return Equality::Yes(matrix.0,matrix.1);
            }
        }
        Equality::No //return

    }
    fn apply_rotation_matrix(&self, r:((i32,i32),(i32,i32)))->TextureMap{
        let mut buffer = Vec::new();
        let unit_vector = bidimensional_column_x_matrix((1,1), r.clone());
        let (w,h) = (r.0.0*self.w as i32+r.0.1*self.h as i32,r.1.0*self.w as i32+r.1.1*self.h as i32);
        let w = w.abs();
        let h = h.abs();
        for y in 0..h{
            buffer.push(Vec::new());
            for x in 0..w{
                buffer[y as usize].push(None);
            }
        }
        for y in 0..self.h{
            for x in 0..self.w{
                let mut xy = (r.0.0*x as i32+r.0.1*y as i32,r.1.0*x as i32+r.1.1*y as i32);
                if unit_vector.0 <0{xy.0+=w-1;}
                if unit_vector.1<0{xy.1+=h-1;}
                buffer[xy.1 as usize][xy.0 as usize]=self.colours[y as usize][x as usize];
            }
        }
        TextureMap { w: w as usize, h: h as usize, colours: buffer}
    }

    fn is_texture_some(&self)->bool{
        for y in 0..self.colours.len(){
            for x in 0..self.colours[y].len(){
                if self.colours[y][x].is_none(){
                    return false;
                }
            }
        }
        true //return
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
fn bidimensional_column_x_matrix(n: (i32,i32),m:((i32,i32),(i32,i32)))->(i32,i32){
    let x = m.0.0*n.0+m.0.1*n.1;
    let y = m.1.0*n.0+m.1.1*n.1;
    (x,y) //return

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
    pub fn from_optimized_cubes(path: PathBuf,my_app: &MyApp, opcubes: &Vec<OptimizedCube>) -> Obj{
        let x = path.file_name().unwrap().to_str().unwrap().to_string().trim_end_matches(".ply").to_string();
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
            texture_map: Some(TextureMap{w:0, h:0, colours:Vec::new()}),
            vt_precisionnumber: my_app.vt_precisionnumber,
            y_is_up: my_app.y_is_up,
            right_handed: my_app.right_handed,
            center_model: my_app.center_model_in_mesh,
            background_color: Rgb{r:(my_app.background_color[0]*255.0) as u8
                                 ,g:(my_app.background_color[1]*255.0) as u8
                                 ,b:(my_app.background_color[2]*255.0) as u8},
            material_map: None,
            materials: None,
            allowed_materials,
            vertices_normals: Vec::new(),

        };
        //If normals then write normals
        if my_app.normals{
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
        }
        /*
        if my_app.normals{
            match (my_app.y_is_up == true, my_app.right_handed == true){
                //Godot
                (true, true) => {
                    //Top
                    obj.vertices_normals.push(ObjVn { nx: 0, ny:  1, nz: 0 });
                    //Bottom
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: -1, nz: 0 });
                    //Forward
                    obj.vertices_normals.push(ObjVn { nx: -1, ny: 0, nz: 0 });
                    //Backwards
                    obj.vertices_normals.push(ObjVn { nx:  1, ny: 0, nz: 0 });
                    //Right
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz:  1 });
                    //Left
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: -1 });

                },
                //Unity
                (true, false) => {
                    //Top
                    obj.vertices_normals.push(ObjVn { nx: 0, ny:  1, nz: 0 });
                    //Bottom
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: -1, nz: 0 });
                    //Forward
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: -1 });
                    //Backwards
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz:  1 });
                    //Right
                    obj.vertices_normals.push(ObjVn { nx: 1, ny: 0, nz:  0 });
                    //Left
                    obj.vertices_normals.push(ObjVn { nx: -1, ny: 0, nz: 0 });

                },
                //Magicavoxel,Blender
                (false, true) => {


                },
                //Unreal engine
                (false, false) => {
                    //Top
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz:  1 });
                    //Bottom
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: 0, nz: -1 });
                    //Forward
                    obj.vertices_normals.push(ObjVn { nx: -1, ny: 0, nz: 0 });
                    //Backwards
                    obj.vertices_normals.push(ObjVn { nx:  1, ny: 0, nz: 0 });
                    //Right
                    obj.vertices_normals.push(ObjVn { nx: 0, ny:  1, nz: 0 });
                    //Left
                    obj.vertices_normals.push(ObjVn { nx: 0, ny: -1, nz: 0 });

                },
            } }
        */
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
            obj.texture_map = Some(TextureMap{w:2, h:2,colours:[[Some(Rgb{r:255,g:0,b:255}),Some(Rgb{r:0,g:0,b:0})].to_vec(),[Some(Rgb{r:0,g:0,b:0}),Some(Rgb{r:255,g:0,b:255})].to_vec()].to_vec()});
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
                let is_texture_some = true;//opcubes[x].textures[t].is_texture_some();
                let mut the_colour = None;
                let mut is_all_same_colour = true;
                if opcubes[x].textures[t].w == 1 && opcubes[x].textures[t].h==1{
                    the_colour=opcubes[x].textures[t].colours[0][0];
                }
                //let mut pixels =
                for y in 0..opcubes[x].textures[t].colours.len(){
                    for pixel in 0..opcubes[x].textures[t].colours[y].len(){
                        if my_app.monochrome && y+pixel!=0{
                            let (yy,xx) = if pixel==0{(y-1,opcubes[x].textures[t].w-1)}else{(y,pixel-1)};
                            let previousp = &opcubes[x].textures[t].colours[yy][xx].clone();
                            let currentp = &opcubes[x].textures[t].colours[y][pixel].clone();
                            the_colour = *currentp;
                            if previousp.is_some() &&
                                currentp.is_some() &&
                                *previousp != *currentp {
                                //texture isn't all of the same colour
                                is_all_same_colour = false;

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
                        tex = TextureMap{w:1,h:1, colours:[[the_colour].to_vec()].to_vec()}
                    }
                    //println!("{:?}", tex);

                    if unique_tid.is_empty(){
                        //if all the texture is of a colour just push that colour
                        unique_tid.push(tex);
                        tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))

                    //if there is a unique texture already
                    }else {
                        //if it is just one colour check if that colour exists already
                        if !my_app.pattern_matching{
                            unique_tid.push(tex.clone());
                            tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))
                        }else{
                            let mut equ = Equality::No;
                            let mut ii = 0;
                            for i in 0..unique_tid.len(){

                                if equ == Equality::No{
                                    match tex.is_equal(&unique_tid[i], my_app.pattern_matching){
                                        Equality::No =>{},
                                        Equality::Yes(x,y) =>{ii=i as i32;equ = Equality::Yes(x,y);}
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
        for y in 0..finaltexture.h{
            finaltexture.colours.push(Vec::new());
            for _x in 0..finaltexture.h{
                finaltexture.colours[y].push(Some(obj.background_color));
            }

        }
        for item in &packed {
            positions[item.data] = (item.rect.x, item.rect.y);
            for y in 0..unique_tid[item.data].colours.len(){
                for x in 0..unique_tid[item.data].colours[y].len(){
                    let ppp = (x as i32+item.rect.x as i32, y as i32+item.rect.y as i32);
                    finaltexture.colours[ppp.1 as usize][ppp.0 as usize] = unique_tid[item.data].colours[y][x];
                }
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
                    }Equality::Yes(x,y) => {
                        let bl = find_key_for_value(temp_vt.clone(), *d).unwrap();
                        let tl = find_key_for_value(temp_vt.clone(), *a).unwrap();
                        let tr = find_key_for_value(temp_vt.clone(), *b).unwrap();
                        let br = find_key_for_value(temp_vt.clone(), *c).unwrap();
                        let size_x = br.0-bl.0;
                        let size_y = tl.1-bl.1;
                        let t = (&bl.0,&bl.1);
                        let size_vector = (size_x,size_y);
                        let size_vector = bidimensional_column_x_matrix(size_vector, (x,y));
                        let mut bottom_left = bidimensional_column_x_matrix((bl.0-t.0,bl.1-t.1), (x,y));
                        let mut top_left = bidimensional_column_x_matrix((tl.0-t.0,tl.1-t.1), (x,y));
                        let mut top_right = bidimensional_column_x_matrix((tr.0-t.0,tr.1-t.1), (x,y));
                        let mut bottom_right = bidimensional_column_x_matrix((br.0-t.0,br.1-t.1), (x,y));
                        if size_vector.0<0{
                            bottom_left.0-=size_vector.0;
                            top_left.0-=size_vector.0;
                            top_right.0-=size_vector.0;
                            bottom_right.0-=size_vector.0;
                        }
                        if size_vector.1<0{
                            bottom_left.1-=size_vector.1;
                            top_left.1-=size_vector.1;
                            top_right.1-=size_vector.1;
                            bottom_right.1-=size_vector.1;
                        }
                        //dbg!((bl,&bottom_left),(tl,&top_left),(tr,&top_right),(br,&bottom_right), size_x, size_y, size_vector);
                        dd = *temp_vt.get(&(bottom_left.0+t.0,bottom_left.1+t.1)).unwrap();
                        aa = *temp_vt.get(&(top_left.0+t.0,top_left.1+t.1)).unwrap();
                        bb = *temp_vt.get(&(top_right.0+t.0,top_right.1+t.1)).unwrap();
                        cc = *temp_vt.get(&(bottom_right.0+t.0,bottom_right.1+t.1)).unwrap();
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
        obj.texture_map = Some(finaltexture);
        //dbg!(&my_app.normals, &obj.vertices_normals);
        obj //return
    }
    pub fn from_optimized_vox(path: PathBuf,my_app: &MyApp, opcubes: &Vec<OptimizedVox>, materials: Vec<vox_importer::Matl>) -> Obj{
        let x = path.file_name().unwrap().to_str().unwrap().to_string().trim_end_matches(".vox").to_string();
        let y = x.replace(' ', "");
        let xx = my_app.picked_path.clone().unwrap().to_string();
        let yy = xx.replace("\\", "/");
        let mut allowed_materials = {
            (true,
            my_app.transparency,
            my_app.emission,
            my_app.roughness,
            my_app.metal,
            my_app.refraction,
            my_app.specular)
        };
        let mut helperbool = false;
        for matl in materials.clone().iter(){
            if matl.rgb_e.is_some(){
                helperbool = true;
            }
        }
        allowed_materials.2 = helperbool;
        let mut helperbool = false;
        for matl in materials.clone().iter(){
            if matl.transparent != 0.0{
                helperbool = true;
            }
        }
        allowed_materials.1 = helperbool;
        let mut obj = Obj {
            name: y,
            export_folder: yy,
            number_of_v_and_f: (0,0),
            faces: Vec::new(),
            vertices: Vec::new(),
            vertices_uvs: Vec::new(),
            texture_map: None,
            vt_precisionnumber: my_app.vt_precisionnumber,
            y_is_up: my_app.y_is_up,
            right_handed: my_app.right_handed,
            center_model: my_app.center_model_in_mesh,
            background_color: Rgb{r:(my_app.background_color[0]*255.0) as u8
                                 ,g:(my_app.background_color[1]*255.0) as u8
                                 ,b:(my_app.background_color[2]*255.0) as u8},
            material_map: Some(MaterialMap { w: 1, h: 1, id: Vec::new(), materials: materials.clone() }),
            materials: Some(materials.clone()),
            allowed_materials,
            vertices_normals: Vec::new(),

        };
        //If normals then write normals
        if my_app.normals{
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
        }

        //insert unique vertices
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
        //push the vertices
        obj.number_of_v_and_f.0 = temp_v.len() as i32;
        for _x in 0..obj.number_of_v_and_f.0{
            obj.vertices.push(ObjV::default());
        }
        for (k,v) in &temp_v{
            obj.vertices[*v-1] = ObjV::from_xyz(k.0, k.1, k.2);
        }
        //push the vertices into the list (there is nothing more we can do)


        //faces
        for x in 0..opcubes.len(){
                //0x 0y 0z
                let a = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                //1x 0y 0z
                let b = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2);
                //1x 1y 0z
                let c = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                //0x 1y 0z
                let d = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2);
                //0x 0y 1z
                let e = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                //1x 0y 1z
                let f = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                //1x 1y 1z
                let g = ObjV::from_xyz(
                     opcubes[x].starting_position.0+opcubes[x].dimensions.0 as i32,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);
                //0x 1y 1z
                let h = ObjV::from_xyz(
                     opcubes[x].starting_position.0,
                     opcubes[x].starting_position.1+opcubes[x].dimensions.1 as i32,
                     opcubes[x].starting_position.2+opcubes[x].dimensions.2 as i32);

                //face 1 top
                if opcubes[x].is_face_enabled[0]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,1),
                    b:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,1),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,1),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,1),
                });}
                //face 2 bottom
                if opcubes[x].is_face_enabled[1]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,2),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,2),
                    c:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,2),
                    d:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,2),
                });}
                //face 3 left
                if opcubes[x].is_face_enabled[2]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,6),
                    b:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,6),
                    c:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,6),
                    d:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,6),
                });}
                //face 4 right
                if opcubes[x].is_face_enabled[3]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,5),
                    b:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,5),
                    c:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,5),
                    d:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,5),
                });}
                //face 5 front
                if opcubes[x].is_face_enabled[4]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(a.x,a.y,a.z)).unwrap() as i32,0,3),
                    b:(*temp_v.get(&(b.x,b.y,b.z)).unwrap() as i32,0,3),
                    c:(*temp_v.get(&(f.x,f.y,f.z)).unwrap() as i32,0,3),
                    d:(*temp_v.get(&(e.x,e.y,e.z)).unwrap() as i32,0,3),
                });}
                //face 6 back
                if opcubes[x].is_face_enabled[5]{
                obj.faces.push(ObjF{
                    a:(*temp_v.get(&(c.x,c.y,c.z)).unwrap() as i32,0,4),
                    b:(*temp_v.get(&(d.x,d.y,d.z)).unwrap() as i32,0,4),
                    c:(*temp_v.get(&(h.x,h.y,h.z)).unwrap() as i32,0,4),
                    d:(*temp_v.get(&(g.x,g.y,g.z)).unwrap() as i32,0,4),
                });}
        }

        obj.number_of_v_and_f.1 = obj.faces.len() as i32;
        let mut tid: Vec<(Option<i32>, Equality)> = Vec::new();
        let mut unique_tid: Vec<MaterialMap> = Vec::new();
        let mut temp_vt: HashMap<(i32,i32),i32> = HashMap::new();
        let mut positions = Vec::new();
        //println!("{:?}",obj);

        if my_app.debug_uv_mode{

            obj.vertices_uvs.push(ObjVt{u:0, v:0});
            obj.vertices_uvs.push(ObjVt{u:0, v:2});
            obj.vertices_uvs.push(ObjVt{u:2, v:2});
            obj.vertices_uvs.push(ObjVt{u:2, v:0});
            let matl_pink = vox_importer::Matl{id: 0, rgb:Rgb{r:255,g:0,b:255},..Default::default()};
            let matl_black = vox_importer::Matl{id: 1, rgb:Rgb{r:0,g:0,b:0},..Default::default()};
            let materials2 = [matl_pink,matl_black].to_vec();
            obj.material_map = Some(MaterialMap{w:2, h:2,id:[[0,1].to_vec(),[1,0].to_vec()].to_vec(), materials: materials2 });
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
                if !opcubes[x].is_face_enabled[t]{
                    //skip
                }else{
                    //println!("opcubes[{:?}].textures[{:?}] = {:?}", x,t,opcubes[x].textures[t]);
                    //what to do if texture is empty or all of the same colour?
                    let is_texture_some = true;//opcubes[x].textures[t].is_texture_some();
                    let mut the_colour = 0;
                    let mut is_all_same_colour = true;
                    if opcubes[x].textures[t].w == 1 && opcubes[x].textures[t].h==1{
                        the_colour=opcubes[x].textures[t].id[0][0];
                    }
                    //let mut pixels =
                    for y in 0..opcubes[x].textures[t].id.len(){
                        for pixel in 0..opcubes[x].textures[t].id[y].len(){
                            if my_app.monochrome && y+pixel!=0{
                                let (yy,xx) = if pixel==0{(y-1,opcubes[x].textures[t].w-1)}else{(y,pixel-1)};
                                let previousp = &opcubes[x].textures[t].id[yy][xx].clone();
                                let currentp = &opcubes[x].textures[t].id[y][pixel].clone();
                                the_colour = *currentp;
                                if *previousp != 0 &&
                                    *currentp != 0 &&
                                    *previousp != *currentp {
                                    //texture isn't all of the same colour
                                    is_all_same_colour = false;

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
                            tex = MaterialMap{w:1,h:1, id:[[the_colour].to_vec()].to_vec(), materials:materials.clone()}
                        }
                        //println!("{:?}", tex);

                        if unique_tid.is_empty(){
                            //if all the texture is of a colour just push that colour
                            unique_tid.push(tex);
                            tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))

                        //if there is a unique texture already
                        }else {
                            //if it is just one colour check if that colour exists already
                            if !my_app.pattern_matching{
                                unique_tid.push(tex.clone());
                                tid.push((Some((unique_tid.len() - 1) as i32), Equality::No))
                            }else{
                                let mut equ = Equality::No;
                                let mut ii = 0;
                                for i in 0..unique_tid.len(){

                                    if equ == Equality::No{
                                        match tex.is_equal(&unique_tid[i], my_app.pattern_matching){
                                            Equality::No =>{},
                                            Equality::Yes(x,y) =>{ii=i as i32;equ = Equality::Yes(x,y);}
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
                panic!("Final texture is too large (over 100 thousand * 100 thousand");
            }
        }
        let packed = match pack(container, items) {
            Ok(all_packed) => {all_packed},
            Err(some_packed) => {println!("{:?}", "some packed");some_packed},
        };
        let mut finaltexture = MaterialMap{w:container.w, h:container.h, id:Vec::new(), materials: materials.clone()};
        for y in 0..finaltexture.h{
            finaltexture.id.push(Vec::new());
            for _x in 0..finaltexture.w{
                finaltexture.id[y].push(0);
            }
        }
        for item in &packed {
            positions[item.data] = (item.rect.x, item.rect.y);
            for y in 0..unique_tid[item.data].id.len(){
                for x in 0..unique_tid[item.data].id[y].len(){
                    let ppp = (x as i32+item.rect.x as i32, y as i32+item.rect.y as i32);
                    finaltexture.id[ppp.1 as usize][ppp.0 as usize] = unique_tid[item.data].id[y][x];
                }
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
                //Top left
                let a = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32,(unique_tid[tid[x].0.unwrap() as usize].h)as i32 + positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                //Top right
                let b = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32 + (unique_tid[tid[x].0.unwrap() as usize].w)as i32,(unique_tid[tid[x].0.unwrap() as usize].h)as i32 + positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                //Bottom Right
                let c = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32 + (unique_tid[tid[x].0.unwrap() as usize].w)as i32,positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();
                //Bottom Left
                let d = temp_vt.get(&(positions[tid[x].0.unwrap() as usize].0 as i32,positions[tid[x].0.unwrap() as usize].1 as i32)).unwrap();

                match tid[x].1 {
                    Equality::No => {
                        aa = *a;
                        bb = *b;
                        cc = *c;
                        dd = *d;
                    }Equality::Yes(x,y) => {
                        match (x,y){
                            ((1,0),(0,1)) =>{
                                dd = *d; //Bottom Left
                                aa = *a; //Top Left
                                bb = *b; //Top Right
                                cc = *c; //Bottom Right
                            },
                            ((-1,0),(0,1)) =>{
                                dd = *c; //Bottom Left
                                aa = *b; //Top Left
                                bb = *a; //Top Right
                                cc = *d; //Bottom Right
                            },
                            ((1,0),(0,-1)) =>{
                                dd = *c; //Bottom Left
                                aa = *b; //Top Left
                                bb = *a; //Top Right
                                cc = *d; //Bottom Right
                            },
                            ((-1,0),(0,-1)) =>{
                                dd = *b; //Bottom Left
                                aa = *c; //Top Left
                                bb = *d; //Top Right
                                cc = *a; //Bottom Right
                            },
                            ((0,1),(1,0)) =>{
                                dd = *d; //Bottom Left
                                aa = *c; //Top Left
                                bb = *b; //Top Right
                                cc = *a; //Bottom Right
                            },
                            ((0,-1),(1,0)) =>{
                                dd = *a; //Bottom Left
                                aa = *b; //Top Left
                                bb = *c; //Top Right
                                cc = *d; //Bottom Right
                            },
                            ((0,1),(-1,0)) =>{
                                dd = *c; //Bottom Left
                                aa = *d; //Top Left
                                bb = *a; //Top Right
                                cc = *b; //Bottom Right
                            },
                            ((0,-1),(-1,0)) =>{
                                dd = *b; //Bottom Left
                                aa = *a; //Top Left
                                bb = *d; //Top Right
                                cc = *c; //Bottom Right
                            },
                            _ => panic!("Invalid Equality::Yes()"),
                        }
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
        obj.material_map = Some(finaltexture);
        //dbg!(&my_app.normals, &obj.vertices_normals);
        obj //return
    }
    ///[.obj and .mtl file specs][https://www.wikiwand.com/en/Wavefront_.obj_file#Physically-based_Rendering]
    ///
    ///
    fn write_mtl(&self){
        let is_vox = self.material_map.is_some();
        let w = if self.texture_map.is_some(){self.texture_map.clone().unwrap().w}else{self.material_map.clone().unwrap().w};
        let h = if self.texture_map.is_some(){self.texture_map.clone().unwrap().h}else{self.material_map.clone().unwrap().h};

        let transparency = if self.allowed_materials.1 &&is_vox{format!("\nTr 0.001\nmap_d -imfchain l {}.png",self.name)}
                                      else{"".to_owned()};
        let emission = if self.allowed_materials.2 &&is_vox{format!("\nmap_Ke {}_emit.png",self.name)
                                }else{"".to_owned()};
        let roughness = if self.allowed_materials.3 &&is_vox{format!("\nmap_Pr {}_extra.png -imfchain r",self.name)
                                }else{"".to_owned()};
        let metallic = if self.allowed_materials.4 && is_vox{format!("\nmap_Pm {}_extra.png -imfchain g",self.name)
                                }else{"".to_owned()};
        let specular = if self.allowed_materials.5 &&is_vox{format!("\nmap_Ns {}_extra.png -imfchain b",self.name)
                                }else{"".to_owned()};
        let ior = if self.allowed_materials.6 && is_vox{format!("\nmap_Ni {}_extra.png -imfchain l",self.name)
                                }else{"".to_owned()};

        let x = format!("#@DL2023 - w:{:?}, h:{:?}\nnewmtl x{}\nmap_Kd {}.png{}{}{}{}{}",
                                w,
                                h,
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
        let water = "#created with MagicaVoxel and VoxelOptimizer ";
        let mark = format!("v:{} - f:{}\n", self.number_of_v_and_f.0, self.number_of_v_and_f.1);
        let watermark = format!("{} - {}", water, mark);
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
                match (self.y_is_up, self.right_handed){
                    //godot
                    (true, true) => {
                        let result = writeln!(&mut obj_file,  "vn {} {} {}",
                        &self.vertices_normals[v].ny,
                        &self.vertices_normals[v].nz,
                        &self.vertices_normals[v].nx);
                        if result.is_err(){
                            panic!("Error while writing vertices normals: Error code 501");
                        }
                    },
                    //unity
                    (true, false) => {
                        let result = writeln!(&mut obj_file,  "vn {} {} {}",
                        &self.vertices_normals[v].ny,
                        &self.vertices_normals[v].nz,
                        &self.vertices_normals[v].nx);
                        if result.is_err(){
                         panic!("Error while writing vertices normals: Error code 501");
                        }

                    },
                    //Magicavoxel, blender
                    (false, true) => {
                        let result = writeln!(&mut obj_file,  "vn {} {} {}",
                        &self.vertices_normals[v].nx,
                        &self.vertices_normals[v].ny,
                        &self.vertices_normals[v].nz);
                        if result.is_err(){
                         panic!("Error while writing vertices normals: Error code 501");
                        }
                    },
                    //Unreal engine
                    (false, false) => {
                        let result = writeln!(&mut obj_file,  "vn {} {} {}",
                        &self.vertices_normals[v].ny,
                        &self.vertices_normals[v].nx,
                        &self.vertices_normals[v].nz);
                        if result.is_err(){
                         panic!("Error while writing vertices normals: Error code 501");
                        }
                    },
                }
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
            let mut xs = String::new();
            let mut ys = String::new();
            let mut zs = String::new();

            if x.fract() == 0.0 {xs = format!("{:.0}", x)}else{xs = format!("{}",x)}
            if y.fract() == 0.0 {ys = format!("{:.0}", y)}else{ys = format!("{}",y)}
            if z.fract() == 0.0 {zs = format!("{:.0}", z)}else{zs = format!("{}",z)}
            match (self.y_is_up, self.right_handed){
                //Godot
                (true, true) => {
                    let result = writeln!(&mut obj_file, "v {} {} {}",ys,zs,xs);
                    if result.is_err(){
                    panic!("Error while writing vertices: Error code 510");
                    }
                },
                //Unity
                (true, false) => {
                    let result = writeln!(&mut obj_file, "v {} {} {}",xs,zs,ys);
                    if result.is_err(){
                    panic!("Error while writing vertices: Error code 511");
                    }
                },
                //Magicavoxel, blender
                (false, true) => {
                    let result = writeln!(&mut obj_file, "v {} {} {}",xs,ys,zs);
                    if result.is_err(){
                    panic!("Error while writing vertices: Error code 512");
                    }
                },
                //Unreal engine
                (false, false) => {
                    let result = writeln!(&mut obj_file, "v {} {} {}",zs,ys,xs);
                    if result.is_err(){
                    panic!("Error while writing vertices: Error code 513");
                    }
                },
            }

        }

        //obj_file.write(list_of_v.as_bytes()).expect("write failed");
        //write vt
        let w = if self.texture_map.is_some(){self.texture_map.clone().unwrap().w}else{self.material_map.clone().unwrap().w};
        let h = if self.texture_map.is_some(){self.texture_map.clone().unwrap().h}else{self.material_map.clone().unwrap().h};

        let mut list_of_vt = String::new();
        if self.vt_precisionnumber == 0{
            if w == 1{
                self.vt_precisionnumber = 0;
            } else if w==2{
                self.vt_precisionnumber = 1;
            } else if w <= 4{
                self.vt_precisionnumber = 2;
            } else if w < 10{
                self.vt_precisionnumber = 3;
            } else if w < 100{
                self.vt_precisionnumber = 4;
            } else if w < 1000{
                self.vt_precisionnumber = 5;
            } else if w < 10000{
                self.vt_precisionnumber = 6;
            }else if w < 100000{
                self.vt_precisionnumber = 7;
            }
        }
        //println!("writing vt's with: {:?} digits", self.vt_precisionnumber);
        for vt in 0..self.vertices_uvs.len(){
            let x = self.vt_precisionnumber as usize;
            let u = format!("{:.*}", x, { self.vertices_uvs[vt].u as f32/w as f32 });
            let v = format!("{:.*}", x, 1.0-(self.vertices_uvs[vt].v as f32/h as f32));
            list_of_vt = format!("{}vt {u} {v}\n",list_of_vt);
        }
        obj_file.write_all(list_of_vt.as_bytes()).expect("write failed");
        //write usemtl
        obj_file.write_all(usemtl.as_bytes()).expect("write failed");
        //write faces
        if !self.vertices_normals.is_empty(){
            for f in 0..self.number_of_v_and_f.1{
                let face = &self.faces[f as usize];
                let result = writeln!(&mut obj_file, "f {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{}"
                ,face.a.0,face.a.1,face.a.2
                ,face.b.0,face.b.1,face.b.2
                ,face.c.0,face.c.1,face.c.2
                ,face.d.0,face.d.1,face.d.2);
                if result.is_err(){
                panic!("Error while writing faces: Error code 504");
                }
            }

        }else{
            for f in 0..self.number_of_v_and_f.1{
            let result = writeln!(&mut obj_file, "f {}/{} {}/{} {}/{} {}/{}"
                ,self.faces[f as usize].a.0,self.faces[f as usize].a.1
                ,self.faces[f as usize].b.0,self.faces[f as usize].b.1
                ,self.faces[f as usize].c.0,self.faces[f as usize].c.1
                ,self.faces[f as usize].d.0,self.faces[f as usize].d.1);
            if result.is_err(){
                panic!("Error while writing faces: Error code 505");
            }
            }

        }
    }
    fn write_png(&self){
        if self.material_map.is_some(){
        let map = self.material_map.clone().unwrap();
        let file = File::create(format!("{}/{}.png",self.export_folder,self.name)).unwrap();

        let file_e = if self.allowed_materials.2{Some(File::create(
            format!("{}/{}_emit.png",self.export_folder,self.name)).unwrap())
        }else{None};
        let file_o = File::create(format!("{}/{}_extra.png",self.export_folder,self.name)).unwrap();
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
            let mut encoder_o = png::Encoder::new(BufWriter::new(file_o),map.w as u32, map.h as u32);
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
            for y in 0..map.id.len(){
                for x in 0..map.id[y].len(){
                    let m = &map.materials[map.id[y][x] as usize];
                    data.push(m.rgb.r);
                    data.push(m.rgb.g);
                    data.push(m.rgb.b);
                    if m.rgb_e.is_some(){
                        data_e.push(m.rgb_e.unwrap().r);
                        data_e.push(m.rgb_e.unwrap().g);
                        data_e.push(m.rgb_e.unwrap().b);
                        data_o.push(0);
                        data_o.push(0);
                        data_o.push(0);
                        data_o.push(0);
                        data.push(255);
                    }else{
                        data.push((255.0-(m.transparent*255.0)) as u8);
                        data_e.push(0);
                        data_e.push(0);
                        data_e.push(0);
                        data_o.push((m.roughness*self.allowed_materials.3 as i32 as f32*255.0) as u8);
                        data_o.push((m.metallic*self.allowed_materials.4 as i32 as f32*255.0) as u8);
                        data_o.push((m.ior/2.0*self.allowed_materials.5 as i32 as f32*255.0) as u8);
                        data_o.push((m.specular*self.allowed_materials.6 as i32 as f32*255.0) as u8);
                    }
                }
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
            for y in 0..map.id.len(){
                for x in 0..map.id.len(){
                    let m = map.materials[map.id[y][x] as usize];
                    data.push(m.rgb.r);
                    data.push(m.rgb.g);
                    data.push(m.rgb.b);
                    if m.rgb_e.is_some(){
                        data_e.push(m.rgb_e.unwrap().r);
                        data_e.push(m.rgb_e.unwrap().g);
                        data_e.push(m.rgb_e.unwrap().b);
                        data_o.push(0);
                        data_o.push(0);
                        data_o.push(0);
                        data_o.push(0);
                        //data.push(255);
                    }else{
                        //data.push((255.0-(m.transparent*255.0)) as u8);
                        data_e.push(0);
                        data_e.push(0);
                        data_e.push(0);
                        data_o.push((m.roughness*self.allowed_materials.3 as i32 as f32*255.0) as u8);
                        data_o.push((m.metallic*self.allowed_materials.4 as i32 as f32*255.0) as u8);
                        data_o.push((m.ior/2.0*self.allowed_materials.5 as i32 as f32*255.0) as u8);
                        data_o.push((m.specular*self.allowed_materials.6 as i32 as f32*255.0) as u8);
                    }
                }
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
            let mut encoder = png::Encoder::new(w, self.texture_map.clone().unwrap().w as u32, self.texture_map.clone().unwrap().h as u32);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_compression(png::Compression::Best);
            let mut writer = encoder.write_header().unwrap();
            let mut data = Vec::new(); // An array containing an RGB sequence
            for y in 0..self.texture_map.clone().unwrap().colours.len(){
                    for x in 0..self.texture_map.clone().unwrap().colours[y].len(){
                        let p = self.texture_map.clone().unwrap().colours[y][x];
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
fn find_key_for_value<K, V>(map: HashMap<K, V>, value: V) -> Option<K> where V:PartialEq, V: Clone, K: Clone{
    map.iter()
        .find_map(|(key, val)| if *val == value { Some(key.clone()) } else { None })
}
