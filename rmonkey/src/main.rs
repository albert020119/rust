use std::io;
use std::io::Write;
use std::mem;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::terminal;
use crossterm::style::Stylize; 
use crossterm::{
    cursor::{
        MoveDown, MoveUp, MoveLeft, MoveRight, DisableBlinking, Hide
    },
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind
    },
    execute,
    queue,
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::time::Duration;

fn listen_for_keys() -> io::Result<()> {
    loop {
        // Blocking read
        let ev = read()?;
        let _ = match ev {
            Event::Key(kev) => {
                let mut c = match kev.code {
                    KeyCode::Char(ch) => ch,
                    _ => '\0',
                };
                if kev.kind != KeyEventKind::Release{
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
            },
            _ => todo!(),
        };

        if ev == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    Ok(())
}

#[derive(Debug, EnumIter, PartialEq)]
enum PossibleOptions{
    Exit,
    Start,
    TimeRun30,
    TimeRun60
}

impl PossibleOptions{
    fn get_string(&self) -> String{
        match &self{
            PossibleOptions::Exit => String::from("Press ESC to exit\n"),
            PossibleOptions::Start => String::from("start game\n"),
            PossibleOptions::TimeRun30 => String::from("time run 30s\n"),
            PossibleOptions::TimeRun60 => String::from("time run 60s\n")
        }
    }

    fn get_all() -> Vec<String>{
        let mut strings = vec![]; 
        for op in PossibleOptions::iter(){
            strings.push(op.get_string())
        }
        return strings;
    }

    fn count() -> u8{
        let mut count: u8 = 0; 
        for op in PossibleOptions::iter(){
            count += 1; 
        }
        return count; 

    }

}

struct Menu{
    pub selected_option: PossibleOptions,    
} 

impl Menu {
    fn print_initial(&self){
        let options: Vec<String> = PossibleOptions::get_all();
        for option in options{
            print!("{}", option);
        }
        io::stdout().execute(
            MoveUp(PossibleOptions::count().into())
        );
        
    }

    fn refresh(&self){
        io::stdout().execute(terminal::Clear(terminal::ClearType::All));

        for option in PossibleOptions::iter(){
            if option == self.selected_option {
                print!(">{}", option.get_string().green())
            } else{ 
                print!("{}", option.get_string())
            }
        }
    }

    fn go_up(&mut self){
        let mut previous: PossibleOptions = PossibleOptions::TimeRun60; 
        for op in PossibleOptions::iter(){
            if op == self.selected_option {
                self.selected_option = previous;
                break;  
            }
            previous = op; 
        }
        self.refresh(); 
    }

    fn go_down(&mut self){
        let mut next: bool = false;
        for op in PossibleOptions::iter(){
            if next {
                self.selected_option = op;
                next = false; 
                break;  
            }
            if op == self.selected_option {
                next = true; 
            }
        };
        if next{
            for op in PossibleOptions::iter(){
                self.selected_option = op;
                break;
            };
        }
        self.refresh(); 
    }
}

fn select_option() -> PossibleOptions{
    let mut exit = false; 
    let mut selection = PossibleOptions::Start;
    let mut menu = Menu{
        selected_option: selection
    };
    menu.print_initial();
    loop {
        let mut ev = read();
        
        let code = match ev {
            Ok(Event::Key(key_event)) => {
                if key_event.kind == KeyEventKind::Release {
                    continue
                }
                key_event.code
            },
            _ => continue,
        };
        
        match code{
            KeyCode::Esc => {
                exit = true; 
                break;
            },
            KeyCode::Up => {
                io::stdout().execute(
                    MoveUp(1)
                );
                menu.go_up();
            },
            KeyCode::Down => {
                io::stdout().execute(
                    MoveDown(1)
                );
                menu.go_down();
            },
            KeyCode::Enter => {
                break
            },
            _ => {}
        }
    }

    menu.selected_option
}


fn main() -> io::Result<()> {
    io::stdout().execute(
        DisableBlinking
    ); 

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let option = select_option();
    match option{
        PossibleOptions::Exit => std::process::exit(0),
        _ => {}      
    };

    if let Err(e) = listen_for_keys() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}