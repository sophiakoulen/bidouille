use std::env;
use std::fs;
use std::process::exit;

struct WavFile{
    filesize: u32,
    datasize: u32,
    header: Vec<u8>,
    data: Vec<u8>,
}

fn parse_file(bytes: Vec<u8>)->Option<WavFile>{
    if &bytes[0..4] != b"RIFF"
    {
        println!("File format not recognized.");
        return None;
    }
    
    if &bytes[8..12] != b"WAVE"
    {
        println!("File format not recognized.");
        return None;
    }

    let filesize: &[u8;4] = &bytes[4..8].try_into().unwrap();
    let filesize = u32::from_le_bytes(*filesize);
    
    if &bytes[70..74] != b"data"
    {
        println!("File format not recognized.");
        return None;
    }

    let header = bytes[12..70].to_vec();

    let datasize: &[u8;4] = &bytes[74..78].try_into().unwrap();
    let datasize = u32::from_le_bytes(*datasize);

    let data = bytes[78..].to_vec();

    Some(WavFile {filesize, datasize, header, data})
}

fn concat_WavFile(a: &WavFile, b: &WavFile)->Option<WavFile>
{
    let filesize = a.filesize + b.datasize;
    let datasize = a.datasize + b.datasize;

    let header = a.header.clone();
    let data = [a.data.to_vec(), b.data.to_vec()].concat();

    Some(WavFile{filesize, datasize, header, data})
}

fn WavFile_to_bytes(file: &WavFile)->Vec<u8>
{
    vec![
        b"RIFF".to_vec(),
        file.filesize.to_le_bytes().to_vec(),
        b"WAVE".to_vec(),
        file.header.to_vec(),
        b"data".to_vec(),
        file.data.to_vec(),
    ].concat()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Should have 1 or 2 arguments.\n");
        exit(2);
    }

    let file1 = &args[1];
    let bytes1 = fs::read(&file1).unwrap();
    let content1 = parse_file(bytes1).unwrap();

    let result;

    if args.len() > 2
    {
        
        let file2 = &args[2];
        let bytes2 = fs::read(&file2).unwrap();
        let content2 = parse_file(bytes2).unwrap();
        result = concat_WavFile(&content1, &content2).unwrap();
    }
    else
    {
        result = concat_WavFile(&content1, &content1).unwrap();
    }

    let output_bytes = WavFile_to_bytes(&result);

    fs::write("output.wav", &output_bytes).unwrap();
}
