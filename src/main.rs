use clap::Parser;
use std::io::Read;  


#[derive(Parser)]
struct CliArgs{
    path: std::path::PathBuf,
    key: String,
}

fn xor_file (mut fd: std::fs::File, metad: std::fs::Metadata, key: String) -> Result<bool, String>{
    let mut buf = vec![0; metad.len().try_into().unwrap()];

    let _ = fd.read_exact(&mut buf);

    let mut dst: Vec<&[u8]> = buf.chunks(key.len()).collect();
    for i in dst.iter_mut(){
        println!("{:?}", i); 
    }
    return Ok(true);
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

    println!("xoring {size} bytes of data...\n",
        size=metad.len()
    );

    let result = match xor_file(f, metad, args.key){
        Ok(_) => {println!("successfully xored files")},
        Err(err) => {
            println!("could not obtain metadata for file");
            std::process::exit(1);
        }
    };
    

}
