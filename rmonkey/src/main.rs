use std::io; 
use crossterm::{
    cursor::{
        DisableBlinking
    },
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode},
};

mod text_generator; 

mod options;
use options::options::PossibleOptions as PossibleOptions;

mod option_menu;
use option_menu::menu::select_option as select_option;
use option_menu::menu::clean_up as clean_up;

mod game_session; 
use game_session::game_session::GameSession as GameSession; 

mod normal_game;

mod wpm_counter; 


fn main() -> io::Result<()> {
    let _ = io::stdout().execute(
        DisableBlinking
    ); 

    enable_raw_mode()?;
    loop{
        let option = select_option();
        match option{
            PossibleOptions::Exit => {
                clean_up(); 
                break;
            },
            PossibleOptions::Start => {
                let session = GameSession::new(PossibleOptions::Start);
                session.init();
                session.start(); 
            },
            PossibleOptions::TimeRun30 => {
                let session = GameSession::new(PossibleOptions::TimeRun30);
                session.init();
                session.start();
            },
            PossibleOptions::TimeRun60 => {
                let session = GameSession::new(PossibleOptions::TimeRun60);
                session.init();
                session.start();
            }
            _ => todo!(),       
        };
    }
    disable_raw_mode()
}