extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
struct PetOwner {
    name: String,
    age: u8,
    pets: Vec<Pet>,
}

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    species: AllowedSpecies,
    age: Option<u8>,
    color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum AllowedSpecies {
    Dog,
    Turtle,
    Cat,
}

fn main() {
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("pet_owner.json")
    .expect("failed to create json file");
    
    let buf_writer = BufWriter::new(&file);
    write_json(buf_writer).expect("failed to write json");
    
    let mut buf_reader = BufReader::new(&file);
    buf_reader.seek(SeekFrom::Start(0))
    .expect("failed to jump to the beginning of the JSON file");
    read_json(buf_reader).expect("failed to read json");
}

fn write_json<W>(mut writer: W) -> serde_json::Result<()>
where
W: Write,
{
    let pet_owner = PetOwner {
        name: "John".to_string(),
        age: 23,
        pets: vec![
            Pet{
                name: "Waldo".to_string(),
                species: AllowedSpecies::Dog,
                age: Some(2),
                color: None,
            },
            Pet {
                name: "Speedy".to_string(),
                species: AllowedSpecies::Turtle,
                age: Some(47),
                color: Some("Green".to_string()),
            },
            Pet {
                name: "Meows".to_string(),
                species: AllowedSpecies::Cat,
                age: None,
                color: Some("Orange".to_string())
            },
        ]
    };
    let json = serde_json::to_string(&pet_owner)?;
    writer.write_all(json.as_bytes())
    .expect("failed to write file");
    Ok(())
}