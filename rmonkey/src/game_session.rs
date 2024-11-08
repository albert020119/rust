pub mod game_session{
    use crossterm::terminal; 
    use crossterm::terminal::ClearType;
    use crossterm::ExecutableCommand; 
    use std::io; 
    use crate::PossibleOptions;
    use crate::normal_game::game::Game;

    enum GameType{
        NormalGame,
        TimeRun30,
        TimeRun60
    }

    pub struct GameSession{
        game_type: GameType
    }

    impl GameSession{
        pub fn new(possible_option: PossibleOptions) -> Self{
            let game_type = match possible_option {
                PossibleOptions::Start => GameType::NormalGame, 
                PossibleOptions::TimeRun30 => GameType::TimeRun30,
                PossibleOptions::TimeRun60 => GameType::TimeRun60,
                PossibleOptions::Exit => todo!()
            };
            GameSession{
                game_type: game_type
            }
        }

        pub fn init(&self){
            GameSession::clean_up();

        } 

        pub fn start(&self){
            match self.game_type {
                GameType::NormalGame => Game::start(0),
                GameType::TimeRun30 => Game::start(30),
                GameType::TimeRun60 => Game::start(60)
            }
        }

        fn clean_up(){   
            let _ = io::stdout().execute(terminal::Clear(terminal::ClearType::All));
        }
    }

} 