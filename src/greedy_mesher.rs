use std::path::PathBuf;
use std::vec;
use eframe::egui::panel::TopBottomSide::Top;
use crate::vox_importer::*;
use crate::texture_mapping::*;
use crate::uv_unwrapping::*;
use crate::{MyApp, vox_importer};
use ndarray::{Array3, Axis, s};
/*
END_PRODUCT

END________
INTERMEDIARY_PRODUCT
*/
#[derive(Debug, Copy, Clone)]
pub struct Cube{
    //0= top, 1= bottom, 2= left, 3= right, 4= front, 5= back
    faces: [Option<cube_f>;6], //(it was about to be outdated even before I uncommented this mess LOL DEATH_EMOJI)
    position: (f32, f32, f32),
    colour: (u8, u8, u8),
    vertices_indices: [i32;8],
    merged: bool
}
impl Cube{
    fn from_face(f: &cube_f) -> Cube{
        let m = false;
        let v = [0;8];
        let faces = match f.dir {
            DIRECTION::TOP =>    {[Some(f.clone()),None,None,None,None,None]}
            DIRECTION::BOTTOM => {[None,Some(f.clone()),None,None,None,None]}
            DIRECTION::LEFT =>   {[None,None,Some(f.clone()),None,None,None]}
            DIRECTION::RIGHT =>  {[None,None,None,Some(f.clone()),None,None]}
            DIRECTION::FRONT =>  {[None,None,None,None,Some(f.clone()),None]}
            DIRECTION::BACK =>   {[None,None,None,None,None,Some(f.clone())]}
        };
        let po = match f.dir{
            DIRECTION::TOP =>    {(f.position.0,f.position.1, f.position.2 - 0.5)}
            DIRECTION::BOTTOM => {(f.position.0,f.position.1, f.position.2 + 0.5)}
            DIRECTION::LEFT =>   {(f.position.0 + 0.5,f.position.1, f.position.2)}
            DIRECTION::RIGHT =>  {(f.position.0 - 0.5,f.position.1, f.position.2)}
            DIRECTION::FRONT =>  {(f.position.0,f.position.1 + 0.5, f.position.2)}
            DIRECTION::BACK =>   {(f.position.0,f.position.1 - 0.5, f.position.2)}
        };
        Cube{
            position: po,
            faces: faces,
            vertices_indices: v,
            colour: f.colour,
            merged: m,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct cube_f{
    position: (f32, f32, f32),
    dir: DIRECTION,
    colour: (u8,u8,u8),
    //vertices_indices: [i32;4]
}
#[derive(Copy, Clone, Debug)]
pub enum DIRECTION{
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    FRONT,
    BACK
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
impl cube_f {
    fn from_vertices(a: &v, b: &v, c: &v, d: &v) -> cube_f {
        let po = ((a.x + b.x + c.x + d.x) / 4.0,
                  (a.y + b.y + c.y + d.y) / 4.0,
                  (a.z + b.z + c.z + d.z) / 4.0);

        let di: DIRECTION = if a.x != b.x {
            if b.y != c.y {
                DIRECTION::TOP
            } else if b.z != c.z {
                DIRECTION::FRONT
            } else { DIRECTION::TOP }
        } else if a.y != b.y {
            if b.x != c.x {
                DIRECTION::BOTTOM
            } else if b.z != c.z {
                DIRECTION::RIGHT
            } else { DIRECTION::TOP }
        } else if a.z != b.z {
            if b.x != c.x {
                DIRECTION::BACK
            } else if b.y != c.y {
                DIRECTION::LEFT
            } else { DIRECTION::TOP }}else { DIRECTION::TOP };

        let col = (a.r, a.g, a.b);

        cube_f {
            position: po,
            dir: di,
            colour: col,
            //vertices_indices:
        }
    }
    fn return_cube_position(&self) -> (f32, f32, f32) {
        let po = match self.dir {
            DIRECTION::TOP =>    {(self.position.0,self.position.1, self.position.2 - 0.5)}
            DIRECTION::BOTTOM => {(self.position.0,self.position.1, self.position.2 + 0.5)}
            DIRECTION::LEFT =>   {(self.position.0 + 0.5,self.position.1, self.position.2)}
            DIRECTION::RIGHT =>  {(self.position.0 - 0.5,self.position.1, self.position.2)}
            DIRECTION::FRONT =>  {(self.position.0,self.position.1 + 0.5, self.position.2)}
            DIRECTION::BACK =>   {(self.position.0,self.position.1 - 0.5, self.position.2)}
        };
        return ((po.0),(po.1),(po.2));
    }
}
pub struct OptimizedCube{
    //___________w_|_h_|_d_|_
    dimensions: (u8, u8, u8),

    //used to evaluate the texture map of each face
    cubes: Vec<Cube>,
    //monochrome: bool,
    //-------------------indices----0 bottom left, 1 bottom right, 2 top right, 3 top left, 4-7 same thing but up and clockwise
    //important_vertices: [i32; 8]
    starting_position: (i32,i32,i32)

}
//*/
//pub(crate) fn convert(my_app: &mut MyApp, path: &std::path::PathBuf, monochrome: &bool, pattern_matching: &bool, is_texturesize_powerof2: &bool, texturemapping_invisiblefaces: &bool, manual_vt: &bool, vt_precisionnumber: &u8, background_color: [f32;3], debug_uv_mode: bool){
pub(crate) fn convert(my_app: &mut MyApp, path: PathBuf){
    let x= format!("{}{}",String::from("converting:"), path.to_string_lossy().to_string());
    my_app.sx.send(x);
    my_app.status = String::from("Reading...");
    let content = read_ply(&path.to_string_lossy().to_string());
    let mut ply_result:Result<ply, vox_importer::vox_importer_errors> = match content {
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
    if let Ok(ply) = &ply_result {
        let x = String::from(format!("Optimizing model with {} vertices and {} faces", &ply.number_of_v_and_f.0, &ply.number_of_v_and_f.1));
        my_app.sx.send(x);

        println!("{:?}", &ply);
    }
    if let Err(e) = &ply_result {
        let x = String::from(format!("Error while parsing!!! {}" ,e));
        my_app.sx.send(x);
        println!("{}", e);
    }
    //1. normalize the vertexes positions
    //
    // 1.3  multiply by 10
    //
    // 1.5 output as one big vector of cube_v
    //2. make a mut [[[Option<cube>;256];256];256]
    //3. insert each face from the ply to the correct cube in the array and derive it's direction and color
    //4. Reduce vertex count
    // 4.1 take the big cube_v vector and remove vertex duplicates (with dedup? (that would include sorting which chatgpt helped me with) )
    // 4.2    https://stackoverflow.com/questions/57641712/is-there-an-efficient-function-in-rust-that-finds-the-index-of-the-first-occurre
    //5. Insert the eight significant vertices in each cube
    //6. greedy meshing algorithm with output being a Vec! of optimized cubes
    //7. create a new Vec! of vertices with the only vertices being the important vertices
    // 7.1 this new vector shall have no duplicates
    //7 alt, create an hashmap with index and value like so        k, v
    //----------------------------------------------------(u8,u8,u8);i32
    //8. for each face of each optimized cube return a texture map (like so: Vec!<(rgb)>
    // 8.1 let textures = Vec!<Vec!(rgb)>
    // 8.2 remove duplicates and assign each texture
    let mut ply = ply_result.unwrap();
    ply = ply.normalize_positions();
    println!("{:?}",&ply);

    let mut vector_of_f: Vec<cube_f> = Vec::new();
    let mut lowest_coordinates = (9000.0,9000.0,9000.0);
    let mut highest_coordinates = (-9000.0, -9000.0, -9000.0);
    for f in &ply.faces{
        let a: &v = &ply.vertices[f.vs.0 as usize];
        let b: &v = &ply.vertices[f.vs.1 as usize];
        let c: &v = &ply.vertices[f.vs.2 as usize];
        let d: &v = &ply.vertices[f.vs.3 as usize];
        let fa = cube_f::from_vertices(a, b, c, d);
        vector_of_f.push(fa);

        //x
        if fa.position.0<lowest_coordinates.0{
            lowest_coordinates.0=fa.position.0
        } else if fa.position.0>highest_coordinates.0{highest_coordinates.0=fa.position.0}
        //y
        if fa.position.1<lowest_coordinates.1{
            lowest_coordinates.1=fa.position.1
        } else if fa.position.1>highest_coordinates.1{highest_coordinates.1=fa.position.1}
        //z
        if fa.position.2<lowest_coordinates.2{
            lowest_coordinates.2=fa.position.2
        } else if fa.position.2>highest_coordinates.2{highest_coordinates.2=fa.position.2}

    }
    let mut cubes =  Array3::<Option<Cube>>::from_elem(
                                     ((highest_coordinates.0 - lowest_coordinates.0) as usize,
                                        (highest_coordinates.1 - lowest_coordinates.1) as usize,
                                        (highest_coordinates.2 - lowest_coordinates.2)as usize), None);
    println!("{:?}", &lowest_coordinates);
    println!("{:?}", &highest_coordinates);

    for fa in &vector_of_f{
        //let index = ((fa.return_cube_position().0-0.5) - lowest_coordinates.0 as u8, fa.return_cube_position()-0.5)as u8, fa.return_cube_position()-0.5)as u8)
        println!("x:{:?},y:{:?},z:{:?}",fa.return_cube_position().0 - lowest_coordinates.0 - 0.5,
                    fa.return_cube_position().1 - lowest_coordinates.1 - 0.5,
                    fa.return_cube_position().2 - lowest_coordinates.2 - 0.5);
        let index = (((fa.return_cube_position().0 - lowest_coordinates.0 - 0.5) as usize),
                            ((fa.return_cube_position().1 - lowest_coordinates.1 - 0.5) as usize),
                            ((fa.return_cube_position().2 - lowest_coordinates.2 - 0.5) as usize));
        //println!("{:?}", &index);

        //if cubes[index.0][index.1][index.2].is_some(){
        if let Some(mut cube) = cubes[[index.0, index.1, index.2]].take(){
            let i = match fa.dir {
                DIRECTION::TOP => {0}
                DIRECTION::BOTTOM => {1}
                DIRECTION::LEFT => {2}
                DIRECTION::RIGHT => {3}
                DIRECTION::FRONT => {4}
                DIRECTION::BACK => {5}
            };
            cube.faces[i] = Some(*fa);
            cubes[[index.0, index.1, index.2]] = Some(cube);
            //println!("{:?}", &fa);

        } else {
            let mut cu = Cube::from_face(fa);
            //println!("{:?}", &cu);
            cubes[[index.0, index.1, index.2]] = Some(cu);
            //cubes[index.0][index.1][index.2] = Some(cu);
        }
    }
    //println!("{:?}", &vector_of_f);
    println!("{:?}", &cubes);
    //let ply: ply = parse_ply(&content);
}

pub fn convert_to_optimized_cubes(cubes: Array3<Option<Cube>>, cross: bool) -> Vec<OptimizedCube>{
    let mut cs = Vec::new();
    /*
    OptimizedCube{
        dimensions: (0, 0, 0),
        cubes: vec![],
        starting_position: (0,0,0),
    }
     */
    for z in cubes.len_of(Axis(2usize)){
        for y in cubes.len_of(Axis(1usize)){
            for x in cubes.len_of(Axis(0usize)) {
                if let Some(opcube) =find_dimensions(cubes.raw_dim(), (x as u8, y as u8, z as u8), &cubes, &cross) {
                    cs.push(opcube);
                }
            }
        }
    }
    cs
}
fn find_dimensions(sh: ndarray::Ix3, index_we_are_at: (u8,u8,u8), cs: &Array3<Option<Cube>>, cross_optimization: &bool) -> Option<OptimizedCube>{

    let shape = (sh[0], sh[1], sh[2]);
    let mut dimensions = (1, 1, 1);
    let mut cubes = Vec::new();
    let mut con = true;
    let mut first = true;

    //x scouting
    while con {
        if (index_we_are_at.0 as usize + dimensions.0 as usize) <= shape.0 {
            if let Some(mut cube) = cs[[index_we_are_at.0 + dimensions.0, index_we_are_at.1, index_we_are_at.2]] {
                if cross_optimization || !&cube.merged {
                    dimensions.0 += 1;
                    //let mut cube = cs[index_we_are_at + dimensions.x].unwrap();
                    cube.merged = true;
                    cubes.push(cube);
                    first = false;
                } else {
                    con = false;
                }
            }
        }
    }

    //y scouting
    con = true;
    first = true;

    while con{
        if (index_we_are_at.1 as usize + dimensions.1 as usize) <= shape.0 {
            let slice = cs.slice[ndarray::s![index_we_are_at.0..index_we_are_at.0+dimensions.1,index_we_are_at.1..index_we_are_at.1+dimensions.1, index_we_are_at.2..index_we_are_at.2+1]];
            if is_slice_some(slice) {
                //is next slice some
                let last = !(is_slice_some(cs.slice[ndarray::s![index_we_are_at.0..index_we_are_at.0+dimensions.1,
                    index_we_are_at.1+1..index_we_are_at.1+dimensions.1+1,
                    index_we_are_at.2..index_we_are_at.2+1]]));
                if can_slice_be_merged(slice, &first, &last, &cross_optimization) {
                    for x in index_we_are_at.0..index_we_are_at.0+dimensions.0{
                        if let Some(mut cube) = cs[[x, index_we_are_at.1 + dimensions.1, index_we_are_at.2]]{
                            dimensions.1 += 1;
                            cube.merged = true;
                            cubes.push(cube);
                            first = false;
                        }
                    }
                }else{con = false}
            }else{con = false}
        }else{con = false}

    //if cs.slice[ndarray::s![starting_x..ending_x,index_we_are_at.y..index_we_are_at.y+dimensions.1, index_we_are_at.z..index_we_are_at.z+1]].is_some() && (cross_optimization || !cs.merged){}

    }
    //z scouting
    con = true;
    while con{

    }
    //optimized cube forming
    let starting_position = index_we_are_at;
    Some(OptimizedCube{
        dimensions: dimensions,
        starting_position: (index_we_are_at.0 as i32, index_we_are_at.1 as i32, index_we_are_at.2 as i32),
        cubes: cubes,

    })
}
fn is_slice_some(slice: Array3<Option<Cube>>) -> bool {todo!();}
fn can_slice_be_merged(slice: Array3<Option<Cube>>, first: &bool, last: &bool, cross: &bool) -> bool{todo!();}

//https://stackoverflow.com/questions/63752622/is-there-a-simple-way-to-find-out-whether-a-vector-is-filled-with-none-in-rust
//do this for some and you are alright