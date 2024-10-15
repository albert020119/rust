pub mod options{
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter; 

    #[derive(Debug, EnumIter, PartialEq)]
    pub enum PossibleOptions{
        Exit,
        Start,
        TimeRun30,
        TimeRun60
    }

    impl PossibleOptions{
        pub fn get_string(&self) -> String{
            match &self{
                PossibleOptions::Exit => String::from("Press ESC to exit\n"),
                PossibleOptions::Start => String::from("start game\n"),
                PossibleOptions::TimeRun30 => String::from("time run 30s\n"),
                PossibleOptions::TimeRun60 => String::from("time run 60s\n")
            }
        }

        pub fn get_all() -> Vec<String>{
            let mut strings = vec![]; 
            for op in PossibleOptions::iter(){
                strings.push(op.get_string())
            }
            return strings;
        }

        pub fn count() -> u8{
            let mut count: u8 = 0; 
            for _op in PossibleOptions::iter(){
                count += 1; 
            }
            return count; 

        }
    }
}
