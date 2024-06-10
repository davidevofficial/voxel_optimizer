use std::{error, fmt};
use std::error::Error;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, Read};
use crate::vox_importer::vox_importer_errors::{NotAscii, NotPly};
use crate::vox_exporter::Rgb;
use std::ops::{Add, Mul};

// Implement multiplication between i32 and (i32, i32, i32)
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3{x:i32,y:i32,z:i32}
#[derive(Debug, Default, Clone, Copy)]
pub struct Scalar{n:i32}
impl Vector3{
    fn from_tuple(xyz: (i32,i32,i32))->Vector3{Vector3 { x: xyz.0, y: xyz.1, z: xyz.2 }}
    fn to(self)->(i32,i32,i32){(self.x,self.y,self.z)}
    fn is_positive(xyz: &(i32,i32,i32))->(bool,bool,bool){(xyz.0>=0,xyz.1>=0,xyz.2>=0)}
}
impl Scalar{
    fn from_number(n: i32)->Scalar{Scalar {n}}
    fn to(self)->i32{self.n}
}
impl Mul<Vector3> for Scalar {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::from_tuple((self.n * rhs.x, self.n * rhs.y, self.n * rhs.z))
    }
}

// Implement addition between two (i32, i32, i32)
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::from_tuple((self.x + rhs.x, self.y + rhs.y, self.z + rhs.z))
    }
}
//Ply reader without using external libraries

