use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let bytes = fs::read(&filename).unwrap();

    let id1: &[u8;4] = &bytes[0..4].try_into().unwrap();

    let file_size: &[u8;4] = &bytes[4..8].try_into().unwrap();

    let id2: &[u8;4] = &bytes[8..12].try_into().unwrap();

    let header = &bytes[12..74];

    let old_size: &[u8;4] = &bytes[74..78].try_into().unwrap();

    let data = &bytes[78..];

    let old_size = u32::from_le_bytes(*old_size);

    dbg!(&old_size);

    let file_size = u32::from_le_bytes(*file_size);

    dbg!(&file_size);

    let new_file_size: u32 = (file_size + old_size).try_into().unwrap();
    let new_file_size = new_file_size.to_le_bytes();

    let new_size: u32 = (old_size * 2).try_into().unwrap();
    let new_size = new_size.to_le_bytes();

    dbg!(&new_size);

    let truc = vec![id1.to_vec(), new_file_size.to_vec(), id2.to_vec(), header.to_vec(), new_size.to_vec(), data.to_vec(), data.to_vec()].concat();

    fs::write("output.wav", &truc).unwrap();


}
