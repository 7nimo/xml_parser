use std::{
    env,
    error::Error,
    fs::File,
    io::BufReader,
    str,
};

use zip::ZipArchive;

use encoding_rs_io::DecodeReaderBytes;

use quick_xml::{Reader, events::{BytesStart, Event}};

const BUF_SIZE: usize = 4096; // 4kb at once

struct Hotel {
    id: u32,
    region_id: u16,
    country_id: u16,
    city_id: u16,
    standard: u8
}

// fn read_id(ev: Event) -> &str {
//     match ev {
//         Event::Start(e) => match e.local_name() {
//             b"hotel" => {
//                 let value = str::from_utf8(&e.attributes().nth(0).unwrap().unwrap().value).unwrap();
//                 return value;
//             }
//         }
        
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("no filename provided")?;
    let zipfile = File::open(path)?;
    let mut zip = ZipArchive::new(zipfile)?;

    if zip.len() != 1 {
        Err("expected one file in zip archive")?;
    }

    let xmlfile = zip.by_index(0)?;
    println!("file is {}, size {} bytes", xmlfile.name(), xmlfile.size());
    let xmlfile = BufReader::new(DecodeReaderBytes::new(xmlfile));
    let mut xmlfile = Reader::from_reader(xmlfile);

    let mut buf = Vec::with_capacity(BUF_SIZE);
    loop {
        match xmlfile.read_event(&mut buf)? {
            Event::Start(e) => match e.local_name() {

                b"hotel" => {
                    println!("id: {:?}", str::from_utf8(&e.attributes().nth(0).unwrap().unwrap().value).unwrap());
                },
                b"city" => { },
                b"region" => { },
                b"country" => { },
                b"standard" => { },
                _ => { }
            },

            Event::End(e) => match e.local_name() {
                b"hotel" => {
                    println!("end {}", str::from_utf8(e.local_name())?);
                },
                _ => { }
            },

            Event::Eof => break,

            _ => { },
        };
        buf.clear();
    }

    Ok(())
}