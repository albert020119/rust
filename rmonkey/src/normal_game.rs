pub mod game{
    use crate::text_generator::text_gen::get_text as get_text;
    use crate::wpm_counter::wpm_counter::WpmCounter as WpmCounter;
    use std::io; 
    use std::io::Write; 
    use crossterm::style::Stylize;
    use crossterm::terminal; 
    use crossterm::terminal::ClearType; 
    use crossterm::{
        cursor::{
            MoveDown, MoveUp, MoveToPreviousLine, RestorePosition, SavePosition, MoveTo, position 
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
        pub fn start(time_limit: i64){
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

            let char_vector: Vec<char> = test_string.chars().collect();

            let mut position_stack: Vec<(u16, u16)> = Vec::new();
            position_stack.push(position().unwrap());

            let mut index: usize = 0; 
            
            let mut first: bool = true;
            let mut counter = WpmCounter::new(time_limit.try_into().unwrap()); 
            loop {

                let ev = read();

                if first {
                    counter.start();
                    first = false; 
                } 
                
                let code = match ev {
                    Ok(Event::Key(key_event)) => {
                        if key_event.kind == KeyEventKind::Release {
                            continue
                        }
                        key_event.code
                    },
                    _ => continue,
                };

                counter.typed(); 
                counter.refresh();
                match code{
                    KeyCode::Esc => { 
                        break;
                    },
                    KeyCode::Enter => {
                        break 
                    },
                    KeyCode::Char(ch) => {
                        position_stack.push(position().unwrap());
                        if ch == char_vector[index] {
                            print!("{}", ch.green());
                            io::stdout().flush().unwrap();
                        }  else {
                            print!("{}", char_vector[index].red());
                            io::stdout().flush().unwrap();
                        } 
                        index += 1;

                        if (index == char_vector.len()) | counter.is_finished(){
                            println!("\nCongrats!\nPress ESC to return to the menu");
                            break; 
                        }
                    },
                    KeyCode::Backspace => {
                        let (x, y)= position_stack.pop().unwrap();
                        let _ = io::stdout().execute(
                            MoveTo(x, y)
                         );
                        index-=1; 
                    },
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }

            }
            loop{
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
                    }
                    _ => {}
                }
            }
        }

        
    }
}