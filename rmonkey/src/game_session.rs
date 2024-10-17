pub mod game_session{
    use crossterm::terminal; 
    use crossterm::terminal::ClearType;
    use crossterm::ExecutableCommand; 
    use std::io; 
    use crate::PossibleOptions;
    use crate::normal_game::game::Game;

    enum GameType{
        NormalGame
    }

    pub struct GameSession{
        game_type: GameType
    }

    impl GameSession{
        pub fn new(possible_option: PossibleOptions) -> Self{
            let game_type = match possible_option {
                PossibleOptions::Start => GameType::NormalGame, 
                _ => todo!(),
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
                GameType::NormalGame => Game::start(),
                _ => todo!(),
            }
        }

        fn clean_up(){   
            let _ = io::stdout().execute(terminal::Clear(terminal::ClearType::All));
        }
    }

} 