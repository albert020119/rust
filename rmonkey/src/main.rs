use std::io;
use std::io::Write;
use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
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

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse, focus and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

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
                if (kev.kind != KeyEventKind::Release){
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

#[non_exhaustive]
enum PossibleOptions{
    Exit,
    Start,
    TimeRun30,
    TimeRun60
}

struct Menu{
    pub selected_option: PossibleOptions,    
} 

impl Menu {
    fn print(){
    }

    fn go_up(){
    }

    fn go_down(){
    }
}

fn select_option() -> PossibleOptions{
    println!("Press ESC to exit \n\nstart game \ntime run 30s\ntime run 60s");
    let mut exit = false; 
    let mut selection = PossibleOptions::Start;
    let mut menu = Menu{
        selected_option: selection
    };
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
            },
            KeyCode::Down => {
                io::stdout().execute(
                    MoveDown(1)
                );
            },
            KeyCode::Enter => {
                break
            },
            _ => {}
        }
    }

    if exit {
        std::process::exit(0);
    }

    menu.selected_option
}


fn main() -> io::Result<()> {
    io::stdout().execute(
        DisableBlinking
    ); 
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    match select_option(){
        _ => {}      
    };

    if let Err(e) = listen_for_keys() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}