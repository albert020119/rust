use clap::Parser; 


#[derive(Parser)]
struct CliArgs{
    path: std::path::PathBuf,
    key: String,
}

fn xor_file(fd: std::fs::File, key: String){
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


    let result = xor_file(f, args.key);
    

}
