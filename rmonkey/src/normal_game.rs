pub mod game{
    use crate::text_generator::text_gen::get_text as get_text;
    use std::io; 
    use std::io::Write; 
    use crossterm::terminal; 
    use crossterm::terminal::ClearType; 
    use crossterm::{
        cursor::{
            MoveDown, MoveUp, MoveToPreviousLine, RestorePosition, SavePosition, MoveTo
        },
        event::{
            read, Event, KeyCode, KeyEventKind
        },
        ExecutableCommand
    };
    pub struct Game{
        test_string: String 
    }

    impl Game{
        pub fn start(){
            let _ = io::stdout().execute(terminal::Clear(terminal::ClearType::All));
            let mut test_string: String = get_text().unwrap();

            let _ = io::stdout().execute(
                MoveTo(0, 0)
            );

            let _ = io::stdout().execute(
                SavePosition
            );

            print!("{}", test_string);
            io::stdout().flush().unwrap();

            let _ = io::stdout().execute(
                RestorePosition
             );
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
                    },
                    KeyCode::Down => {
                        let _ = io::stdout().execute(
                            MoveDown(1)
                        );
                    },
                    KeyCode::Enter => {
                        break 
                    },
                    _ => {}
                }
            }
        }
    }
}