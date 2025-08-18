use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let bytes = fs::read(&filename).unwrap();

    let header = &bytes[..78];

    let old_size: &[u8;4] = &bytes[78..82].try_into().unwrap();

    let data = &bytes[82..];

    let old_size = u32::from_le_bytes(*old_size);
    let new_size: u32 = (old_size * 2).try_into().unwrap();
    let new_size = new_size.to_le_bytes();

    dbg!(&new_size);

    let truc = vec![header.to_vec(), new_size.to_vec(), data.to_vec()].concat();

    fs::write("output.wav", &truc).unwrap();


}
