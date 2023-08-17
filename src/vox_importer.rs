use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
//Ply reader without using external libraries

//Reads the
//
//
fn read_ply(filepath: String) -> Result<String, io::Error>{
    let mut result = String::new();
    File::open(filepath)
}
