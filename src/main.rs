use std::{
    env,
    error::Error,
    fs::File,
    io::BufReader,
    str,
};

use zip::ZipArchive;

use encoding_rs_io::DecodeReaderBytes;

use quick_xml::{
    events::Event,
    Reader,
};

const BUF_SIZE: usize = 4096; // 4kb at once

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
                    println!("start {}", str::from_utf8(e.local_name())?);


                    // todo: if has attributes, execute for below
                    println!("id: {:?}", str::from_utf8(&e.attributes().nth(0).unwrap().unwrap().value).unwrap())

                    // ? in case more than one attribute needed
                    // let attributes =  e.attributes();
                    // let mut count = 0;
                    // for attr in attributes {
                    //     count += 1;
                    //     println!("{}: ", count);
                    //     // todo: extract first attr instead of looping
                    //     let key = str::from_utf8(&attr.key).unwrap();
                    //     if key.eq("id")  {
                    //         print!("Id: {:?}\n", str::from_utf8(&attr.value).unwrap());
                    //     }
                    // }
                },
                _ => { }
            },

            Event::End(e) => match e.local_name() {
                b"hotel" => {
                    println!("end {}", str::from_utf8(e.local_name())?);
                },
                _ => { }
            },

            // Event::Text(e) => {
            //     println!("text: {}", str::from_utf8(&e.unescaped()?)?);
            // },

            Event::Eof => break,

            _ => { },
        };
        buf.clear();
    }

    Ok(())
}

// fn main() {
//     let text = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
//     let doc = roxmltree::Document::parse(&text).unwrap();

//     let mut elements = 0;

//     for node in doc.root().descendants().filter(|n| n.is_element()) {
//         elements += 1;
//         println!("Element: {:?}, Id: {:?}",node.tag_name(), node.attribute("id").unwrap_or("None"));

//         if elements == 10 {
//             break;
//         }

//     }

//     println!("Elements: {}", elements);
// }