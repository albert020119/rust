use clap::Parser;
use std::io::Read;
use std::io::Write;   


#[derive(Parser)]
struct CliArgs{
    path: std::path::PathBuf,
    key: String,
    out_path: std::path::PathBuf
}

fn xor_file (mut fd: std::fs::File, metad: std::fs::Metadata, key: String) -> Result<Vec<u8>, String>{
    let mut buf = vec![0; metad.len().try_into().unwrap()];

    let _ = fd.read_exact(&mut buf);

    let mut dst: Vec<_> = buf.chunks_mut(key.len()).collect();
    for mut chunk in dst.iter_mut(){
        let key_iter = key.as_bytes().iter(); 
        for (pos, key_val) in key_iter.enumerate(){
            if pos >= chunk.len() {
                break;
            }
            chunk[pos] = chunk[pos] ^ key_val;
        }
    }

    return Ok(buf);
}

fn main() {

    let args = CliArgs::parse();

    let path_exists = std::path::Path::new(&args.path).exists();
    match path_exists {
        false => {
            println!("given path does not exist");
        },
        _ => {}
    }

    let f = match std::fs::File::open(args.path) {
        Ok(file) => file,
        Err(err) => {
            println!("could not open file: {}", err);
            std::process::exit(1);
        }
    };

    let metad = match f.metadata(){
        Ok(metad) => metad,
        Err(err) => {
            println!("could not obtain metadata for file");
            std::process::exit(1);
        }
    };

    println!("xoring {size} bytes of data...", size=metad.len());

    let result = match xor_file(f, metad, args.key){
        Ok(buf) => {
            println!("successfully xored file contents");
            buf
        },
        Err(err) => {
            println!("could not obtain metadata for file");
            std::process::exit(1);
        }
    };

    println!("writing results to output file {}", args.out_path.display()); 
    
    let mut f = std::fs::File::create(args.out_path).expect("Unable to create file");
    f.write_all(&result).expect("Unable to write data");
}
