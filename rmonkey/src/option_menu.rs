#[path = "options.rs"] mod options; 

pub mod menu{
    use crate::PossibleOptions; 
    use std::io;
    use strum::IntoEnumIterator; 
    use crossterm::terminal;
    use crossterm::style::Stylize; 
    use crossterm::{
        cursor::{
            MoveDown, MoveUp
        },
        event::{
            read, Event, KeyCode, KeyEventKind
        },
        ExecutableCommand
    };
    pub struct Menu{
        pub selected_option: PossibleOptions,    
    } 
    
    impl Menu {
        fn print_initial(&self){
            let options: Vec<String> = PossibleOptions::get_all();
            for option in options{
                print!("{}", option);
            }
            let _ = io::stdout().execute(
                MoveUp(PossibleOptions::count().into())
            );
            
        }
    
        fn refresh(&self){
            let _ = io::stdout().execute(terminal::Clear(terminal::ClearType::All));
    
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
    
    pub fn select_option() -> PossibleOptions{
        let selection = PossibleOptions::Start;
        let mut menu = Menu{
            selected_option: selection
        };
        menu.print_initial();
        loop {
            let ev = read();
            
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
                    break;
                },
                KeyCode::Up => {
                    let _ = io::stdout().execute(
                        MoveUp(1)
                    );
                    menu.go_up();
                },
                KeyCode::Down => {
                    let _ = io::stdout().execute(
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

    pub fn clean_up(){   
        let _ = io::stdout().execute(terminal::Clear(terminal::ClearType::All));
    }
}