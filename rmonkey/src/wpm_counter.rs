pub mod wpm_counter{
    use std::time::{Duration, SystemTime};
    use std::io; 
    use crossterm::{
        cursor::{
            MoveDown, MoveUp, MoveToPreviousLine, RestorePosition, SavePosition, MoveTo, position 
        },
        event::{
            read, Event, KeyCode, KeyEventKind
        },
        ExecutableCommand
    };
    pub struct WpmCounter{
        chars_typed: u64,
        start_time: SystemTime, 
        time_elapsed: u64,
        current_wpm: u64
    }

    const COUNTER_POS_X: u16 = 0;
    const COUNTER_POS_Y: u16 = 5; 
    impl WpmCounter{
        pub fn new() -> Self{
            WpmCounter{
                chars_typed: 0, 
                start_time: SystemTime::now(), 
                time_elapsed: 0, 
                current_wpm: 0
            }
        }

        pub fn typed(&mut self){
            self.chars_typed += 1; 
        }

        pub fn start(&mut self){
            self.start_time = SystemTime::now();
        }
        
        pub fn refresh(&mut self){
            let _ = io::stdout().execute(
                SavePosition
            );

            let _ = io::stdout().execute(
                MoveTo(COUNTER_POS_X, COUNTER_POS_Y)
             );

            match self.start_time.elapsed() {
                Ok(elapsed) => {
                    let words_typed: u64 = self.chars_typed / 5; 
                    if elapsed.as_secs() > 0{
                        self.current_wpm = (60 * words_typed) / elapsed.as_secs();
                        println!("Current: WPM {}", self.current_wpm);
                    }
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
            let _ = io::stdout().execute(
                RestorePosition
             );

        }
    }

}
