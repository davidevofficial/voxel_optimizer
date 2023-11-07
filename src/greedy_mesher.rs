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
#[derive(Debug)]
pub struct OptimizedCube{
    //___________w_|_h_|_d_|_
    dimensions: (u16, u16, u16),

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

    //normalize positions
    let mut ply = ply_result.unwrap();
    ply = ply.normalize_positions();
    println!("{:?}",&ply);

    //get size
    let mut vector_of_f: Vec<cube_f> = Vec::new();
    let mut lowest_coordinates = (9999.0,9999.0,9999.0);
    let mut highest_coordinates = (-9999.0, -9999.0, -9999.0);
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

    /*
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
    //convert_to_optimized_cubes(cubes,&cross)
    */

    //todo()! -> lowestcoordinates unsafe
    let mut mapofcubes: MapOfCubes = MapOfCubes{Hashmap:HashMap::new(), Shape:(1,1,1),
     Lowest_coordinates:(lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32)};

    mapofcubes.set_shape((highest_coordinates.0 - lowest_coordinates.0) as i32,
                        (highest_coordinates.1 - lowest_coordinates.1) as i32,
                        (highest_coordinates.2 - lowest_coordinates.2)as i32);

    for fa in &vector_of_f{
        let index = (   ((fa.return_cube_position().0 -  0.5) as i32),
                        ((fa.return_cube_position().1  - 0.5) as i32),
                        ((fa.return_cube_position().2  - 0.5) as i32)   );

        if let Some(mut cube) = mapofcubes.get_cube(index.0, index.1, index.2){
            let i = match fa.dir {
                DIRECTION::TOP => {0}
                DIRECTION::BOTTOM => {1}
                DIRECTION::LEFT => {2}
                DIRECTION::RIGHT => {3}
                DIRECTION::FRONT => {4}
                DIRECTION::BACK => {5}
            };
            cube.faces[i] = Some(*fa);
            mapofcubes.set_cube(index.0, index.1, index.2, cube);
            //println!("{:?}", &fa);

        } else {
            let mut cu = Cube::from_face(fa);
            mapofcubes.set_cube(index.0, index.1, index.2, cu);
        }
    }
    println!("{:?}", &mapofcubes);
    let optimized_cubes = convert_to_optimized_cubes(&mut mapofcubes, my_app.cross,
     (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32));
    let x = String::from(format!("Reduced model voxels count from {:?} to {:?} voxels", &ply.number_of_v_and_f.0/8, optimized_cubes.len()));
        my_app.sx.send(x);
    
    println!("{:?}", optimized_cubes.len());

}

use std::collections::HashMap;
#[derive(Debug)]
pub struct MapOfCubes {
    Hashmap: HashMap<(i32, i32, i32),Cube>,
    Shape: (i32, i32, i32),
    Lowest_coordinates: (i32,i32,i32),
}
impl MapOfCubes{

    fn set_cube(&mut self, x:i32, y:i32, z:i32, cube:Cube){
        self.Hashmap.insert((x,y,z),cube);
    }

    fn get_cube(&self, x:i32, y:i32, z:i32)->Option<Cube>{
        self.Hashmap.get(&(x,y,z)).copied()
    }

    fn set_shape(&mut self, x:i32, y:i32, z:i32){
        self.Shape = (x, y, z);
    }

    fn is_slice_some(&self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> bool {
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    if self.Hashmap.get(&(x,y,z)).is_none(){
                        return false;
                    }
                } 
            }
        }
        return true;
    }

    fn can_slice_be_merged(&self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> CanBeMerged {
        
        let mut is_slice_already_merged: bool = false;
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                let cube = self.Hashmap.get(&(x,y,z));
                match cube{
                    None => {return CanBeMerged::No;}
                    Some(x) => {if x.merged == true{is_slice_already_merged = true;}}
                    }
                } 
            }
        }
        
        if is_slice_already_merged{
            return CanBeMerged::Cross;
        }
        return CanBeMerged::Yes;
    }

    fn merge_slice(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32){
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    if let Some(entry) = self.Hashmap.get_mut(&(x,y,z)) {
                        entry.merged = true;
                    }
                } 
            }
        }
    }

    fn get_cubes_from_slice(&self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> Vec<Cube>{
        let mut vector_to_return: Vec<Cube> = Vec::new();
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    let cube = self.Hashmap.get(&(x,y,z));
                    match cube{
                    None => {println!("x1:{:?}, x2:{:?}, y1:{:?}, y2:{:?}, z1:{:?}, z2:{:?}, ",x1,x2,y1,y2,z1,z2 );
                            println!("x:{:?} y:{:?} z:{:?} ", x,y,z);
                        unimplemented!()}
                    Some(x) => {vector_to_return.push(*x);}
                    }
                } 
            }
        }
        return vector_to_return;
    }

}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum CanBeMerged{
    Yes,
    No,
    Cross,
}

pub fn convert_to_optimized_cubes(cubes: &mut MapOfCubes, cross: bool, lowest_coordinates:(i32, i32, i32)) -> Vec<OptimizedCube>{
    let mut cs = Vec::new();
    /*
    OptimizedCube{
        dimensions: (0, 0, 0),
        cubes: vec![],
        starting_position: (0,0,0),
    }
     */
    println!("{:?}", lowest_coordinates);
    println!("{:?}", cubes.Shape);
    for z in lowest_coordinates.2..cubes.Shape.2+lowest_coordinates.2{
        for y in lowest_coordinates.1..cubes.Shape.1+lowest_coordinates.1{
            for x in lowest_coordinates.0..cubes.Shape.0+lowest_coordinates.0{
                println!("x:{:?} y:{:?} z:{:?}", x as i32,y as i32,z as i32);
                if let Some(opcube) = find_dimensions(cubes, (x as i32, y as i32, z as i32), &cross) {

                    cs.push(opcube);
                }
            }
        }
    }
    cs
}


