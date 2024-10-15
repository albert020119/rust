use std::io; 
use crossterm::{
    cursor::{
        DisableBlinking
    },
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode},
};


mod options;
use options::options::PossibleOptions as PossibleOptions;
mod option_menu;
use option_menu::menu::select_option as select_option; 
mod text_generator;
use text_generator::text_gen::get_text as get_text;  

fn main() -> io::Result<()> {
    let _ = io::stdout().execute(
        DisableBlinking
    ); 

    enable_raw_mode()?;

    let option = select_option();
    match option{
        PossibleOptions::Exit => std::process::exit(0),
        PossibleOptions::Start => {
            println!("{:?}",get_text().unwrap());
        },
        _ => todo!(),       
    };

    disable_raw_mode()
}