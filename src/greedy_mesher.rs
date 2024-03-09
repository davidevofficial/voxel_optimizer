use std::path::PathBuf;
use std::vec;
use std::collections::HashSet;

use crate::vox_importer::*;
use crate::vox_exporter::*;
use crate::vox_exporter;

use crate::{MyApp, vox_importer};


/*
END_PRODUCT

END________
INTERMEDIARY_PRODUCT
*/
#[derive(Debug, Clone, Default)]
pub struct ColourMatrix{
    matrixc: Vec<Vec<Vec<Option<(u8,u8,u8)>>>>,
    matrixb: Vec<Vec<Vec<Option<bool>>>>,
    shape: (i32, i32, i32),
    lowest_coordinates: (i32,i32,i32),
}
impl ColourMatrix{
    fn def()->ColourMatrix{
        ColourMatrix{
            matrixc: Vec::new(),
            matrixb: Vec::new(),
            shape:(0,0,0),
            lowest_coordinates:(0,0,0),
        }
    }
    fn set_size(&mut self, shapex: i32, shapey: i32, shapez: i32){
        self.shape = (shapex, shapey, shapez);
        for z in 0..shapez{
            self.matrixc.push(Vec::new());
            self.matrixb.push(Vec::new());
            for y in 0..shapey{
                self.matrixc[z as usize].push(Vec::new());
                self.matrixb[z as usize].push(Vec::new());
                for x in 0..shapex{
                    self.matrixc[z as usize][y as usize].push(None);
                    self.matrixb[z as usize][y as usize].push(None);
                }
            }
        }
        /*
        for _ in 0..(shapex*shapey*shapez)as usize{
            self.matrixc.push(None);
            self.matrixb.push(None);
        }
        */
    }
    fn pos_to_index(&self, x:i32, y:i32, z:i32)->(usize,usize,usize){
        let xx = x-self.lowest_coordinates.0;
        let yy = y-self.lowest_coordinates.1;
        let zz = z-self.lowest_coordinates.2;
        return (xx as usize,yy as usize,zz as usize);
    }
    /*
    fn index_to_pos(&self, x:i32, y:i32, z:i32)->(i32,i32,i32){
        let xx = x+self.lowest_coordinates.0;
        let yy = y+self.lowest_coordinates.1;
        let zz = z+self.lowest_coordinates.2;
        return (xx,yy,zz);
    }
    */
    fn vector_to_scalar_index(&mut self, x:i32, y:i32, z:i32)->usize{
        let (xx,yy,zz) = self.pos_to_index(x,y,z);
        return (self.shape.0*self.shape.1*(zz as i32)+self.shape.0*(yy as i32)+(xx as i32)) as usize;
    }
    fn get_cube_colour(&mut self, x: i32, y:i32, z: i32)->Option<(u8,u8,u8)>{
        /*
        if x < self.shape.0 && y < self.shape.1 && z < self.shape.2{
        let i = self.vector_to_scalar_index(x,y,z);
        if i >= self.matrixb.len(){
            return None;
        }
        return self.matrixc[i]; }else {
            return None;
        }
        */
        let (xx,yy,zz) = self.pos_to_index(x, y, z);
        if let Some(z) = self.matrixc.get(zz) {
            if let Some(y) = z.get(yy) {
                if let Some(x) = y.get(xx) {
                    return self.matrixc[zz][yy][xx];
                }
            }
        }
        return None;
        //return self.matrixc.get(zz)?.get(yy)?.get(xx)?;
    }
    fn get_cube_bool(&mut self, x: i32, y:i32, z: i32)->Option<bool>{
        /*
        if x < self.shape.0 && y < self.shape.1 && z < self.shape.2{
        let i = self.vector_to_scalar_index(x,y,z);
        if i >= self.matrixb.len(){
            return None;
        }
        return self.matrixb[i]; }else {
            return None;
        }
        */
        let (xx,yy,zz) = self.pos_to_index(x, y, z);
        if let Some(z) = self.matrixb.get(zz) {
            if let Some(y) = z.get(yy) {
                if let Some(x) = y.get(xx) {
                    return self.matrixb[zz][yy][xx];
                }
            }
        }
        return None;
    }
    fn set_cube_colour(&mut self, i:(i32,i32,i32), rgb:(u8,u8,u8)){
        /*
        let ii = self.vector_to_scalar_index(i.0,i.1,i.2);
        if ii >= self.matrixc.len() {println!("{:?}{:?}{:?}{:?}",self.shape,self.lowest_coordinates, i, rgb);}
        self.matrixc[ii]=Some((rgb.0,rgb.1,rgb.2));
        */
        let (xx,yy,zz) = self.pos_to_index(i.0, i.1, i.2);
        self.matrixc[zz][yy][xx]=Some((rgb.0,rgb.1,rgb.2));
    }
    fn set_cube_bool(&mut self, i:(i32,i32,i32), b:bool){
        /*
        let ii = self.vector_to_scalar_index(i.0,i.1,i.2);
        self.matrixb[ii]=Some(b);
        */
        let (xx,yy,zz) = self.pos_to_index(i.0, i.1, i.2);
        self.matrixb[zz][yy][xx]=Some(b);
    }
    fn can_slice_be_merged(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> CanBeMerged {
        
        let mut is_slice_already_merged: bool = false;
        let mut is_all_not_merged:bool=false;
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                let cube = self.get_cube_bool(x,y,z);
                //println!("self.Hashmap.get({:?},{:?},{:?};)={:?}",x,y,z,self.Hashmap.get(&(x,y,z)));
                match cube{
                    None => {return CanBeMerged::No;}
                    Some(w) => {if w == true{
                                if !is_all_not_merged{is_slice_already_merged = true;
                                }else{is_slice_already_merged=false}
                                }else{is_all_not_merged=true;}}
                    }
                } 
            }
        }
        
        if is_slice_already_merged{
            return CanBeMerged::Cross;
        }
        return CanBeMerged::Yes;
    }
    fn is_slice_some(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> bool {
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    if self.get_cube_bool(x,y,z).is_none(){
                        return false;
                    }
                } 
            }
        }
        return true;
    }
    fn merge_slice(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32){
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    self.set_cube_bool((x,y,z),true);
                } 
            }
        }
    }
    fn get_texturemap(&mut self, i: i32, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> vox_exporter::TextureMap{
        let mut vector_of_colours: Vec<Option<vox_exporter::Rgb>> = Vec::new();
        let mut w = 1;
        let mut h = 1;
        //top
        if i == 0{
            w = x2-x1;
            h = y2-y1;
            for y in (y1..y2).rev(){
                for x in x1..x2{   
                        let rgb = self.get_cube_colour(x,y,z2-1);                
                        if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //bottom//
        if i == 1{
            w = x2-x1;
            h = y2-y1;
            for y in (y1..y2){
                for x in (x1..x2){
                    //let rgb = self.get_cube_colour(x2-1-x,y2-1-y,z1);
                    let rgb = self.get_cube_colour(x,y,z1);
                    if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //left//
        if i == 2{
            w = y2-y1;
            h = z2-z1;
            for z in (z1..z2).rev(){
                for y in (y1..y2).rev(){
                    let rgb = self.get_cube_colour(x1,y,z);
                    if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //right
        if i == 3{
            w = y2-y1;
            h = z2-z1;
            for z in (z1..z2).rev(){
                for y in (y1..y2){
                    let rgb = self.get_cube_colour(x2-1,y,z);
                    if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //front
        if i == 4{
            w = x2-x1;
            h = z2-z1;
            for z in (z1..z2).rev(){
                for x in x1..x2{
                    let rgb = self.get_cube_colour(x,y1,z);
                    if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //back//
        if i == 5{
            w = x2-x1;
            h = z2-z1;
            for z in (z1..z2).rev(){
                for x in (x1..x2).rev(){
                    let rgb = self.get_cube_colour(x,y2-1,z);
                    if let None = rgb{vector_of_colours.push(None);}else if let Some(c) = rgb{
                        vector_of_colours.push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        vox_exporter::TextureMap{w:w as usize,h:h as usize, colours:vector_of_colours}
    }
}
pub struct BoolMatrix{
    shape:(i32,i32,i32),
    lowest_coordinates:(i32,i32,i32),
    matrixb: Vec<Vec<Vec<bool>>>,
}
impl BoolMatrix{
    fn from_size(shapex:i32, shapey:i32, shapez:i32, lc:(i32,i32,i32))->BoolMatrix{
        let shape = (shapex, shapey, shapez);
        let mut v = Vec::new();
        for z in 0..shapez{
            v.push(Vec::new());
            for y in 0..shapey{
                v[z as usize].push(Vec::new());
                for x in 0..shapex{
                    v[z as usize][y as usize].push(false);
                }
            }
        }
        BoolMatrix{shape:shape, matrixb:v, lowest_coordinates:lc}
    }
    fn set_size(&mut self, shapex: i32, shapey: i32, shapez: i32){
        self.shape = (shapex, shapey, shapez);
        for z in 0..shapez{
            self.matrixb.push(Vec::new());
            for y in 0..shapey{
                self.matrixb[z as usize].push(Vec::new());
                for x in 0..shapex{
                    self.matrixb[z as usize][y as usize].push(false);
                }
            }
        }
    }
    fn pos_to_index(&self, x:i32, y:i32, z:i32)->(usize,usize,usize){
        let xx = x-self.lowest_coordinates.0;
        let yy = y-self.lowest_coordinates.1;
        let zz = z-self.lowest_coordinates.2;
        return (xx as usize,yy as usize,zz as usize);
    }
    /*
    fn vector_to_scalar_index(&mut self, x:i32, y:i32, z:i32)->usize{
        let (xx,yy,zz) = self.pos_to_index(x,y,z);
        return (self.shape.0*self.shape.1*zz+self.shape.0*yy+xx) as usize;
    }
    */
    fn get_cube_bool(&mut self, x: i32, y:i32, z: i32)->bool{
        /*

        */
        let (xx,yy,zz) = self.pos_to_index(x,y,z);
        if let Some(zw) = self.matrixb.get(zz) {
            if let Some(yw) = zw.get(yy) {
                if let Some(x) = yw.get(xx) {
                    return self.matrixb[zz][yy][xx];
                }
            }
        }
        return false;
    }
    fn set_cube_bool(&mut self, i:(i32,i32,i32), b:bool){
        let ii = self.pos_to_index(i.0,i.1,i.2);
        self.matrixb[ii.2 as usize][ii.1 as usize][ii.0 as usize]=b;
    }
    fn contains(&mut self, x:usize, y:usize, z:usize)->bool{
        return self.matrixb[z][y][x] == true;
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Cube{
    //0= top, 1= bottom, 2= left, 3= right, 4= front, 5= back
    faces: [Option<cube_f>;6], //(it was about to be outdated even before I uncommented this mess LOL DEATH_EMOJI)
    position: (f32, f32, f32),
    colour: (u8, u8, u8),
    merged: bool
}
impl Cube{
    fn from_face(f: &cube_f) -> Cube{
        let m = false;
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
            colour: f.colour,
            merged: m,
        }
    }
    fn from_nothing(position: (f32, f32, f32))-> Cube{
        Cube{
            position: position,
            faces: [None,None,None,None,None,None],
            colour: (0,0,0),
            merged: false,
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
        
        let mut di: DIRECTION = DIRECTION::TOP;
        //either top or bottom
        if a.z == b.z && b.z == c.z && c.z == d.z{
            let top_left = (po.0-0.5, po.1+0.5, po.2);
            let top_right = (po.0 + 0.5, po.1 +0.5, po.2);
            let bottom_left = (po.0 - 0.5, po.1 - 0.5, po.2);
            let bottom_right = (po.0 + 0.5, po.1 - 0.5, po.2);
            //top
            if  (a.x, a.y, a.z) == top_left && (b.x, b.y, b.z) == bottom_left ||
                (a.x, a.y, a.z) == bottom_left && (b.x, b.y, b.z) == bottom_right||
                (a.x, a.y, a.z) == bottom_right && (b.x, b.y, b.z) == top_right ||
                (a.x, a.y, a.z) == top_right && (b.x, b.y, b.z) == top_left{di = DIRECTION::TOP;}
            //not top 
            else{
                di = DIRECTION::BOTTOM;
            }
        }
        //either front or back
        if a.y == b.y && b.y == c.y && c.y == d.y{
            let top_left = (po.0-0.5, po.1, po.2 + 0.5);
            let top_right = (po.0 + 0.5, po.1, po.2 + 0.5);
            let bottom_left = (po.0 - 0.5, po.1, po.2 -0.5);
            let bottom_right = (po.0 + 0.5, po.1, po.2 -0.5);
            //front
            if  (a.x, a.y, a.z) == top_left && (b.x, b.y, b.z) == bottom_left ||
                (a.x, a.y, a.z) == bottom_left && (b.x, b.y, b.z) == bottom_right||
                (a.x, a.y, a.z) == bottom_right && (b.x, b.y, b.z) == top_right ||
                (a.x, a.y, a.z) == top_right && (b.x, b.y, b.z) == top_left{di = DIRECTION::FRONT;}
            //back
            else{
                di = DIRECTION::BACK;
            }
        }
        //either left or right (Actually either right or left for unknown reasons)
        if a.x == b.x && b.x == c.x && c.x == d.x{
            let top_left = (po.0, po.1 -0.5, po.2 + 0.5);
            let top_right = (po.0, po.1 +0.5, po.2 + 0.5);
            let bottom_left = (po.0, po.1 -0.5, po.2 -0.5);
            let bottom_right = (po.0, po.1 +0.5, po.2 -0.5);
            //left
            if  (a.x, a.y, a.z) == top_left && (b.x, b.y, b.z) == bottom_left ||
                (a.x, a.y, a.z) == bottom_left && (b.x, b.y, b.z) == bottom_right||
                (a.x, a.y, a.z) == bottom_right && (b.x, b.y, b.z) == top_right ||
                (a.x, a.y, a.z) == top_right && (b.x, b.y, b.z) == top_left{di = DIRECTION::RIGHT;}
            //right
             else{
                di = DIRECTION::LEFT;
            }
        }
        
        /*
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
        */
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
    pub dimensions: (u16, u16, u16),

    //used to evaluate the texture map of each face
    //pub cubes: Vec<Cube>,
    //____________Top,Bottom,Left,Right,Front,Back
    pub textures: Vec<vox_exporter::TextureMap>,
    //monochrome: bool,
    //-------------------indices----0 bottom left, 1 bottom right, 2 top right, 3 top left, 4-7 same thing but up and clockwise
    //important_vertices: [i32; 8]
    pub starting_position: (i32,i32,i32)

}
#[derive(Hash,Eq,PartialEq,Debug)]
pub struct CubeIndexPosition{
    x: i32,
    y: i32,
    z: i32
}
impl CubeIndexPosition{
    fn new(p:(f32,f32,f32))->CubeIndexPosition{
        CubeIndexPosition{
            x: (p.0-0.5) as i32,
            y: (p.1-0.5) as i32,
            z: (p.2-0.5) as i32,
        }
    }
    fn from(p:(f32,f32,f32))->CubeIndexPosition{
        CubeIndexPosition{
            x: p.0 as i32,
            y: p.1 as i32,
            z: p.2 as i32,
        }
    }
    fn to_tuple_xyz(&self)->(i32,i32,i32){
        return (self.x, self.y, self.z);
    }
}
//*/
//pub(crate) fn convert(my_app: &mut MyApp, path: &std::path::PathBuf, monochrome: &bool, pattern_matching: &bool, is_texturesize_powerof2: &bool, texturemapping_invisiblefaces: &bool, manual_vt: &bool, vt_precisionnumber: &u8, background_color: [f32;3], debug_uv_mode: bool){
pub(crate) fn convert(my_app: &mut MyApp, path: PathBuf){
    let x= format!("{}{}",String::from("converting:"), path.to_string_lossy().to_string());
    let _ = my_app.sx.send(x);
    my_app.status = String::from("Reading...");
    let content = read_file(&path.to_string_lossy().to_string());
    let ply_result:Result<ply, vox_importer::vox_importer_errors> = match content {
        Ok(content) => {
            //println!("{}", content);
            let x = format!("{}{}" ,String::from("parsing:"), &path.to_string_lossy().to_string());
            let _ = my_app.sx.send(x);
            parse_ply(&content)
            //my_app.status = "parsing" ; parse(content)
        },
        Err(error) => {
            println!("couldn't read!");
            let x = String::from(format!("Error while Reading!!! {}",error.to_string()));
            let _ = my_app.sx.send(x);

            return;
        }

    };
    let t = std::time::Instant::now();
    if let Ok(ply) = &ply_result {
        let x = String::from(format!("Optimizing model with {} vertices and {} faces", &ply.number_of_v_and_f.0, &ply.number_of_v_and_f.1));
        let _ = my_app.sx.send(x);

        //println!("{:?}", &ply);
    }
    if let Err(e) = &ply_result {
        let x = String::from(format!("Error while parsing!!! {}" ,e));
        let _ = my_app.sx.send(x);
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
    //println!("{:?}",&ply);

    //get size
    let mut vector_of_f: Vec<cube_f> = Vec::new();
    let mut lowest_coordinates = (99999.0,99999.0,99999.0);
    let mut highest_coordinates = (-99999.0, -99999.0, -99999.0);
    for v in &ply.vertices{
        //x
        if v.x<lowest_coordinates.0{
            lowest_coordinates.0=v.x
        } else if v.x>highest_coordinates.0{highest_coordinates.0=v.x}
        //y
        if v.y<lowest_coordinates.1{
            lowest_coordinates.1=v.y
        } else if v.y>highest_coordinates.1{highest_coordinates.1=v.y}
        //z
        if v.z<lowest_coordinates.2{
            lowest_coordinates.2=v.z
        } else if v.z>highest_coordinates.2{highest_coordinates.2=v.z}
    }
    for f in &ply.faces{
        let a: &v = &ply.vertices[f.vs.0 as usize];
        let b: &v = &ply.vertices[f.vs.1 as usize];
        let c: &v = &ply.vertices[f.vs.2 as usize];
        let d: &v = &ply.vertices[f.vs.3 as usize];
        let fa = cube_f::from_vertices(a, b, c, d);
        vector_of_f.push(fa);
    }

    let mut colourmatrix= ColourMatrix::def();
    colourmatrix.lowest_coordinates = (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32);
    colourmatrix.set_size((highest_coordinates.0 - lowest_coordinates.0) as i32,
                        (highest_coordinates.1 - lowest_coordinates.1) as i32,
                        (highest_coordinates.2 - lowest_coordinates.2)as i32);
    for fa in &vector_of_f{
        let index = (   ((fa.return_cube_position().0 - 0.5) as i32),
                        ((fa.return_cube_position().1 - 0.5) as i32),
                        ((fa.return_cube_position().2 - 0.5) as i32)   );
         if index.0 <= colourmatrix.shape.0 + colourmatrix.lowest_coordinates.0
         && index.1 <= colourmatrix.shape.1 + colourmatrix.lowest_coordinates.1 
         && index.2 <= colourmatrix.shape.2 + colourmatrix.lowest_coordinates.2
         && index.0 >= colourmatrix.lowest_coordinates.0
         && index.1 >= colourmatrix.lowest_coordinates.1
         && index.2 >= colourmatrix.lowest_coordinates.2{
            colourmatrix.set_cube_colour(index, fa.colour);
            colourmatrix.set_cube_bool(index, false);
        }else {
            println!("bad fa: {:?}, fa.return_cube_position()->{:?}", fa, fa.return_cube_position());
        }
    }
    if my_app.cull_optimization == true{
        let mut h_top = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_bottom = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_left = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_right = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_front = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_back = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        //println!("There are {:?} faces", &vector_of_f.len());
        for fa in &vector_of_f{
            let i = CubeIndexPosition::new(fa.return_cube_position());
                match fa.dir{
                DIRECTION::TOP    => {h_top.set_cube_bool(i.to_tuple_xyz(), true);}
                DIRECTION::BOTTOM => {h_bottom.set_cube_bool(i.to_tuple_xyz(), true);}
                DIRECTION::LEFT   => {h_left.set_cube_bool(i.to_tuple_xyz(), true);}
                DIRECTION::RIGHT  => {h_right.set_cube_bool(i.to_tuple_xyz(), true);}
                DIRECTION::FRONT  => {h_front.set_cube_bool(i.to_tuple_xyz(), true);}
                DIRECTION::BACK   => {h_back.set_cube_bool(i.to_tuple_xyz(), true);}
                }
            
        }
        for z in 0..h_left.matrixb.len(){
            for y in 0..h_left.matrixb[z].len(){
                for x in 0..h_left.matrixb[z][y].len(){
                    if h_left.matrixb[z][y][x] && !h_right.matrixb[z][y][x]{
                        let mut w = 1;
                        while !h_right.matrixb[z][y][x+w]{                       
                            if !h_front.matrixb[z][y][x+w] &&
                                !h_back.matrixb[z][y][x+w] &&
                                !h_top.matrixb[z][y][x+w] &&
                                !h_bottom.matrixb[z][y][x+w]{
                                colourmatrix.matrixb[z][y][x+w]=Some(false);  
                            }
                        w+=1;
                        if x > colourmatrix.shape.0 as usize { break };
                        }
                    }
                }
            }
        }
    }
    //println!("mapofcubes.len()={:?}", &colourmatrix.matrixb.len());
    //println!("{:?}", &mapofcubes);
    //if my_app.cull_optimization == false{
    let optimized_cubes = convert_to_optimized_cubes(&mut colourmatrix, my_app.cross,
     (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32));

    println!("{:?} optimized cubes in total", optimized_cubes.len());
    let mut obj = Obj::from_optimized_cubes(path.clone(), &my_app, &optimized_cubes);
    let x = String::from(format!("Exporting the mesh with {} vertices, {} faces and {}x{} texture size"
                ,obj.number_of_v_and_f.0, obj.number_of_v_and_f.1, obj.texture_map.w, obj.texture_map.h));
        let _ = my_app.sx.send(x);
    obj.export_all(colourmatrix.shape, (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32));
    println!("{:?}", "Finished optimizing mesh");
    let x = String::from(format!("{} {:?} in {:?}! ","Converted",path.to_string_lossy().to_string(),t.elapsed()));
        let _ = my_app.sx.send(x);
    /*
    }else{
        let optimized_cubes = convert_to_optimized_cubes_cull_optimized(&mut mapofcubes,
     (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32));

    println!("{:?} optimized cubes in total", optimized_cubes.len());
    let mut obj = Obj::from_optimized_cubes(path, &my_app, &optimized_cubes);
    let x = String::from(format!("Exporting the mesh with {} vertices, {} faces and {}x{} texture size"
                ,obj.number_of_v_and_f.0, obj.number_of_v_and_f.1, obj.texture_map.w, obj.texture_map.h));
        my_app.sx.send(x);
    obj.export_all();
    println!("{:?}", "Finished optimizing mesh");
    let x = String::from(format!("{}","Operation completed successfully!"));
        my_app.sx.send(x);
    }
    */
}
pub fn convert_vox(my_app: &mut MyApp, path:PathBuf){
    let x= format!("{}{}",String::from("converting:"), path.to_string_lossy().to_string());
    let _ = my_app.sx.send(x);
    my_app.status = String::from("Reading...");
    let content = read_file(&path.to_string_lossy().to_string());
    let vox_result:Result<Vox, vox_importer::vox_importer_errors> = match content {
        Ok(content) => {
            //println!("{}", content);
            let x = format!("{}{}" ,String::from("parsing:"), &path.to_string_lossy().to_string());
            let _ = my_app.sx.send(x);
            parse_vox(&content)
            //my_app.status = "parsing" ; parse(content)
        },
        Err(error) => {
            println!("couldn't read!");
            let x = String::from(format!("Error while Reading!!! {}",error.to_string()));
            let _ = my_app.sx.send(x);

            return;
        }

    };
    let t = std::time::Instant::now();
    if let Ok(Vox) = &vox_result {
        let x = String::from(format!("Optimizing model"));
        let _ = my_app.sx.send(x);

        //println!("{:?}", &ply);
    }
    if let Err(e) = &vox_result {
        let x = String::from(format!("Error while parsing!!! {}" ,e));
        let _ = my_app.sx.send(x);
        println!("{}", e);
    }
    let mut vox = vox_result.unwrap();
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum CanBeMerged{
    Yes,
    No,
    Cross,
}
pub fn convert_to_optimized_cubes(cubes: &mut ColourMatrix, cross: bool, lowest_coordinates:(i32, i32, i32)) -> Vec<OptimizedCube>{
    let mut cs = Vec::new();
    /*
    OptimizedCube{
        dimensions: (0, 0, 0),
        cubes: vec![],
        starting_position: (0,0,0),
    }
     */
    //println!("{:?}", lowest_coordinates);
    //println!("{:?}", cubes.shape);
    for z in lowest_coordinates.2..cubes.shape.2+lowest_coordinates.2+1{
        for y in lowest_coordinates.1..cubes.shape.1+lowest_coordinates.1+1{
            for x in lowest_coordinates.0..cubes.shape.0+lowest_coordinates.0+1{
                //println!("x:{:?} y:{:?} z:{:?}", x as i32,y as i32,z as i32);
                if let Some(opcube) = find_dimensions(cubes, (x as i32, y as i32, z as i32), &cross) {

                    cs.push(opcube);
                }
            }
        }
    }
    cs
}


fn find_dimensions(mymap: &mut ColourMatrix, index_we_are_at: (i32,i32,i32), cross_optimization: &bool) -> Option<OptimizedCube>{

    let mut shape = (1, 1, 1);
    //println!("{:?}", shape);
    //let mut cubes: std::vec::Vec<T> = Vec::new();


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
                /*
                if let Some(x) = mymap.get_cube(index_we_are_at.0 as i32, index_we_are_at.1 as i32, index_we_are_at.2 as i32) {
                    cubes.push(x);
                    mymap.merge_slice(index_we_are_at.0, index_we_are_at.0, index_we_are_at.1, index_we_are_at.1,
                        index_we_are_at.2, index_we_are_at.2);
                }
                */
            }
        }

    } else {return None;}
    //todo: implement a cache function (a vector of possible values
    //that answers the question can it be merged? (Yes, No, Cross (already been merged)))
    //like so: Yes, cross, cross, Yes
    //or: Yes, no -> you therefore stop
    //or: Yes, cross, cross, No -> you evaluate that the third is the last but being a Cross it cannot be last
    //so it asks the second one, can you be last? and he is a cross too so it becomes Yes, and that is it
    //println!("{:?}", "has it crashed yet? 1");

    let mut v_cached = Vec::new();
    //x
    let i = index_we_are_at.0.clone();
    let j = index_we_are_at.1.clone();
    let k = index_we_are_at.2.clone();
    //println!("{:?}", "has it crashed yet? 1.1");
    //println!("i:{:?} j:{:?} k:{:?}", i, j, k);
    while (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k) == CanBeMerged::Yes) ||
            (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k) == CanBeMerged::Cross && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k));
                shape.0 += 1;
            }
    //println!("{:?}", "has it crashed yet? 1.2");
    //println!("{:?}", shape);
    //todo()! -> v_cached gets sanitized (based on the cross rule) and then I put cubes in the optimized cubes based
    //           on vector lenght
    //println!("{:?}", v_cached);
    

    v_cached = cache_sanitization( v_cached, *cross_optimization);  
    shape.0 = v_cached.len() as i32 + 1 ;
    /*
    if shape.0 > 1{

        for v in mymap.get_cubes_from_slice(i+1, i+shape.0-1, j, j, k, k){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j, k, k)
    }
    */
    //println!("{:?}", "has it crashed yet? 1.3");
    //println!("{:?}", shape);
    //y
    v_cached = Vec::new();
    //println!("{:?}", mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k));
    //println!("{:?}", (i, i+shape.0, j+shape.1, j+shape.1, k, k));
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k));
                shape.1 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);    
    shape.1 = v_cached.len() as i32  + 1;
    /*
    if shape.1 > 1{
        for v in mymap.get_cubes_from_slice(i, i+shape.0-1, j+1, j+shape.1-1, k, k){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k)
    }
    */
    //println!("{:?}", "has it crashed yet? 1.4");
    //println!("shape:{:?}, ijk:{:?}", (shape),(i,j,k));
    //z
    v_cached = Vec::new();
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2));
                shape.2 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);
    shape.2 = v_cached.len() as i32  + 1;
    /*
    if shape.2 > 1{
        for v in mymap.get_cubes_from_slice(i, i+shape.0-1, j, j+shape.1-1, k+1, k+shape.2-1){
            cubes.push(v);
        }
        mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k+shape.2-1)
    }
    */
    let mut txt = Vec::new();
    for x in 0..6{
        txt.push(mymap.get_texturemap(x, i, i+shape.0, j, j+shape.1, k, k+shape.2));
    }
    mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k+shape.2-1);

    //println!("{:?}", "has it crashed yet? 2");
    //optimized cube forming
    //let starting_position = index_we_are_at;
    //println!("{:?}", shape);
    Some(OptimizedCube{
        dimensions: (shape.0 as u16, shape.1 as u16, shape.2 as u16),
        starting_position: (index_we_are_at.0 as i32, index_we_are_at.1 as i32, index_we_are_at.2 as i32),
        textures: txt,

    })
}

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
