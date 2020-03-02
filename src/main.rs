extern crate byteorder;
use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt,
WriteByteExt};

fn main() {
    let binary_nums = vec![2, 3, 12, 8, 5, 0];
    let mut buff = Cursor::new(binary_nums);
    let first_byte = buff.read_u8().expect("Failed to read byte");
    println!("first byte in binary {:b}", first_byte);

    let second_byte = buff.read_u8().expect("Failed to read byte");
    println!("second byte as int: {}", second_byte);

    buff.write_u8(123).expect("Failed to overwrite a byte");
    println!("After: {:?}", buff);

    println!("Old position: {}", buff.position());
    buff.set_position(0);
    println!("New position: {}", buff.position());

    buff.seek(SeekFrom::End(0)).expect("Faile to seek end");
    println!("Last position: {}", buff.position());

    buff.set_position(0);

    let as_u32 = buff.read_u32::<LittleEndian>().expect("Failed to read bytes");
    println!("First four bytes as u32 in little endian order:\t{}",
    as_u32);

    buff.set_position(0);
    let as_u32 = buff.read_u32::<BigEndian>().expect("Failed to read bytes");
    println!("First four bytes as u32 in big endian order:\t{}",
    as_u32);

    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    buff.write_f32::<LittleEndian>(-33.4).expect("Failed to write to end");

    let mut read_buff = [0; 5];
    buff.set_position(0);
    buff.read_u16_into::<LittleEndian>(&mut read_buff).expect("Failed to read all bytes");
    println!("All bytes as u16s in little endian order: {:?}", read_buff);
}