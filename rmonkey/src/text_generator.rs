extern crate reqwest; 

pub mod text_gen{
    pub fn get_text() -> Result<String, Box<dyn std::error::Error>>{
        let body = reqwest::blocking::get("http://metaphorpsum.com/sentences/3")?.text()?;
        return Ok(body);
    }

}