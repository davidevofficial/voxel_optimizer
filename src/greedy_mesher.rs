use std::path::PathBuf;
use png::chunk::tRNS;

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
                for _x in 0..shapex{
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
        (xx as usize,yy as usize,zz as usize) //return
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
        (self.shape.0*self.shape.1*(zz as i32)+self.shape.0*(yy as i32)+(xx as i32)) as usize //return
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
                if let Some(_x) = y.get(xx) {
                    return self.matrixc[zz][yy][xx];
                }
            }
        }
        None //return
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
        None //return
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
                    Some(w) => {if w{
                                is_slice_already_merged = !is_all_not_merged;
                                }else{is_all_not_merged=true;}}
                    }
                } 
            }
        }
        
        if is_slice_already_merged{
            return CanBeMerged::Cross;
        }
        CanBeMerged::Yes //return
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
        true //return
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
        let mut vector_of_colours: Vec<Vec<Option<vox_exporter::Rgb>>> = Vec::new();
        let mut w = 1;
        let mut h = 1;
        //top
        if i == 0{
            w = x2-x1;
            h = y2-y1;
            let mut i = 0;
            for y in (y1..y2).rev(){
                for x in x1..x2{   
                        let rgb = self.get_cube_colour(x,y,z2-1);                
                        if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                        vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
                i+=1;
            }
        }
        //bottom//
        if i == 1{
            w = x2-x1;
            h = y2-y1;
            let mut i = 0;
            for y in (y1..y2){
                for x in (x1..x2){
                    //let rgb = self.get_cube_colour(x2-1-x,y2-1-y,z1);
                    let rgb = self.get_cube_colour(x,y,z1);
                    if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                 
                 i+=1;       vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
            }
        }
        //left//
        if i == 2{
            w = y2-y1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                for y in (y1..y2).rev(){
                    let rgb = self.get_cube_colour(x1,y,z);
                    if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                        vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
                i+=1;
            }
        }
        //right
        if i == 3{
            w = y2-y1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                for y in (y1..y2){
                    let rgb = self.get_cube_colour(x2-1,y,z);
                    if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                        vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
                i+=1;
            }
        }
        //front
        if i == 4{
            w = x2-x1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                for x in x1..x2{
                    let rgb = self.get_cube_colour(x,y1,z);
                    if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                        vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
                i+=1;
            }
        }
        //back//
        if i == 5{
            w = x2-x1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                for x in (x1..x2).rev(){
                    let rgb = self.get_cube_colour(x,y2-1,z);
                    if rgb.is_none(){vector_of_colours[i].push(None);}else if let Some(c) = rgb{
                        vector_of_colours[i].push(Some(Rgb{r:c.0, g: c.1, b: c.2}));}
                }
                i+=1;
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
        false //return
    }
    fn set_cube_bool(&mut self, i:(i32,i32,i32), b:bool){
        let ii = self.pos_to_index(i.0,i.1,i.2);
        self.matrixb[ii.2][ii.1][ii.0]=b;
    }
    fn contains(&mut self, x:usize, y:usize, z:usize)->bool{
        self.matrixb[z][y][x] == true //return
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Cube{
    //0= top, 1= bottom, 2= left, 3= right, 4= front, 5= back
    faces: [Option<CubeF>;6], //(it was about to be outdated even before I uncommented this mess LOL DEATH_EMOJI)
    position: (f32, f32, f32),
    colour: (u8, u8, u8),
    merged: bool
}
impl Cube{
    fn from_face(f: &CubeF) -> Cube{
        let m = false;
        let faces = match f.dir {
            Direction::Top =>    {[Some(*f),None,None,None,None,None]}
            Direction::Bottom => {[None,Some(*f),None,None,None,None]}
            Direction::Left =>   {[None,None,Some(*f),None,None,None]}
            Direction::Right =>  {[None,None,None,Some(*f),None,None]}
            Direction::Front =>  {[None,None,None,None,Some(*f),None]}
            Direction::Back =>   {[None,None,None,None,None,Some(*f)]}
        };
        let po = match f.dir{
            Direction::Top =>    {(f.position.0,f.position.1, f.position.2 - 0.5)}
            Direction::Bottom => {(f.position.0,f.position.1, f.position.2 + 0.5)}
            Direction::Left =>   {(f.position.0 + 0.5,f.position.1, f.position.2)}
            Direction::Right =>  {(f.position.0 - 0.5,f.position.1, f.position.2)}
            Direction::Front =>  {(f.position.0,f.position.1 + 0.5, f.position.2)}
            Direction::Back =>   {(f.position.0,f.position.1 - 0.5, f.position.2)}
        };
        Cube{
            position: po,
            faces,
            colour: f.colour,
            merged: m,
        }
    }
    fn from_nothing(position: (f32, f32, f32))-> Cube{
        Cube{
            position,
            faces: [None,None,None,None,None,None],
            colour: (0,0,0),
            merged: false,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct CubeF{
    position: (f32, f32, f32),
    dir: Direction,
    colour: (u8,u8,u8),
    //vertices_indices: [i32;4]
}
#[derive(Copy, Clone, Debug)]
pub enum Direction{
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}
pub struct CubeV{
    x: i32,
    y: i32,
    z: i32
}
pub struct CubeVt{
    u: f32,
    v: f32
}
impl CubeF {
    ///Returns a Face of the cube from 4 vertex
    ///
    ///Calculates average position, average colour and direction which is the cross product of the vectors a->b, b->c
    fn from_vertices(a: &v, b: &v, c: &v, d: &v) -> CubeF {
        let mut po = ((a.x + b.x + c.x + d.x) / 4.0,
                  (a.y + b.y + c.y + d.y) / 4.0,
                  (a.z + b.z + c.z + d.z) / 4.0);
        //Calculate the two vectors 6cycles
        let va = (b.x - a.x, b.y - a.y, b.z - a.z);
        let vb = (c.x - b.x, c.y - b.y, c.z - b.z);
        //cross product 9cycles
        //result = Normal of the face e.g. (-1, 0, 0) = Direction::Left
        let vc: (f32,f32,f32) = (
            va.1*vb.2-va.2*vb.1,
            va.2*vb.0-va.0*vb.2,
            va.0*vb.1-va.1*vb.0);
        po.0 -= vc.0/2.0;
        po.1 -= vc.1/2.0;
        po.2 -= vc.2/2.0;
        //let mut di: Direction = Direction::Front;
        
        let dir = match vc {
            (1.0,0.0,0.0) => Direction::Right,
            (-1.0,0.0,0.0) => Direction::Left,
            (0.0,1.0,0.0) => Direction::Top,
            (0.0,-1.0,0.0) => Direction::Bottom,
            (0.0,0.0,1.0) => Direction::Back,
            (0.0,0.0,-1.0) => Direction::Right,
            _ => panic!("Invalid cross product. Error code 205"),
        };
        
        let col = (a.r, a.g, a.b);

        CubeF {
            position: po,
            dir,
            colour: col,
            //vertices_indices:
        }
    }
    fn return_cube_index(&self) -> (i32, i32, i32) {
        ((self.position.0 - 0.5) as i32,(self.position.1 -0.5) as i32,(self.position.2 - 0.5) as i32) //return
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
        (self.x, self.y, self.z) //return
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
            let x = format!("Error while Reading!!! {}",error.to_string());
            let _ = my_app.sx.send(x);

            return;
        }

    };
    let t = std::time::Instant::now();
    if let Ok(ply) = &ply_result {
        let x = format!("Optimizing model with {} vertices and {} faces", &ply.number_of_v_and_f.0, &ply.number_of_v_and_f.1);
        let _ = my_app.sx.send(x);

        //println!("{:?}", &ply);
    }
    if let Err(e) = &ply_result {
        let x = format!("Error while parsing!!! {}" ,e);
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
    let mut vector_of_f: Vec<CubeF> = Vec::new();
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
        let fa = CubeF::from_vertices(a, b, c, d);
        vector_of_f.push(fa);
    }

    let mut colourmatrix= ColourMatrix::def();
    colourmatrix.lowest_coordinates = (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32);
    colourmatrix.set_size((highest_coordinates.0 - lowest_coordinates.0) as i32,
                        (highest_coordinates.1 - lowest_coordinates.1) as i32,
                        (highest_coordinates.2 - lowest_coordinates.2)as i32);
    for fa in &vector_of_f{
        let index = fa.return_cube_index();
         if index.0 <= colourmatrix.shape.0 + colourmatrix.lowest_coordinates.0
         && index.1 <= colourmatrix.shape.1 + colourmatrix.lowest_coordinates.1 
         && index.2 <= colourmatrix.shape.2 + colourmatrix.lowest_coordinates.2
         && index.0 >= colourmatrix.lowest_coordinates.0
         && index.1 >= colourmatrix.lowest_coordinates.1
         && index.2 >= colourmatrix.lowest_coordinates.2{
            colourmatrix.set_cube_colour(index, fa.colour);
            colourmatrix.set_cube_bool(index, false);
        }else {
            println!("bad fa: {:?}, fa.return_cube_position()->{:?}", fa, fa.return_cube_index());
        }
        //let i = 18446744073709551615;
    }
    if my_app.cull_optimization{
        let mut h_top = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_bottom = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_left = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_right = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_front = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        let mut h_back = BoolMatrix::from_size(colourmatrix.shape.0, colourmatrix.shape.1, colourmatrix.shape.2, colourmatrix.lowest_coordinates);
        //println!("There are {:?} faces", &vector_of_f.len());
        for fa in &vector_of_f{
                let i = fa.return_cube_index();
                match fa.dir{
                Direction::Top    => {h_top.set_cube_bool(i, true);}
                Direction::Bottom => {h_bottom.set_cube_bool(i, true);}
                Direction::Left   => {h_left.set_cube_bool(i, true);}
                Direction::Right  => {h_right.set_cube_bool(i, true);}
                Direction::Front  => {h_front.set_cube_bool(i, true);}
                Direction::Back   => {h_back.set_cube_bool(i, true);}
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
    let mut obj = Obj::from_optimized_cubes(path.clone(), my_app, &optimized_cubes);
    let x = format!("Exporting the mesh with {} vertices, {} faces and {}x{} texture size"
                ,obj.number_of_v_and_f.0, obj.number_of_v_and_f.1, obj.texture_map.clone().unwrap().w, obj.texture_map.clone().unwrap().h);
        let _ = my_app.sx.send(x);
    obj.export_all(colourmatrix.shape, (lowest_coordinates.0 as i32, lowest_coordinates.1 as i32, lowest_coordinates.2 as i32));
    println!("{:?}", "Finished optimizing mesh");
    let x = format!("{} {:?} in {:?}! ","Converted",path.to_string_lossy().to_string(),t.elapsed());
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
    let x= format!("{}{}",String::from("converting:"), path.to_string_lossy());
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
            let x = format!("Error while Reading!!! {}",error.to_string());
            let _ = my_app.sx.send(x);
            return;
        }

    };
    let t = std::time::Instant::now();
    if let Ok(vox) = &vox_result {
        let x = format!("Optimizing project with {} models ({} nodes)",vox.chunks.len(), vox.nodes.len());
        let _ = my_app.sx.send(x);

        //println!("{:?}", &ply);
    }
    if let Err(e) = &vox_result {
        let x = format!("Error while parsing!!! {}" ,e);
        let _ = my_app.sx.send(x);
        println!("{}", e);
        return;
    }
    let mut vox = vox_result.unwrap();
    //let mut vox = vox_result.unwrap();
    //case where everything is one mesh
    if my_app.all_in_one_mesh{
        //create a really big MaterialMatrix, optimize it, export it
        let mut lowest_coordinates = (99999,99999,99999);
        let mut highest_coordinates = (-99999,-99999,-99999);
        for m in vox.chunks.iter(){
            if m.position.0 < lowest_coordinates.0{
                lowest_coordinates.0 = m.position.0;
            }
            if m.position.1 < lowest_coordinates.1{
                lowest_coordinates.1 = m.position.1;
            }
            if m.position.2 < lowest_coordinates.2{
                lowest_coordinates.2 = m.position.2;
            }
            if m.position.0 + m.size.0 as i32 > highest_coordinates.0{
                highest_coordinates.0 = m.position.0 + m.size.0 as i32;
            }
            if m.position.1 + m.size.1 as i32> highest_coordinates.1{
                highest_coordinates.1 = m.position.1 + m.size.1 as i32;
            }
            if m.position.2 + m.size.2 as i32> highest_coordinates.2{
                highest_coordinates.2 = m.position.2 + m.size.2 as i32;
            }
        }
        let shape = (highest_coordinates.0-lowest_coordinates.0,
                                    highest_coordinates.1-lowest_coordinates.1,
                                    highest_coordinates.2-lowest_coordinates.2);
        //WARNING for the user
        if shape.0*shape.1*shape.2 > 512*512*512{
            let x = format!("WARNING: ({}x{}x{}) could be too big of a size to optimize, it might slow down your computer and use lots of ram"
                ,shape.0,shape.1,shape.2);
            let _ = my_app.sx.send(x);
            //std::thread::sleep(std::time::Duration::from_millis(6666));
        }
        let mut materialmatrix= MaterialMatrix::def(vox.materials.clone(),my_app.glass_creates_more_mesh);
        materialmatrix.lowest_coordinates = lowest_coordinates;
        materialmatrix.set_size(shape.0, shape.1, shape.2);

        for m in vox.chunks.iter(){
            for xyzi in &m.xyzi{
                let indexx = xyzi.x as i32+m.position.0;
                let indexy = xyzi.y as i32+m.position.1;
                let indexz = xyzi.z as i32+m.position.2;
                materialmatrix.set_cube_material((indexx,indexy,indexz), Some(xyzi.i), false);
            }
        }
        let optimized_vox = convert_to_optimized_vox(&mut materialmatrix, lowest_coordinates,&my_app.cross);
        println!("{:?} optimized cubes in total", optimized_vox.len());
        let mut obj = Obj::from_optimized_vox(path.clone(), my_app, &optimized_vox, vox.materials.clone());
        let x = format!("Exporting the mesh with {} vertices, {} faces and {}x{} texture size"
                ,obj.number_of_v_and_f.0, obj.number_of_v_and_f.1, obj.material_map.clone().unwrap().w, obj.material_map.clone().unwrap().h);
        let _ = my_app.sx.send(x);
        obj.export_all(materialmatrix.shape, lowest_coordinates);
        println!("{:?}", "Finished optimizing mesh");
        let x = format!("{} {:?} in {:?}! ","Converted",path.to_string_lossy().to_string(),t.elapsed());
        let _ = my_app.sx.send(x);

    }else{
        for m in vox.chunks.iter(){
            let mut lowest_coordinates = (99999,99999,99999);
            let mut highest_coordinates = (-99999,-99999,-99999);
            if m.position.0 < lowest_coordinates.0{
                lowest_coordinates.0 = m.position.0;
            }
            if m.position.1 < lowest_coordinates.1{
                lowest_coordinates.1 = m.position.1;
            }
            if m.position.2 < lowest_coordinates.2{
                lowest_coordinates.2 = m.position.2;
            }
            if m.position.0 + m.size.0 as i32 > highest_coordinates.0{
                highest_coordinates.0 = m.position.0 + m.size.0 as i32;
            }
            if m.position.1 + m.size.1 as i32> highest_coordinates.1{
                highest_coordinates.1 = m.position.1 + m.size.1 as i32;
            }
            if m.position.2 + m.size.2 as i32> highest_coordinates.2{
                highest_coordinates.2 = m.position.2 + m.size.2 as i32;
            }
            let shape = 
            (highest_coordinates.0-lowest_coordinates.0,
             highest_coordinates.1-lowest_coordinates.1,
             highest_coordinates.2-lowest_coordinates.2);
            //Path
            let mut p = path.clone();
            p.set_extension("");
            let path = PathBuf::from(format!("{}_{:?}.vox",p.to_string_lossy(), m.id));dbg!(&path);
            //WARNING for the user
            if shape.0*shape.1*shape.2 > 512*512*512{
                let x = format!("WARNING: ({}x{}x{}) could be too big of a size to optimize, it might slow down your computer and use lots of ram"
                    ,shape.0,shape.1,shape.2);
                let _ = my_app.sx.send(x);
                std::thread::sleep(std::time::Duration::from_millis(6666));
            }
            let mut materialmatrix= MaterialMatrix::def(vox.materials.clone(),my_app.glass_creates_more_mesh);
            materialmatrix.lowest_coordinates = lowest_coordinates;
            materialmatrix.set_size(shape.0, shape.1, shape.2);
            for xyzi in &m.xyzi{
                let indexx = xyzi.x as i32+m.position.0;
                let indexy = xyzi.y as i32+m.position.1;
                let indexz = xyzi.z as i32+m.position.2;
                materialmatrix.set_cube_material((indexx,indexy,indexz), Some(xyzi.i), false);
            
            }
            dbg!(&materialmatrix.is_glass, &materialmatrix.extra_mesh_glass);
            let optimized_vox = convert_to_optimized_vox(&mut materialmatrix, lowest_coordinates,&my_app.cross);
            
            println!("{:?} optimized cubes in total", optimized_vox.len());
            let mut obj = Obj::from_optimized_vox(path.clone(), my_app, &optimized_vox, vox.materials.clone());
            let x = format!("Exporting the mesh with {} vertices, {} faces and {}x{} texture size"
                    ,obj.number_of_v_and_f.0, obj.number_of_v_and_f.1, obj.material_map.clone().unwrap().w, obj.material_map.clone().unwrap().h);
            let _ = my_app.sx.send(x);
            obj.export_all(materialmatrix.shape, lowest_coordinates);
            println!("{:?}", "Finished optimizing mesh");
            let x = format!("{} {:?} in {:?}! ","Converted",path.to_string_lossy().to_string(),t.elapsed());
            let _ = my_app.sx.send(x); 
        }
    
    }
    //create a material matrix, optimize it, export it for each model
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum CanBeMerged{
    Yes,
    No,
    Cross,
}
///Materials is of lenght 256 and matrixm is of lenght [shape.0][shape.1][shape.2]
///
///If matrixm[x][y][z] is none then it doesn't exist, if it is Some(0) it exists but was merged
pub struct MaterialMatrix{
    shape: (i32, i32, i32),
    lowest_coordinates: (i32,i32,i32),
    matrixm: Vec<Vec<Vec<Option<u8>>>>,
    is_merged: Vec<Vec<Vec<bool>>>,
    materials: Vec<Matl>,
    is_glass: bool,
    extra_mesh_glass: bool,
}
impl MaterialMatrix{
    fn def(materials: Vec<Matl>, extra_mesh_glass: bool)->MaterialMatrix{
        let is_glass = if !extra_mesh_glass{
            false
        }else{
            let mut b = false;
            for m in 0..materials.len(){
                if materials[m].transparent != 0.0{
                    b = true;
                }
            }
            b
        };
        MaterialMatrix{
            matrixm: Vec::new(),
            is_merged: Vec::new(),
            materials,
            shape:(0,0,0),
            lowest_coordinates:(0,0,0),
            is_glass,
            extra_mesh_glass,
        }
    }
    fn set_size(&mut self, shapex: i32, shapey: i32, shapez: i32){
        self.shape = (shapex, shapey, shapez);
        for z in 0..shapez{
            self.matrixm.push(Vec::new());
            self.is_merged.push(Vec::new());
            for y in 0..shapey{
                self.matrixm[z as usize].push(Vec::new());
                self.is_merged[z as usize].push(Vec::new());
                for _x in 0..shapex{
                    self.matrixm[z as usize][y as usize].push(None);
                    self.is_merged[z as usize][y as usize].push(false);
                }
            }
        }
    }
    fn pos_to_index(&self, x:i32, y:i32, z:i32)->(usize,usize,usize){
        
        let xx = x-self.lowest_coordinates.0;
        let yy = y-self.lowest_coordinates.1;
        let zz = z-self.lowest_coordinates.2;
        
        (xx as usize,yy as usize,zz as usize) //return
    }
    fn get_cube_material(&mut self, x: i32, y:i32, z: i32)->Option<u8>{
        let (xx,yy,zz) = self.pos_to_index(x, y, z);
        if self.matrixm.get(zz).is_some() && self.matrixm[zz].get(yy).is_some() && self.matrixm[zz][yy].get(xx).is_some(){
            if self.matrixm[zz][yy][xx].is_some(){
                self.matrixm[zz][yy][xx]
            }else{None}
        }else{None} //return
    }
    fn get_cube_merged(&self, x:i32, y:i32, z:i32)->Option<bool>{
        let (xx,yy,zz) = self.pos_to_index(x, y, z);
        let id = if self.is_merged.get(zz).is_some() &&
                        self.is_merged[zz].get(yy).is_some() &&
                        self.is_merged[zz][yy].get(xx).is_some(){
                            self.is_merged[zz][yy][xx]
                        }else{return None};
        Some(id) //return
    }
    fn set_cube_material(&mut self, i:(i32,i32,i32), id:Option<u8>, merge: bool){
        let (xx,yy,zz) = self.pos_to_index(i.0, i.1, i.2);
        self.matrixm[zz][yy][xx]=id;
        self.is_merged[zz][yy][xx]=merge;
    }
    fn can_slice_be_merged(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32, transparent:f32) -> CanBeMerged {
        let mut is_slice_already_merged: bool = false;
        //let mut is_all_merged: bool=false;
        let is_glass = self.is_glass;
        let glass_creates_more_mesh = self.extra_mesh_glass;
        let deoptimize_glass = is_glass && glass_creates_more_mesh;
        let first_id = if self.get_cube_material(x1, y1, z1).is_none(){return CanBeMerged::No}else{
            self.get_cube_material(x1, y1, z1).unwrap() as usize
        };
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    let cube = self.get_cube_material(x, y, z);
                    let merged = self.get_cube_merged(x, y, z);
                    match cube{
                        None => {return CanBeMerged::No;}
                        Some(id) => {if self.materials[id as usize].transparent!=transparent && deoptimize_glass{return CanBeMerged::No;}
                                           if merged.unwrap(){is_slice_already_merged = true;}
                        }
                    }    
                }
            }
        }
        if is_slice_already_merged{
            return CanBeMerged::Cross;
        }
        CanBeMerged::Yes //return
    }
    fn is_slice_some(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> bool {
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    if self.get_cube_material(x,y,z).is_none() || Some(0) == self.get_cube_material(x,y,z){
                        return false;
                    }
                } 
            }
        }
        true //return
    }
    fn merge_slice(&mut self, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32){
        for z in z1..=z2{
            for y in y1..=y2{
                for x in x1..=x2{
                    let id =self.get_cube_material(x,y,z);
                    self.set_cube_material((x,y,z),id, true);
                } 
            }
        }
    }
    fn get_materialmap(&mut self, i: i32, x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> vox_exporter::MaterialMap{
        let mut vector_of_materials: Vec<Vec<u8>> = Vec::new();
        let mut w = 1;
        let mut h = 1;
        let materials = self.materials.clone();
        //top//
        if i == 0{
            w = x2-x1;
            h = y2-y1;
            let mut i = 0;
            for y in (y1..y2).rev(){
                vector_of_materials.push(Vec::new());
                for x in x1..x2{   
                        let id = self.get_cube_material(x,y,z2-1);                
                        if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }
        }
        //bottom//
        if i == 1{
            w = x2-x1;
            h = y2-y1;
            let mut i = 0;
            for y in (y1..y2){
                vector_of_materials.push(Vec::new());
                for x in (x1..x2){
                    //let rgb = self.get_cube_colour(x2-1-x,y2-1-y,z1);
                    let id= self.get_cube_material(x,y,z1);
                    if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }
        }
        //left//
        if i == 2{
            w = y2-y1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                vector_of_materials.push(Vec::new());
                for y in (y1..y2).rev(){
                    let id = self.get_cube_material(x1,y,z);
                    if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }
        }
        //right
        if i == 3{
            w = y2-y1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                vector_of_materials.push(Vec::new());
                for y in (y1..y2){
                    let id = self.get_cube_material(x2-1,y,z);
                    if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }
        }
        //front
        if i == 4{
            w = x2-x1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                vector_of_materials.push(Vec::new());
                for x in x1..x2{
                    let id = self.get_cube_material(x,y1,z);
                    if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }

        }
        //back//
        if i == 5{
            w = x2-x1;
            h = z2-z1;
            let mut i = 0;
            for z in (z1..z2).rev(){
                vector_of_materials.push(Vec::new());
                for x in (x1..x2).rev(){
                    let id = self.get_cube_material(x,y2-1,z);
                    if id.is_none(){vector_of_materials[i].push(0);}else if let Some(c) = id{
                        vector_of_materials[i].push(c);}
                }
                i+=1;
            }
        }
        vox_exporter::MaterialMap{w:w as usize,h:h as usize, id: vector_of_materials, materials }
    }
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
                if let Some(opcube) = find_dimensions(cubes, (x, y, z), &cross) {

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
    if mymap.is_slice_some(index_we_are_at.0, index_we_are_at.0,
                             index_we_are_at.1, index_we_are_at.1,
                              index_we_are_at.2, index_we_are_at.2){
        //can it be merged?
        match mymap.can_slice_be_merged(index_we_are_at.0, index_we_are_at.0,
                             index_we_are_at.1, index_we_are_at.1,
                              index_we_are_at.2, index_we_are_at.2){
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
    let i = index_we_are_at.0;
    let j = index_we_are_at.1;
    let k = index_we_are_at.2;
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
        starting_position: (index_we_are_at.0, index_we_are_at.1, index_we_are_at.2),
        textures: txt,

    })
}
pub struct OptimizedVox{
    pub starting_position: (i32,i32,i32),
    pub dimensions: (u16,u16,u16),
    pub textures: Vec<MaterialMap>,
}
pub fn convert_to_optimized_vox(mymap: &mut MaterialMatrix, lowest_coordinates:(i32, i32, i32), cross_optimization: &bool) -> Vec<OptimizedVox>{
    let mut cs = Vec::new();
    for z in lowest_coordinates.2..mymap.shape.2+lowest_coordinates.2+1{
        for y in lowest_coordinates.1..mymap.shape.1+lowest_coordinates.1+1{
            for x in lowest_coordinates.0..mymap.shape.0+lowest_coordinates.0+1{
                if let Some(opcube) = find_dimensions_vox(mymap, (x,y,z), cross_optimization) {
                    cs.push(opcube);
                }
            }
        }
    }
    cs
}
pub fn find_dimensions_vox(mymap: &mut MaterialMatrix, index_we_are_at: (i32,i32,i32), cross_optimization: &bool)->Option<OptimizedVox>{
    let mut shape = (1, 1, 1);
    //is the first cube a some value?
    if mymap.is_slice_some(index_we_are_at.0, index_we_are_at.0,
                             index_we_are_at.1, index_we_are_at.1,
                              index_we_are_at.2, index_we_are_at.2){
        let matl_id = mymap.get_cube_material(index_we_are_at.0, index_we_are_at.1, index_we_are_at.2).unwrap() as usize;
        let transparent = mymap.materials[matl_id].transparent;
        //can it be merged?
        match mymap.can_slice_be_merged(index_we_are_at.0, index_we_are_at.0,
                             index_we_are_at.1, index_we_are_at.1,
                              index_we_are_at.2, index_we_are_at.2, transparent){
            CanBeMerged::No => {return None;}
            CanBeMerged::Cross => {return None;}
            CanBeMerged::Yes => { 
            }
        }

    } else {return None;}

    let i = index_we_are_at.0;
    let j = index_we_are_at.1;
    let k = index_we_are_at.2;
    let matl_id = mymap.get_cube_material(index_we_are_at.0, index_we_are_at.1, index_we_are_at.2).unwrap() as usize;
    let transparent = mymap.materials[matl_id].transparent;
    //x
    let mut v_cached = Vec::new();
    while (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k, transparent) == CanBeMerged::Yes) ||
            (mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k, transparent) == CanBeMerged::Cross && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i+shape.0, i+shape.0, j, j, k, k, transparent));
                shape.0 += 1;
            }

    v_cached = cache_sanitization( v_cached, *cross_optimization);  
    shape.0 = v_cached.len() as i32 + 1 ;

    //y
    v_cached = Vec::new();
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k, transparent) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k, transparent) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j+shape.1, j+shape.1, k, k, transparent));
                shape.1 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);    
    shape.1 = v_cached.len() as i32  + 1;

    //z
    v_cached = Vec::new();
    while (mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2, transparent) == CanBeMerged::Yes) ||
            ((mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2, transparent) == CanBeMerged::Cross) && *cross_optimization){
                v_cached.push(mymap.can_slice_be_merged(i, i+shape.0-1, j, j+shape.1-1, k+shape.2, k+shape.2, transparent));
                shape.2 += 1;
            }  
    v_cached = cache_sanitization( v_cached, *cross_optimization);
    shape.2 = v_cached.len() as i32  + 1;

    let mut txt = Vec::new();
    for x in 0..6{
        txt.push(mymap.get_materialmap(x, i, i+shape.0, j, j+shape.1, k, k+shape.2));
    }
    mymap.merge_slice(i, i+shape.0-1, j, j+shape.1-1, k, k+shape.2-1);
    Some(OptimizedVox{
        dimensions: (shape.0 as u16, shape.1 as u16, shape.2 as u16),
        starting_position: (index_we_are_at.0, index_we_are_at.1, index_we_are_at.2),
        textures: txt,

    })
}
fn cache_sanitization(mut v_cached: Vec<CanBeMerged>, cross: bool) -> Vec<CanBeMerged>{
    if !v_cached.is_empty(){
    if cross {
        while !v_cached.is_empty() && v_cached[v_cached.len()-1] == CanBeMerged::Cross{
            v_cached.pop();
        }
        v_cached //return

    } else {
        let mut is_not_mergeable = false;
        let mut i = 0;
        while !v_cached.is_empty() && (i < v_cached.len()) && !is_not_mergeable{
            if v_cached[i] == CanBeMerged::Yes{
                i += 1;
            }else{
                is_not_mergeable = true;
            }
        }
        while i < v_cached.len() {
            v_cached.pop();
        }
        v_cached //return
    }
    }else {
        Vec::new()
    }

}
