use std::env;
use std::fs;
use std::process::exit;
use std::io::ErrorKind;

struct WavFile {
    filesize: u32,
    datasize: u32,
    header: Vec<u8>,
    data: Vec<u8>,
}

#[derive(Debug)]
enum ParseError {
    FileFormat,
}

fn parse_file(bytes: Vec<u8>)->Result<WavFile, ParseError> {
    if &bytes[0..4] != b"RIFF" {
        return Err(ParseError::FileFormat);
    }
    
    if &bytes[8..12] != b"WAVE" {
        return Err(ParseError::FileFormat);
    }

    let filesize: &[u8;4] = &bytes[4..8].try_into().unwrap();
    let filesize = u32::from_le_bytes(*filesize);
    
    if &bytes[70..74] != b"data" {
        return Err(ParseError::FileFormat);
    }

    let header = bytes[12..70].to_vec();

    let datasize: &[u8;4] = &bytes[74..78].try_into().unwrap();
    let datasize = u32::from_le_bytes(*datasize);

    let data = bytes[78..].to_vec();

    Ok(WavFile {filesize, datasize, header, data})
}

#[derive(Debug)]
enum OperationError {
    ResultingFileTooLarge,
}

fn concat_WavFile(a: &WavFile, b: &WavFile)->Result<WavFile, OperationError> {
    let filesize = a.filesize.checked_add(b.datasize);
    let filesize = match filesize {
        Some(a) => a,
        None => {
            return Err(OperationError::ResultingFileTooLarge);
        }
    };
    let datasize = a.datasize.checked_add(b.datasize);
    let datasize = match datasize {
        Some(a) => a,
        None => {
            return Err(OperationError::ResultingFileTooLarge);
        }
    };

    let header = a.header.clone();
    let data = [a.data.to_vec(), b.data.to_vec()].concat();

    Ok(WavFile{filesize, datasize, header, data})
}

fn WavFile_to_bytes(file: &WavFile)->Vec<u8> {
    vec![
        b"RIFF".to_vec(),
        file.filesize.to_le_bytes().to_vec(),
        b"WAVE".to_vec(),
        file.header.to_vec(),
        b"data".to_vec(),
        file.data.to_vec(),
    ].concat()
}

fn open_WavFile(filename: &str)->WavFile {
    let res = fs::read(filename);
    let bytes = match res{
        Ok(bytes) => bytes,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Error: could not find input file '{filename}'.");
                exit(1);
            }
            else if e.kind() == ErrorKind::PermissionDenied {
                println!("Error: permission denied for input file '{filename}'.");
                exit(1);
            }
            else {
                panic!("Failed to open input file '{filename}': {e:?}.");
            }
        }
    };
    let parse_result = parse_file(bytes);
    let content = match parse_result {
        Ok(content) => content,
        Err(e) => {
            match e {
                ParseError::FileFormat => {
                    println!("Failed to parse input file '{filename}': file format not recognised.");
                    exit(1);
                }
                _ => {
                    println!("Failed to parse input file '{filename}'.");
                    exit(1);
                }
            }
        }
    };
    content
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Should have 1 or 2 arguments.\n");
        exit(2);
    }

    let wav1 = open_WavFile(&args[1]);
    let result;

    if args.len() > 2 {
        let wav2 = open_WavFile(&args[2]);
        result = concat_WavFile(&wav1, &wav2);
    }
    else {
        result = concat_WavFile(&wav1, &wav1);
    }

    let result = match result {
        Ok(res) => res,
        Err(e) => {
            match e {
                OperationError::ResultingFileTooLarge => {
                    println!("Failed to concatenate the input files because the resulting file would be too large.");
                    exit(1);
                },
                _ => {
                    println!("Failed to concatenate the input files.");
                    exit(1);
                }
            };
        }
    };

    let output_bytes = WavFile_to_bytes(&result);
    let output_filename = "output.wav";
    fs::write(output_filename, &output_bytes).expect("Failed to write to output file.");
}