#[derive(Debug)]
pub enum vox_importer_errors{
    NotPly,
    NotAscii,
    NotEphtracy,
    NotVox,
    NotVersion200,
    Other(String),
}
impl std::fmt::Display for vox_importer_errors{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self{
            vox_importer_errors::NotPly => write!{f,"Not ply"},
            vox_importer_errors::NotAscii => write!{f,"Not ascii"},
            vox_importer_errors::NotEphtracy => write!{f,"Not Ephtracy"},
            vox_importer_errors::NotVox => write!{f,"Not Vox"},
            vox_importer_errors::NotVersion200 => write!{f,"Not Version 200"},
            vox_importer_errors::Other(ref s) => write!{f,"Other error:{}",s},
        }
    }
}
impl std::error::Error for vox_importer_errors{}
#[derive(Debug, Default)]
pub struct v{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ply{
    pub(crate) fn normalize_positions(mut self) -> Self{
        for va in 0..self.vertices.len(){
            self.vertices[va].x = (self.vertices[va].x*10.0).round();
            self.vertices[va].y = (self.vertices[va].y*10.0).round();
            self.vertices[va].z = (self.vertices[va].z*10.0).round();
        }
            
    self
    }
}

#[derive(Debug, Default)]
pub struct f{
    pub(crate) vs: (i32, i32, i32, i32)
}

#[derive(Debug, Default)]
pub struct ply{
    //metadata
    ply_format: String,
    exported_by: String,
    pub number_of_v_and_f: (i32, i32),
    //vertices and faces
    pub vertices: Vec<v>,
    pub faces: Vec<f>
}

#[derive(Debug, Default)]
pub struct Vox{
    //metadata
    number_of_models: usize,
    vox_version: usize,
    //cubes
    pub chunks:Vec<Chunks>,
    pub colours: Vec<Rgb>,
    pub materials: Vec<Matl>,
    pub nodes: Vec<Node>,
    //each Node will be responsible of changing its children Translation and Rotation
    //pub tree: Vec<VoxTree>
}
impl Vox{
    fn update_nodes(&mut self){
        for nod in 0..self.nodes.len(){
            let x = self.nodes[nod].clone().find_children();
            let children_id = x.1.clone();

            //println!("Rotation: {:?}, r.0.to_vector({:?}),r.1.to_vector({:?}),r.2.to_vector({:?}),",);
            //if nSHP change the model (chunk) position and rotate all voxels inside
            if x.0{
                let mut ch = &mut self.chunks[children_id[0] as usize];
                ch.rotation = self.nodes[nod].find_attributes().rotation;
                let old_size = ch.size;
                let c  = Vector3::from_tuple((ch.size.0 as i32,ch.size.1 as i32,ch.size.2 as i32));
                let rx = Vector3::from_tuple(ch.rotation.0.to_vector());
                let ry = Vector3::from_tuple(ch.rotation.1.to_vector());
                let rz = Vector3::from_tuple(ch.rotation.2.to_vector());;
                let mut new_size = column_times_matrix(c,(rx,ry,rz));
                if new_size.x<0{
                    new_size.x= -new_size.x;
                }
                if new_size.y<0{
                    new_size.y= -new_size.y;
                }
                if new_size.z<0{
                    new_size.z= -new_size.z;
                }
                ch.size = (new_size.x as u8,new_size.y as u8, new_size.z as u8);
                //dbg!(ch);
                let cx = if old_size.0 % 2 == 0{
                    (old_size.0/2) as i32
                }else{
                    ((old_size.0 - 1)/2) as i32
                };
                let cy = if old_size.1 % 2 == 0{
                    (old_size.1/2) as i32
                }else{
                    ((old_size.1 - 1)/2) as i32
                };
                let cz = if old_size.2 % 2 == 0{
                    (old_size.2/2) as i32
                }else{
                    ((old_size.2 - 1)/2) as i32
                };
                let c = Vector3::from_tuple((cx,cy,cz));
                let rx = Vector3::from_tuple(ch.rotation.0.to_vector());
                let ry = Vector3::from_tuple(ch.rotation.1.to_vector());
                let rz = Vector3::from_tuple(ch.rotation.2.to_vector());
                let mut new_position = column_times_matrix(c, (rx,ry,rz));
                    let sign = Vector3::is_positive(&new_position.to());
                    if !sign.0{
                        new_position.x += ch.size.0 as i32;
                    }
                    if !sign.1{
                        new_position.y += ch.size.1 as i32;
                    }
                    if !sign.2{
                        new_position.z += ch.size.2 as i32;
                    }
                let (ccx,ccy,ccz) = new_position.to();
                println!("New centre: {:?}", (ccx,ccy,ccz));
                let t =   (self.nodes[nod].find_attributes().translation.0 - ccx,
                                            self.nodes[nod].find_attributes().translation.1 - ccy,
                                            self.nodes[nod].find_attributes().translation.2 - ccz);
                ch.position = t;
                //self.chunks[x.1[0] as usize].rotation = r;
                for v in 0..ch.xyzi.len(){
                    let c = Vector3::from_tuple((ch.xyzi[v].x as i32,ch.xyzi[v].y as i32,ch.xyzi[v].z as i32));
                    let rx = Vector3::from_tuple(ch.rotation.0.to_vector());
                    let ry = Vector3::from_tuple(ch.rotation.1.to_vector());
                    let rz = Vector3::from_tuple(ch.rotation.2.to_vector());
                    let mut new_position = column_times_matrix(c, (rx,ry,rz));
                    /*
                    let new_x = Scalar::from_number(ch.xyzi[v].x as i32) * Vector3::from_tuple(ch.rotation.0.to_vector());
                    let new_y = Scalar::from_number(ch.xyzi[v].y as i32) * Vector3::from_tuple(ch.rotation.1.to_vector());
                    let new_z = Scalar::from_number(ch.xyzi[v].z as i32) * Vector3::from_tuple(ch.rotation.2.to_vector());
                    */
                    //let mut new_position = new_x+ new_y+ new_z;
                    let sign = Vector3::is_positive(&new_position.to());
                    if !sign.0{
                        new_position.x += ch.size.0 as i32;
                    }
                    if !sign.1{
                        new_position.y += ch.size.1 as i32;
                    }
                    if !sign.2{
                        new_position.z += ch.size.2 as i32;
                    }
                    ch.xyzi[v].x = new_position.x as u8;
                    ch.xyzi[v].y = new_position.y as u8;
                    ch.xyzi[v].z = new_position.z as u8;

                }
            }else{
                for y in 0..x.1.len(){
                    let attributes = self.nodes[nod].clone().find_attributes();
                    //makes a new node with the same rotation as before and 
                    //translation of this node + the node we be modifying
                    self.nodes[x.1[y] as usize] = Node::add_attributes(&mut self.nodes[x.1[y] as usize].clone(), attributes); 
                    //Node::set_attributes(&mut self.nodes[x.1[y] as usize],attributes);
                }
            }
            
        }
    }
}
/*
#[derive(Debug, Default)]
pub struct VoxTree{
    //vector of nodes ordered by their ID like node[0] is the node with id 0, node[5] is the node with id 5
    parent: Option<u16>,
    node: (Node, NodeAttributes),
    child: Vec<u16>,
}

impl VoxTree{
    fn new(nodes: Vec<&Node>, parent:Option<u16>,node:Node, children: Vec<u16>)->Self{
        let node_attributes = if parent.is_some(){
            parent.unwrap().node.1
        }else{
            NodeAttributes::new()
        };
        VoxTree{
            parent,
            node: (node,node.find_attributes() + node_attributes),
            child: children,
        }

    }
    fn add_child(&mut self, nodeid:u16){self.child.append(nodeid);}
    /*
    fn check_translation_and_rotation(&mut self)->Option<NodeAttributes>{

    }
    */
}
*/
#[derive(Debug,Default,PartialEq, Eq, Clone,Copy)]
pub enum Versor{
    #[default] PosX,
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}
impl Versor{
    fn from_vector(vector: (i32,i32,i32))->Versor{
        match vector{
            (1,0,0) =>  Versor::PosX,
            (0,1,0) =>  Versor::PosY,
            (0,0,1) =>  Versor::PosZ,
            (-1,0,0) =>  Versor::NegX,
            (0,-1,0) =>  Versor::NegY,
            (0,0,-1) =>  Versor::NegZ,
            _ => panic!("Error code: 202, Invalid Rotation in one of the nTRN: {:?}",vector),

        }
    }
    fn to_vector(self)->(i32,i32,i32){
        match self{
            Versor::PosX => (1,0,0),
            Versor::PosY => (0,1,0),
            Versor::PosZ => (0,0,1),
            Versor::NegX => (-1,0,0),
            Versor::NegY => (0,-1,0),
            Versor::NegZ => (0,0,-1),
        }
    }
}
#[derive(Debug, Default, Clone, Copy,PartialEq, Eq)]
pub struct NodeAttributes{
    rotation: (Versor,Versor,Versor),
    translation: (i32,i32,i32),
}
impl Add for NodeAttributes{
    type Output = NodeAttributes;
    fn add(self, rhs: Self) -> Self::Output {
        if self.rotation != (Versor::PosX,Versor::PosY,Versor::PosZ) &&
             rhs.rotation != (Versor::PosX,Versor::PosY,Versor::PosZ){
                println!("Cannot rotate higher level model, Warning code:188, share it to davidevufficial@gmail.com")
             }
             //First matrix
        let a = self.rotation.0.to_vector();
        let b = self.rotation.1.to_vector();
        let c = self.rotation.2.to_vector();
        //println!("First matrix 279 :\n{:?}\n{:?}\n{:?}",a,b,c);
            //Second Matrix
        let d = rhs.rotation.0.to_vector();
        let e = rhs.rotation.1.to_vector();
        let f = rhs.rotation.2.to_vector();
        //println!("Second matrix 284 :\n{:?}\n{:?}\n{:?}",d,e,f);

        //matrix multiplication
        //first row
        let g = (a.0*d.0)+(a.1*e.0)+(a.2*f.0);
        let h = (a.0*d.1)+(a.1*e.1)+(a.2*f.1);
        let i = (a.0*d.2)+(a.1*e.2)+(a.2*f.2);
        //println!("First row 285 :\n{:?}\n{:?}\n{:?}",g,h,i);

        let j = (b.0*d.0)+(b.1*e.0)+(b.2*f.0);
        let k = (b.0*d.1)+(b.1*e.1)+(b.2*f.1);
        let l = (b.0*d.2)+(b.1*e.2)+(b.2*f.2);
        //println!("Second row 285 :\n{:?}\n{:?}\n{:?}",j,k,l);

        let m = (c.0*d.0)+(c.1*e.0)+(c.2*f.0);
        let n = (c.0*d.1)+(c.1*e.1)+(c.2*f.1);
        let o = (c.0*d.2)+(c.1*e.2)+(c.2*f.2);
        //println!("Third row 285 :\n{:?}\n{:?}\n{:?}",m,n,o);

        //Versorize it
        let (p,q,r) = (Versor::from_vector((g,h,i)),
            Versor::from_vector((j,k,l)),Versor::from_vector((m,n,o)));
        //Return rotation and translation
        NodeAttributes { rotation: (p,q,r),
                         translation:  (self.translation.0+rhs.translation.0,
                                        self.translation.1+rhs.translation.1,
                                        self.translation.2+rhs.translation.2) 
        }
    }
}
impl NodeAttributes{
    fn from(r: u8, t: (i32,i32,i32))->NodeAttributes{
        //Thanks to this:
        //https://github.com/jpaver/opengametools/blob/master/src/ogt_vox.h
        //I managed to make sense of the .vox file format better
        //and also this: https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox-extension.txt
        if r == 0{
            return NodeAttributes{rotation: (Versor::PosX, Versor::PosY, Versor::PosZ), translation: t};
        }
        let versors = [(1,0,0),(0,1,0),(0,0,1)];
        let axisx = (r>>0)&3; // axisx can be 0,1,2 
        let axisy = (r>>2)&3; // axisy can be 0,1,2
        // based on axis x and axis y axis z will be whatever axis x and y are not
        let mut axisz = 2;
        if (axisx == 1 && axisy == 2) || (axisx == 2 && axisy == 1){
        axisz = 0;
        }else if(axisx==2 && axisy==0)||(axisx==0 && axisy==2){
            axisz = 1;
        }
        let mut rowx = versors[axisx as usize];
        let mut rowy = versors[axisy as usize];
        let mut rowz = versors[axisz];
        //bit five, six and seven if is one flips the versor
        if ((r>>4)%2)==1{
            rowx = (-rowx.0, -rowx.1, -rowx.2);
        }
        if ((r>>5)%2)==1{
            rowy = (-rowy.0, -rowy.1, -rowy.2);
        }
        if ((r>>6)%2)==1{
            rowz = (-rowz.0, -rowz.1, -rowz.2);
        }
        let rotation = (Versor::from_vector(rowx),Versor::from_vector(rowy),Versor::from_vector(rowz));
        NodeAttributes { rotation, translation: t }
    }
    fn new()->NodeAttributes{
        NodeAttributes { rotation: (Versor::PosX,Versor::PosY,Versor::PosZ), translation: (0,0,0) }
    }
}
#[derive(Debug, Default, Clone)]
pub struct Trn{
    size_in_bytes: u16,
    node_id: u16,
    //_name, _hidden 
    //attributes: Dict,
    name: Vec<u8>,
    hidden: u8,
    child_node_id: u16,
    layer: u8,
    //n_of_frames: u8,
    //_r, _t, _f
    //properties: Dict,
    node_attributes: NodeAttributes,
}
impl Trn{
    pub fn from_bytes(bytes: Vec<&u8>)->Trn{

        //bytesize
        let mut i = 0;
        let bytesize = *(bytes[i])as u16+(256**(bytes[i+1])as u16);
        i += 8;
        //ID
        let id = *bytes[i] as u16 + *bytes[i+1] as u16 *255;
        i += 4;
        //println!("Bytesize:{:?}, Id:{:?}", bytesize, id);
        //Attributes Number
        let mut attributes_n = *bytes[i];
        i += 4;

        let mut name: Vec<u8> = Vec::new();
        let mut hidden = 0_u8;
        while attributes_n >= 1{
            //how many bytes is the next word long? (can be either _name or _hidden so 5 or 7)
            let j = *bytes[i];
            i+=4;

            //either _name or _hidden
            let a = &bytes[i..(i+j as usize)];
            i+=j as usize;

            //how many bytes is the next word? (can be either a name or the hidden flag of type bool 0|1)
            let j = *bytes[i];
            i+=4;
            //either a bool or a string
            let b = &bytes[i..(i+j as usize)];
            i+=j as usize;

            if a[1]==&b'n'{
                for x in 0..b.len(){
                    name.push(*b[x]);
                }
            }else{
                hidden = *b[0] - 48_u8;
            }
            attributes_n -= 1;
        }
        //println!("Name:{:?}, Hidden:{:?}", name, hidden);
        //ID of his only child (it is not a vec because it is a Trn not a Grp)
        let child_id = (*bytes[i] as u16) + (*bytes[i+1] as u16 *255);
        i+=4;

        // -1 (reserved_id) (skip)
        i+=4;

        // Number of frames (panics if it is more than 1)
        let nf =(*bytes[i] as usize) +
                        (*bytes[i+1] as usize *255) +
                        (*bytes[i+2] as usize *255*255) +
                        (*bytes[i+3] as usize *255*255*255);
        /*
        if nf > 0{
            panic!("More than one frame! No animations allowed, error code: 166")
        } 
        */
        i+=4;
        //println!("Childid:{:?}, Number of frames:{:?}", child_id, nf);

        //layer_id
        let layer_id = bytes[i];
        i+=4;
        
        //Attributes Number
        let mut attributes_n = *bytes[i];
        i += 4;
        if attributes_n >= 3{panic!("More than one frame! No animations allowed, error code: 167")}
        //println!("Layerid:{:?}, attributes_n:{:?}", layer_id, attributes_n);
        let mut translation = (0,0,0);
        let mut rotation = 0_u8;
        while attributes_n >= 1{
            //how many bytes is the next word long? (can be either _r or _t so 2)
            let j = *bytes[i];
            i+=4;

            //either _r or _t
            let a = &bytes[i..(i+j as usize)];
            i+=j as usize;
            //println!("a:{:?}, i:{:?}, j:{:?}",a,i,j);
            //how many bytes is the next word? (can be either a int32x3 or a int8)
            let j = *bytes[i];
            i+=4;
            //either an int32x3 or an int8
            let b = &bytes[i..(i+j as usize)];
            i+=j as usize;
            //println!("b:{:?}, i:{:?}, j:{:?}",b,i,j);

            if a[1]==&b't'{
                let mut int32x3 = Vec::new();
                let mut i32x3 = Vec::new();
                int32x3.push(Vec::new());
                int32x3.push(Vec::new());
                int32x3.push(Vec::new());

                i32x3.push(0_i32);
                i32x3.push(0_i32);
                i32x3.push(0_i32);

                //let mut spaces_indices = Vec::new();
                let mut c = 0;
                for x in 0..b.len(){
                    if *b[x] == b' '{
                        c+=1;
                    }else{
                    int32x3[c].push(*b[x]);
                    }
                    /*
                    if *b[x] == b' '{
                        let n = spaces_indices.len();
                        spaces_indices.push(x);
                        int32x3.push(b[spaces_indices[n-1]..spaces_indices[n]].to_vec());
                    }
                    */
                }
                for x in 0..3{
                    i32x3[x] = bytes_to_numeric(int32x3[x].as_slice()).unwrap();
                    /*
                    let negative = if *int32x3[x][0] == b'-'{true}else{false};
                    for y in 0..int32x3[x].len(){
                        if negative{
                            if y!=0{
                            i32x3[x] -= (int32x3[x][y]-48) as i32*10_i32.pow((int32x3.len()-1-y) as u32);
                            }
                        }else{
                        i32x3[x] += (int32x3[x][y]-48) as i32*10_i32.pow((int32x3.len()-1-y) as u32);
                        }
                    }
                    */
                }
                translation = (i32x3[0], i32x3[1], i32x3[2]);
            }else if a[1] == &b'r'{
                let mut int8 = 0;
                for x in 0..b.len(){
                    //println!("{:?} at {:?}",b,i);
                       int8 += ((b[b.len()-1-x]-48) as i32*(10_i32.pow(x as u32))) as u8; 
                }
                rotation = int8;
            }
            attributes_n -= 1;
        }

        /*
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        //println!("{:?}", dict);
        let childid = bytes[16+size_of_dict];
        let layer = bytes[24+size_of_dict];
        let n_of_frames = bytes[28 + size_of_dict];
        if n_of_frames != &1{
            panic!("More than one frame! No animations allowed");
        }
        let attributes_n = bytes[28 + size_of_dict];
        let mut size_of_dict2 = 0;
        let mut dict2 = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for _x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[28+size_of_dict+size_of_dict2];
                size_of_dict2 += v_string1.buffer_size as usize;
                for x in 32+size_of_dict+size_of_dict2..32+size_of_dict+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[28+size_of_dict];
                size_of_dict2 += v_string2.buffer_size as usize;
                for x in 32+size_of_dict+size_of_dict2..32+size_of_dict+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict2.key_values.push((v_string1,v_string2))
            }
        }
        */
        Trn{
            size_in_bytes:bytesize,
            node_id: id,
            //attributes:dict,
            child_node_id: child_id,
            layer:*layer_id,
            //n_of_frames:*n_of_frames,
            //properties: dict2,
            name: name,
            hidden,
            node_attributes: NodeAttributes::from(rotation, translation),
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct Grp{
    size_in_bytes: u16,
    node_id: u16,
    //_name, _hidden 
    node_attributes: NodeAttributes,
    n_of_children: u8,
    children_node_id: Vec<u16>,
}
impl Grp{
    pub fn from_bytes(bytes: Vec<&u8>)->Grp{
        //bytesize
        let mut i = 0;
        let bytesize = *(bytes[i+0])as u16+(256**(bytes[i+1])as u16)as u16;
        i += 8;
        //ID
        let id = *bytes[i] as u16 + *bytes[i+1] as u16 *255;
        i += 4;

        //Attributes Number
        let mut attributes_n = *bytes[i];
        i += 4;

        //ignore this atributes
        while attributes_n >= 1{
            //how many bytes is the next word long? (can be either ??)
            let j = *bytes[i];
            i+=4;

            //either ??
            let a = &bytes[i..(i+j as usize)];
            i+=j as usize;

            //how many bytes is the next word? (can be either ??)
            let j = *bytes[i];
            i+=4;
            //either ??
            let b = &bytes[i..(i+j as usize)];
            i+=j as usize;

            /*
            if a[1]=&b'n'{
                name = b;
            }else{
                hidden = b;
            }
            */
            attributes_n -= 1;
        }
        // Number of children
        let n_of_children = bytes[i];
        i+=4;

        //Vector of all its children
        let mut childid = Vec::new();
        for n in 0..*n_of_children{
            childid.push(*bytes[i] as u16+*bytes[i+1] as u16 *255);
            i+=4;
        }

        /*
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        let n_of_children = bytes[16+size_of_dict];
        let mut childid = Vec::new();
        for n in 0..*n_of_children{
            childid.push(*bytes[16+size_of_dict+4+4*n as usize]);
        }
        */
        //let childid = bytes[16+size_of_dict];
        
        Grp{
            size_in_bytes:bytesize,
            node_id:id,
            node_attributes:NodeAttributes::new(),
            n_of_children:*n_of_children,
            children_node_id: childid,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Shp{
    size_in_bytes: u16,
    node_id: u16,
    //_name, _hidden ?
    //attributes: Dict,
    n_of_models: u8,
    model_id: u16,
    node_attributes: NodeAttributes,
}
impl Shp{
    pub fn from_bytes(bytes: Vec<&u8>)->Shp{
        //bytesize
        let mut i = 0;
        let bytesize = *(bytes[i+0])as u16+(256**(bytes[i+1])as u16)as u16;
        i += 8;
        //ID
        let id = *bytes[i] as u16 + *bytes[i+1] as u16 *255;
        i += 4;

        //Attributes Number
        let mut attributes_n = *bytes[i];
        i += 4;

        while attributes_n >= 1{
            //Key
            let j = *bytes[i];
            i+=4;
            let a = &bytes[i..(i+j as usize)];
            i+=j as usize;
            //Value
            let j = *bytes[i];
            i+=4;
            let b = &bytes[i..(i+j as usize)];
            i+=j as usize;
            attributes_n -= 1;
        }
        // Number of models (should be just 1, panic if it isn't)
        let n_of_models = *bytes[i];
        if n_of_models != 1{
            panic!("No more than one model per nSHP is allowed, are you sure you have disabled all animations?\n
                     Error code 177\n Share the code to davidevufficial@gmail.com");
        }
        i+=4;

        //Model id
        let model_id = *(bytes[i])as u16+(256**(bytes[i+1])as u16)as u16;

        //let mut modelsid = Vec::new();

        /*
        for n in 0..*n_of_models{

        }
        let bytesize = *(bytes[0])as u16+(256**(bytes[1])as u16)as u16;
        let id = bytes[8];
        let attributes_n = bytes[12];
        let mut size_of_dict = 0;
        let mut dict = Dict{n_of_key_values:*attributes_n, key_values:Vec::new()};
        if attributes_n > &0{
            for x in 0..*attributes_n{
                let mut v_string1 = VoxString{buffer_size:0, content:Vec::new()};
                let mut v_string2 = VoxString{buffer_size:0, content:Vec::new()};
                v_string1.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string1.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string1.buffer_size as usize{
                    v_string1.content.push(*bytes[x]);
                }
                v_string2.buffer_size = *bytes[16+size_of_dict];
                size_of_dict += v_string2.buffer_size as usize;
                for x in 20+size_of_dict..20+1+v_string2.buffer_size as usize{
                    v_string2.content.push(*bytes[x]);
                }
                dict.key_values.push((v_string1,v_string2))
            }
        }
        let n_of_models = bytes[16+size_of_dict];
        let mut modelsid = Vec::new();
        for n in 0..*n_of_models{
            modelsid.push(*bytes[16+size_of_dict+4+4*n as usize]);
        }
        */
        //let childid = bytes[16+size_of_dict];
        
        Shp{
            size_in_bytes:bytesize,
            node_id:id,
            //attributes:dict,
            n_of_models,
            model_id,
            node_attributes: NodeAttributes::new(),
        }
    }
}
#[derive(Debug, Default)]
pub struct Dict{
    n_of_key_values: u8,
    key_values: Vec<(VoxString, VoxString)>,
}
#[derive(Debug, Default)]
pub struct VoxString{
    buffer_size: u8,
    content: Vec<u8>,
}
#[derive(Debug, Clone)]
pub enum Node{
    TRN(Trn),
    GRP(Grp),
    SHP(Shp),
}
impl Default for Node{
    fn default() -> Self{
        Node::TRN(Trn::default())
    }
}
impl Node{
    fn find_attributes(&self)->NodeAttributes{
        match &self {
            Node::TRN(trn) => trn.node_attributes,
            Node::GRP(grp) => grp.node_attributes,
            Node::SHP(shp) => shp.node_attributes,
        }
    }
    ///Finds all the children of the node
    //
    /// # Return
    /// Returns a tuple of a bool (is nSHP?) and the children
    fn find_children(&self)->(bool,Vec<u16>){
        match &self {
            Node::TRN(trn) => (false,vec![trn.child_node_id]),
            Node::GRP(grp) => (false,grp.children_node_id.clone()),
            Node::SHP(shp) => (true,vec![shp.model_id]),
        }
    }
    fn add_attributes(node:&mut Node, node_attribute: NodeAttributes)->Self{
        match node {
            Node::TRN(ref mut trn) => {let mut ret = trn.clone();
                                    ret.node_attributes = trn.node_attributes+node_attribute;
                                    Node::TRN(ret)},
            Node::GRP(ref mut grp) => {let mut ret = grp.clone();
                                    ret.node_attributes = grp.node_attributes+node_attribute;
                                    Node::GRP(ret)},
            Node::SHP(ref mut shp) => {let mut ret = shp.clone();
                                    ret.node_attributes.rotation=node_attribute.rotation;
                                    ret.node_attributes = shp.node_attributes+node_attribute;
                                    Node::SHP(ret)},
        }
    }
}
#[derive(Debug, Default)]
pub struct Chunks{
    pub id: u16,
    pub position: (i32,i32,i32),
    pub rotation: (Versor, Versor, Versor),
    pub size: (u8, u8, u8),
    pub xyzi: Vec<VoxCubes>,
}
#[derive(Debug, Default)]
pub struct VoxCubes{
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub i: u8,
}
impl VoxCubes{
    pub fn from(x:u8,y:u8,z:u8,i:u8)->VoxCubes{return VoxCubes{x,y,z,i};}
}
#[derive(Debug, Default)]
pub struct Matl{
    pub id: u8,
    //albedo
    pub rgb: Rgb, //rgb
    pub transparent: f32, //_alpha or _trans, 0<x<=1, if != 0 -> Later check for the is glass different model flag
    //roughness map (r channel)
    pub roughness: f32, //_rough 0<=x<=1
    //refraction map (a channel)
    pub ior: f32, //_ior = _ri - 1.0, 0<=_ior<=2
    //metallic map (g and b channel)
    pub specular: f32, //_sp 0<=x<=1
    pub metallic: f32, //_metal 0<=x<=1
    //emission map (optional)
    pub rgb_e: Option<Rgb>,
}
//Reads the ply files and returns the content as a string
//
//
pub(crate) fn is_valid_ply(ply_path: &std::path::PathBuf) -> bool{
    ply_path.extension().unwrap() == std::ffi::OsStr::new("ply") // return true|false
}
pub(crate) fn is_vox(vox_path: &std::path::PathBuf) -> bool{
    vox_path.extension().unwrap() == std::ffi::OsStr::new("vox") //return true|false
}
//pub fn is_valid_vox()
pub fn read_file(filepath: &String) -> Result<Vec<u8>, io::Error>{
    //let mut output = String::new();
    //File::open(filepath)?.read_to_string(&mut output)?;
    let mut output = Vec::new();
    File::open(filepath)?.read_to_end(&mut output)?;
    Ok(output)
}
//Parses the ply file and returns a list of vertices and faces as a list
pub fn parse_ply(content: &Vec<u8>) -> Result<ply, vox_importer_errors>{

    let mut ply: ply = ply::default();
    //let ply_bytes = content.as_bytes();
    let ply_bytes = content;

    //ply check
    let result: Result<&[u8; 3], _> = ply_bytes[0..3].try_into();
    //println!("{:?}",result);
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != b"ply"{
                return Err(vox_importer_errors::NotPly);
                }
            }
            Err(_) => println!("Failed!"),
        }

    //ascii check
    let result: Result<&[u8; 16], _> = ply_bytes[5..0x15].try_into();
    match result{
        Ok(b) =>{
            if b != b"format ascii 1.0"{
                return Err(vox_importer_errors::NotAscii);
            } else { ply.ply_format = String::from("ascii 1.0") }
        }
        Err(_) => println!("Invalid!"),
    }
    //magicavoxel check
    let result: Result<&[u8; 32], _> = ply_bytes[0x17..0x37].try_into();
    match result{
        Ok(b) => {
            if b != b"comment : MagicaVoxel @ Ephtracy"{
                return Err(vox_importer_errors::NotEphtracy);
            } else { ply.exported_by = String::from("comment : Magicavoxel @ Ephtracy") }
        }
        Err(_) => println!("Error not made by Ephtracy's software"),
    }

    let nv_index = find_x_in_y(b"element vertex ", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap();
    let nv_newline_index = find_next_newline_after_index(&ply_bytes[nv_index..]).unwrap() +nv_index-1;
    let nf_index = find_x_in_y(b"element face ", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap();
    let nf_newline_index = find_next_newline_after_index(&ply_bytes[nf_index..]).unwrap() + nf_index-1;

    ply.number_of_v_and_f = (bytes_to_numeric::<i32>(&ply_bytes[(nv_index + 15)..nv_newline_index]).unwrap(),
                             bytes_to_numeric::<i32>(&ply_bytes[(nf_index + 13)..nf_newline_index]).unwrap());


    let mut start_index: usize = find_x_in_y(b"end_header", &ply_bytes).ok_or(vox_importer_errors::Other(String::from("Error while reading"))).unwrap() + 12;
    let mut end_index: usize = 0;
    let mut vec_v: Vec<v> = Vec::new();
    for v in 0..ply.number_of_v_and_f.0 {
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        //println!("{:?}", &tokens);
        //println!("{:?}", &v);
        start_index = end_index + 1;
        vec_v.push(v::default());
        //x, y, z value
        vec_v[v as usize].x = (bytes_to_numeric::<f32>(&tokens[0])).unwrap();
        vec_v[v as usize].y = (bytes_to_numeric::<f32>(&tokens[1])).unwrap();
        vec_v[v as usize].z = (bytes_to_numeric::<f32>(&tokens[2])).unwrap();
        //r, g, b value
        vec_v[v as usize].r = (bytes_to_numeric::<u8>(&tokens[3])).unwrap();
        vec_v[v as usize].g = (bytes_to_numeric::<u8>(&tokens[4])).unwrap();
        vec_v[v as usize].b = (bytes_to_numeric::<u8>(&tokens[5])).unwrap();
    }
    let mut vec_f: Vec<f> = Vec::new();
    for f in 0..ply.number_of_v_and_f.1 {
        end_index = find_next_newline_after_index(&ply_bytes[start_index..]).unwrap() + start_index;
        let tokens = split_into_words(&ply_bytes[start_index..(end_index - 1)]);
        //println!("{:?}", &tokens);
        //println!("{:?}", &f);
        start_index = end_index + 1;
        vec_f.push(f::default());
        //a, b, c, d indices
        vec_f[f as usize].vs.0 = (bytes_to_numeric::<i32>(&tokens[1])).unwrap();
        vec_f[f as usize].vs.1 = (bytes_to_numeric::<i32>(&tokens[2])).unwrap();
        vec_f[f as usize].vs.2 = (bytes_to_numeric::<i32>(&tokens[3])).unwrap();
        vec_f[f as usize].vs.3 = (bytes_to_numeric::<i32>(&tokens[4])).unwrap();


    }
    ply.faces = vec_f;
    ply.vertices = vec_v;
    Ok(ply)
}
pub fn parse_vox(content: &Vec<u8>) -> Result<Vox, vox_importer_errors>{
    let mut vox: Vox = Vox::default();
    let vox_bytes = content;
    //vox check
    let result: Result<&[u8; 4], _> = vox_bytes[0..4].try_into();
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != b"VOX "{
                return Err(vox_importer_errors::NotVox);
                }
            }
            Err(_) => println!("Failed!"),
        }
    let result: Result<u8, _> = vox_bytes[4].try_into();
        match result {
            Ok(bytes_fixed) => {
                if bytes_fixed != 200{
                return Err(vox_importer_errors::NotVersion200);
                }
            }
            Err(_) => println!("Failed!"),
        }
    vox.vox_version = 200;
    //while find_x_in_y(x)
    //__________________S
    // 0001020304    05060708090A0B0C0D0E0F 10 11 12 13            14151617 18191A1B1C1D1E1F 20-2324-2728-2B
    //  V O X  (200)  . . . M A I N . . . . (reverse order size 4b) S I Z E 0C . . . . . . . sizexsizeysizez
    // 2c 2d 2e 2f 30 31 32 33 34 35 36 37
    //  X  Y  Z  I (rev. size 8b) ({37}-{37}+[{33}*16^3+{32}*16^2+{31}*16+{30}]) (35, 36, 37 will be 0)
    // goes from "XYZI........"|to|"SIZE" or "nTRP"
    // S+36 to S
    if vox_bytes[20] != 0x53{
        return Err(vox_importer_errors::Other("No models in the .vox file".to_string()));
    }
    //                  0x14
    let mut size_index = 20;
    while vox_bytes[size_index] == 0x53{

        let mut chunk = Chunks::default();
        chunk.rotation.0 = Versor::PosX;
        chunk.rotation.1 = Versor::PosY;
        chunk.rotation.2 = Versor::PosZ;
        chunk.size.0 = vox_bytes[size_index+12];
        chunk.size.1 = vox_bytes[size_index+16];
        chunk.size.2 = vox_bytes[size_index+20];

        let byte_size = (vox_bytes[size_index+31]as usize)*256*256*256+
                            (vox_bytes[size_index+30]as usize)*256*256+
                            (vox_bytes[size_index+29]as usize)*256+
                            (vox_bytes[size_index+28]as usize);
        let n_of_voxels = ((byte_size-4)/4);
        //println!("{:?}", byte_size);
        for voxel in 0..n_of_voxels{
        //                                  (8 chunk byte size, 4 number of voxels)
        //________________Size->z, 3by -> 4bytes(XYZI) -> 12 Size -> +n (.n)->voxel
            let x = vox_bytes[size_index + 20 +3+ 4 + 12 + 1+4*voxel];
            let y = vox_bytes[size_index + 20 +3+ 4 + 12 + 2+4*voxel];
            let z = vox_bytes[size_index + 20 +3+ 4 + 12 + 3+4*voxel];
            let i = vox_bytes[size_index + 20 +3+ 4 + 12 + 4+4*voxel];
            chunk.xyzi.push(VoxCubes::from(x,y,z,i));
        }
        //size_index = size_index + 20 +3+ 4 + 12 + 4+4*(product as usize-1)+1;
        size_index += 36 + byte_size;
        vox.chunks.push(chunk)
    }
    vox.number_of_models = vox.chunks.len();
    //println!("{:?}", vox.chunks);
    let mut buf = 0;
    let mut nodes = Vec::new();
    while vox_bytes[size_index]==110{
        buf = vox_bytes[size_index+4] as usize+256*vox_bytes[size_index+5] as usize;
        let mut b = Vec::new();
        for x in 0..buf+9{
            b.push(&vox_bytes[size_index+4+x]);
        }
        match vox_bytes[size_index+1] {
            //S hape
            0x53 => nodes.push(Node::SHP(Shp::from_bytes(b.clone()))),
            //G roup
            0x47 => nodes.push(Node::GRP(Grp::from_bytes(b.clone()))),
            //T ransform
            0x54 =>nodes.push(Node::TRN(Trn::from_bytes(b.clone()))),
            _ => return Err(vox_importer_errors::Other(".vox file nXXX is invalid".to_string())),
        }
        size_index+=b.len();
        size_index+=3;
        //println!("{:?}", size_index);
        //println!("{:?}", vox_bytes[size_index]);
        //for x in 1..10{println!("{:?}", vox_bytes[size_index+x]);}
        
    }
    println!("{:?}", nodes);
    if nodes.is_empty(){
    return Err(vox_importer_errors::Other(".vox file is corrupted, NO nXXX data (data about tree structure)".to_string()))
    }
    //RGBA find_RGBA_in_(allthefile)
    let rgba_index = find_x_in_y(&[0x52,0x47,0x42,0x41], &vox_bytes);
    if rgba_index.is_none(){return Err(vox_importer_errors::Other(".vox file is corrupted (NO RGBA TAG)".to_string()))}
    let mut palette = Vec::new();
    palette.push(Rgb{r:0, g:0, b:0});
    for x in 0..256{
        let r = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize];
        let g = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize+1];
        let b = vox_bytes[rgba_index.unwrap()+4+8+4*x as usize+2];
        palette.push(Rgb{r,g,b});
    }
    //MATL
    let mut matl = Vec::new();
    matl.push(Matl::default()); //index 0 is empty
    let matl_index = rgba_index.unwrap() + 1036;
    let mut i = matl_index;
    for x in 1..256{
        let mut m = Matl::default();
        m.rgb = palette[x];
        //Skip MATL bytes
        i+=4;
        //MATL size
        let j = vox_bytes[i];
        i+=8;
        //Id
        m.id = vox_bytes[i];
        i+=4;
        //number of attributes
        let mut n_of_attributes = vox_bytes[i];
        i+=4;
        let mut term = 0.0;
        let mut emit = 0.0;
        let mut flux = 0;
        let mut ldr = 0.0;
        while n_of_attributes >= 1{
            let s = vox_bytes[i];
            i+=4;
            let key = &vox_bytes[i..i+s as usize];
            i+=s as usize;

            let t = vox_bytes[i];
            i+=4;
            let value = &vox_bytes[i..i+t as usize];
            i+=t as usize;

            
            match key{
                //ignore type
                b"_type" => m.id += 0,
                //toughness for roughness map
                b"_rough" => m.roughness += bytes_to_numeric::<f32>(value).unwrap(),
                //Index of Rifraction (ri = ior+1)
                b"_ior" => m.ior += bytes_to_numeric::<f32>(value).unwrap(),
                b"_ri" => m.id += 0,
                //density and phase (ignore)
                b"_d" => m.id += 0,
                b"_g" => m.id += 0,
                //The three horsemen of destruction
                b"_emit" => emit += bytes_to_numeric::<f32>(value).unwrap(),
                b"_flux" => flux += bytes_to_numeric::<i32>(value).unwrap(),
                b"_ldr" => ldr += bytes_to_numeric::<f32>(value).unwrap(),
                //Transparency for transparency map
                b"_alpha" => m.transparent += bytes_to_numeric::<f32>(value).unwrap(),
                //Metal
                b"_metal" => m.metallic += bytes_to_numeric::<f32>(value).unwrap(),
                b"_sp" => m.specular += bytes_to_numeric::<f32>(value).unwrap(),
                _ => m.id+=0,
            }
            n_of_attributes -= 1; 
        }
        let initial_luminance = 0.3*(m.rgb.r as f32) + 0.59*(m.rgb.r as f32) +0.11*(m.rgb.b as f32);
        term = -0.5*((x as f32-127.5).abs())+63.75;
        let mut delta_luminance = 0.0;
        if emit != 0.0 || flux != 0 || ldr != 0.0{
            if ldr == 0.0{
                delta_luminance = 3.75_f32.powi(flux)*term*emit;
            }else{
                delta_luminance = ldr * term;
            }
        }
        let final_luminance = initial_luminance+delta_luminance;
        let f = if final_luminance>255.0{
            255_u8
        }else{
            final_luminance.round() as u8
        };
        let ratio = f as f32/initial_luminance;
        if delta_luminance != 0.0{
        m.rgb_e = Some(Rgb{ r:(m.rgb.r as f32*ratio).floor() as u8, 
                            g:(m.rgb.g as f32*ratio).floor() as u8, 
                            b:(m.rgb.b as f32*ratio).floor() as u8 })
        }
        matl.push(m);
    }
    //println!("{:?}",matl);
    //println!("{:?}", palette);
    vox.colours = palette;
    vox.nodes = nodes;
    vox.materials = matl;
    for c in 0..vox.chunks.len(){
    println!("Size: {:?}, Position{:?}, Rotation{:?}", vox.chunks[c].size, vox.chunks[c].position, vox.chunks[c].rotation);
    }
    vox.update_nodes();
    for c in 0..vox.chunks.len(){
    println!("Size: {:?}, Position{:?}, Rotation{:?}", vox.chunks[c].size, vox.chunks[c].position, vox.chunks[c].rotation);
    }
    //dbg!(&vox);
    Ok(vox)
}
fn bytes_to_numeric<T>(bytes: &[u8]) -> Option<T> where T:std::str::FromStr{
    if let Ok(str_value) = std::str::from_utf8(bytes){
        if let Ok(numeric_value) = str_value.parse::<T>(){
            return Some(numeric_value)
        }
    }
    None
}

fn find_x_in_y(x: &[u8], y: &[u8]) -> Option<usize> {
    for (index, window) in y.windows(x.len()).enumerate(){
        if window == x{
            return Some(index);
        }
    }
    None
}
fn split_into_words(input: &[u8]) -> Vec<&[u8]>{
    input.split(|&x| x==b' ').collect()
}
fn find_next_x(bytes: &[u8], x: &[u8]) -> Option<usize>{bytes.windows(x.len()).position(|window| window == x)}
fn find_next_space_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b' ')}
fn find_next_newline_after_index(bytes: &[u8]) -> Option<usize> {bytes.iter().position(|&x| x==b'\n')}
pub fn is_made_by_ephtracy(ply: ply) -> bool { if ply.exported_by == "comment : MagicaVoxel @ Ephtracy"{true} else {false}}
fn column_times_matrix(n: Vector3, m: (Vector3,Vector3,Vector3))->Vector3{
    let a = ((m.0.x*n.x)+(m.0.y*n.y)+(m.0.z*n.z));
    let b = ((m.1.x*n.x)+(m.1.y*n.y)+(m.1.z*n.z));
    let c = ((m.2.x*n.x)+(m.2.y*n.y)+(m.2.z*n.z));
    Vector3::from_tuple((a,b,c))
}
