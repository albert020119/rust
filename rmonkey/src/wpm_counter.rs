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
        current_wpm: u64,
        time_limit: u64,
        finished: bool
    }

    const COUNTER_POS_X: u16 = 0;
    const COUNTER_POS_Y: u16 = 5; 
    impl WpmCounter{
        pub fn new(time_limit: u64) -> Self{
            WpmCounter{
                chars_typed: 0, 
                start_time: SystemTime::now(), 
                time_elapsed: 0, 
                current_wpm: 0,
                time_limit: time_limit,
                finished: false
            }
        }

        pub fn typed(&mut self){
            self.chars_typed += 1; 
        }

        pub fn is_finished(&mut self) -> bool{
            return self.finished;
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
                        let words_typed: u64 = self.chars_typed / 5; 
                        self.current_wpm = (60 * words_typed) / elapsed.as_secs();
                        println!("Current: {} WPM ", self.current_wpm);
                        println!("Chars typed: {}", self.chars_typed); 
                        println!("Words typed: {}", words_typed); 
                        println!("Elapsed: {}", elapsed.as_secs()); 

                        if (self.time_limit < elapsed.as_secs()) & (self.time_limit != 0){
                            self.finished = true; 
                        } 
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
