use hex;
use sha1::{Digest, Sha1};
use std::error::Error;
use std::io::Cursor;
use xml::reader::{EventReader, XmlEvent};

fn check_hash(unhashed: &str, validation_hash: &str) -> Result<bool, Box<dyn Error>> {
    let mut hasher = Sha1::new();
    hasher.update(unhashed.as_bytes());
    let res = hasher.finalize();
    Ok(res.to_vec() == hex::decode(validation_hash)?)
}

async fn download(url: &str, location: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(location)?;
    let res = reqwest::get(url).await?;
    let mut content = Cursor::new(res.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

fn init() -> Result<(), Box<dyn Error>> {
    if !std::path::Path::is_dir(std::path::Path::new("./binaries")) {
        std::fs::create_dir("./binaries")?;
    }
    let file = std::fs::read_to_string("./binaries.xml")?;
    let mut binaries = EventReader::new(file.as_bytes());
    loop {
        let current = binaries.next();
        match current {
            Ok(event) => match event {
                XmlEvent::EndDocument => {
                    break;
                }
                XmlEvent::Characters(c) => {
                    println! {"{}", c};
                }
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    println! {"{}, {:?}, {:?}", name, attributes, namespace};
                }
                XmlEvent::EndElement { name } => {
                    println! {"{}", name};
                }
                _ => {
                    continue;
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    todo! {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    todo! {}
}
