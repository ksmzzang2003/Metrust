#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, fs::{File, self}, io::BufReader};

use macroquad::prelude::*;

struct Train{
    line : i32,
    orientation : bool, 
    destination : i32, 
    prev : i32,
    location : Vec2, 
    velocity : Vec2,
    capacity : i32,  
    heading : Vec<i32>, 
    embarked : i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Station {
    line : String,
    name : String,
    code : i64, 
    lat : f64,
    lng : f64,
}


use serde::de::DeserializeOwned;
use serde_json::{self, Deserializer};
use std::io::{self, Read};

fn read_skipping_ws(mut reader: impl Read) -> io::Result<u8> {
    loop {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;
        if !byte.is_ascii_whitespace() {
            return Ok(byte);
        }
    }
}

fn invalid_data(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

fn deserialize_single<T: DeserializeOwned, R: Read>(reader: R) -> io::Result<T> {
    let next_obj = Deserializer::from_reader(reader).into_iter::<T>().next();
    match next_obj {
        Some(result) => result.map_err(Into::into),
        None => Err(invalid_data("premature EOF")),
    }
}

fn yield_next_obj<T: DeserializeOwned, R: Read>(
    mut reader: R,
    at_start: &mut bool,
) -> io::Result<Option<T>> {
    if !*at_start {
        *at_start = true;
        if read_skipping_ws(&mut reader)? == b'[' {
            // read the next char to see if the array is empty
            let peek = read_skipping_ws(&mut reader)?;
            if peek == b']' {
                Ok(None)
            } else {
                deserialize_single(io::Cursor::new([peek]).chain(reader)).map(Some)
            }
        } else {
            Err(invalid_data("`[` not found"))
        }
    } else {
        match read_skipping_ws(&mut reader)? {
            b',' => deserialize_single(reader).map(Some),
            b']' => Ok(None),
            _ => Err(invalid_data("`,` or `]` not found")),
        }
    }
}

pub fn iter_json_array<T: DeserializeOwned, R: Read>(
    mut reader: R,
) -> impl Iterator<Item = Result<T, io::Error>> {
    let mut at_start = false;
    std::iter::from_fn(move || yield_next_obj(&mut reader, &mut at_start).transpose())
}
pub fn getXY(lat : f64, lng : f64) -> Vec2 {
    return const_vec2!([(lng as f32 + 180f32) * (screen_width() as f32 / 360f32),(-lat as f32  + 90f32) * (screen_height() as f32/ 180f32)]); 
}



#[macroquad::main("MetRust")]
async fn main() {
    let mut LINE_COLOR : Vec<Color> = vec![
        Color::new(0f32,0f32,0f32,0f32),
        Color::new(0f32, 82f32, 164f32, 1f32),
        Color::new(0f32, 168f32, 77f32, 1f32),
        Color::new(239f32, 124f32, 28f32, 1f32),
        Color::new(0f32, 164f32, 227f32, 1f32),
        Color::new(153f32, 108f32, 172f32, 1f32),
        Color::new(205f32, 124f32, 47f32, 1f32),
        Color::new(116f32, 127f32, 0f32, 1f32),
        Color::new(230f32, 24f32, 108f32, 1f32),
        Color::new(189f32, 176f32, 146f32, 1f32)
    ];
    let mut STATIONS : Vec<Station> = vec![]; 
    let reader = BufReader::new(File::open("stations.txt").unwrap());
    for user in iter_json_array(reader) {
        if user.is_ok(){
            let user: Station = user.unwrap();
            STATIONS.push(user);
            //println!("{:?}", user);
        }
    }

    request_new_screen_size(720.0f32, 450.0f32);
    loop {

        next_frame().await;
    }
}
