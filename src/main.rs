#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, fs::{File, self}, io::BufReader, cmp::{max, min}};

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
    use utm::to_utm_wgs84;
    let (mut northing, mut easting, meridian_convergence) = to_utm_wgs84(lat, lng, 52);
    let northingf32 = (northing as f32 -4071260.5f32)/70000f32; 
    let eastingf32 = (easting as f32 - 276782f32)/70000f32;
    return const_vec2!([(eastingf32) * screen_width() ,(1.0f32-northingf32 + 0.5) * screen_height()]);
}



#[macroquad::main("MetRust")]
async fn main() {
    let LINE_COLOR : Vec<Color> = vec![
        color_u8!(0,0,0,0),
        color_u8!(0, 82, 164, 255),
        color_u8!(0, 168, 77, 255),
        color_u8!(239, 124, 28, 255),
        color_u8!(0, 164, 227, 255),
        color_u8!(153, 108, 172, 255),
        color_u8!(205, 124, 47, 255),
        color_u8!(116, 127, 0, 255),
        color_u8!(230, 24, 108, 255),
        color_u8!(189, 176, 146, 255) // NEED EXPAND
    ];
    let mut STATIONS : Vec<Station> = vec![]; 
    let mut INDEXOFSTATION : HashMap<String,usize> = HashMap::new(); 
    let mut CONNECTED : Vec<Vec<usize>> = Vec::with_capacity(STATIONS.len()); 
    let reader = BufReader::new(File::open("stations.txt").unwrap());
    for stop in iter_json_array(reader) {
        if stop.is_ok(){
            STATIONS.push(stop.unwrap());
            INDEXOFSTATION.insert(STATIONS[STATIONS.len()-1].name.clone(), STATIONS.len()-1);
        }
    }

    request_new_screen_size(720.0f32, 450.0f32);
    loop{
        clear_background(WHITE);
        for stop in STATIONS.iter(){
            let pos = getXY(stop.lat,stop.lng); 
            //println!("{} : {}",stop.line,stop.name);
            draw_circle(pos.x, pos.y, 2.0f32, match stop.line.as_str() {
                "01호선" => LINE_COLOR[1],
                "02호선" => LINE_COLOR[2],
                "03호선" => LINE_COLOR[3],
                "04호선" => LINE_COLOR[4],
                "05호선" => LINE_COLOR[5],
                "06호선" => LINE_COLOR[6],
                "07호선" => LINE_COLOR[7],
                "08호선" => LINE_COLOR[8],
                "09호선" => LINE_COLOR[9],
                _ => Color::new(0f32,0f32,0f32,0f32),
            }); 
        }

        next_frame().await;
    }
}