fn find_dimensions(mymap: &mut MapOfCubes, index_we_are_at: (i32,i32,i32), cross_optimization: &bool) -> Option<OptimizedCube>{

    let mut shape = (1, 1, 1);
    println!("{:?}", shape);
    //let mut dimensions = (1u8, 1u8, 1u8);
    let mut cubes = Vec::new();


    //is the first cube a some value?
    if mymap.is_slice_some(index_we_are_at.0 as i32, index_we_are_at.0 as i32,
                             index_we_are_at.1 as i32, index_we_are_at.1 as i32,
                              index_we_are_at.2 as i32, index_we_are_at.2 as i32){
        //can it be merged?
        match mymap.can_slice_be_merged(index_we_are_at.0 as i32, index_we_are_at.0 as i32,
                             index_we_are_at.1 as i32, index_we_are_at.1 as i32,
                              index_we_are_at.2 as i32, index_we_are_at.2 as i32){
            CanBeMerged::No => {return None;}
            CanBeMerged::Cross => {return None;}
            CanBeMerged::Yes => {
                //if so it will be the first cube of the vector
                if let Some(x) = mymap.get_cube(index_we_are_at.0 as i32, index_we_are_at.1 as i32, index_we_are_at.2 as i32) {
                    cubes.push(x);
                    mymap.merge_slice(index_we_are_at.0, index_we_are_at.0, index_we_are_at.1, index_we_are_at.1,
                        index_we_are_at.2, index_we_are_at.2);
                }
                
            }
        }

    } else {return None;}
    //todo: implement a cache function (a vector of possible values
    //that answers the question can it be merged? (Yes, No, Cross (already been merged)))
    //like so: Yes, cross, cross, Yes
    //or: Yes, no -> you therefore stop
    //or: Yes, cross, cross, No -> you evaluate that the third is the last but being a Cross it cannot be last
    //so it asks the second one, can you be last? and he is a cross too so it becomes Yes, and that is it
    println!("{:?}", "has it crashed yet? 1");
    println!("{:?}", shape);

    let mut v_cached = Vec::new();
    //x
    let i = index_we_are_at.0.clone();
    let j = index_we_are_at.1.clone();
    let k = index_we_are_at.2.clone();
    println!("{:?}", "has it crashed yet? 1.1");
    println!("i:{:?} j:{:?} k:{:?}", i, j, k);
    println!("{:?}", mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k));
    while (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k) == CanBeMerged::Yes) ||
            (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k) == CanBeMerged::Cross && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k));
                shape.0 += 1;
            }
    println!("{:?}", "has it crashed yet? 1.2");
    println!("{:?}", shape);
    //todo()! -> v_cached gets sanitized (based on the cross rule) and then I put cubes in the optimized cubes based
    //           on vector lenght
    println!("{:?}", v_cached);
    v_cached = cache_sanitization( v_cached, *cross_optimization);
    println!("{:?}", v_cached);
    shape.0 = v_cached.len() as i32 + 1 ;
    if shape.0 > 1{

        for v in mymap.get_cubes_from_slice(i+1, i+shape.0-1, j, j, k, k){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j, k, k)
    }
    println!("{:?}", "has it crashed yet? 1.3");
    println!("{:?}", shape);
    //y
    v_cached = Vec::new();
    println!("{:?}", mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k));
    println!("{:?}", (i, i+shape.0, j+shape.1, j+shape.1, k, k));
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k));
                shape.1 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);
    shape.1 = v_cached.len() as i32  + 1;
    if shape.1 > 1{
        for v in mymap.get_cubes_from_slice(i, i+shape.0-1, j+1, j+shape.1-1, k, k){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k)
    }
    println!("{:?}", "has it crashed yet? 1.4");
    println!("shape:{:?}, ijk:{:?}", (shape),(i,j,k));
    //z
    v_cached = Vec::new();
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2));
                shape.2 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);
    shape.2 = v_cached.len() as i32  + 1;
    if shape.2 > 1{
        for v in mymap.get_cubes_from_slice(i, i+shape.0-1, j, j+shape.1-1, k+1, k+shape.2-1){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k+shape.2-1)
    }
    println!("{:?}", "has it crashed yet? 2");
    //optimized cube forming
    //let starting_position = index_we_are_at;
    println!("{:?}", shape);
    Some(OptimizedCube{
        dimensions: (shape.0 as u16, shape.1 as u16, shape.2 as u16),
        starting_position: (index_we_are_at.0 as i32, index_we_are_at.1 as i32, index_we_are_at.2 as i32),
        cubes: cubes,

    })
}

//todo!();
fn cache_sanitization(mut v_cached: Vec<CanBeMerged>, cross: bool) -> Vec<CanBeMerged>{
    if v_cached.len() != 0{
    if cross {
        while v_cached.len() != 0 && v_cached[v_cached.len()-1] == CanBeMerged::Cross{
            v_cached.pop();
        }
        return v_cached;

    } else {
        let mut is_not_mergeable = false;
        let mut i = 0;
        while v_cached.len() != 0 && (i <= v_cached.len()-1) && is_not_mergeable == false{
            if v_cached[i] == CanBeMerged::Yes{
                i += 1;
            }else{
                is_not_mergeable = true;
            }
        }
        while i <= v_cached.len() -1 {
            v_cached.pop();
        }
        return v_cached;
    }
    }else {
        Vec::new()
    }

}
