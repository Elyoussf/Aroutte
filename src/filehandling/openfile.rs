

use std::{env, error::Error, fs::File};

//>

#[derive(Debug)]
pub struct MyError{
    details : String
}

impl MyError{
    fn new(msg : &str) -> MyError{
        MyError{
            details: msg.to_string()
        }
    }
}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    
}

pub fn analyzefilename() -> Result<File,MyError>{ // choosen Error trait to alert the existence of the error rather than specifying the error
    let file_name = env::args().nth(1);
    match file_name {
       None => {
        Err(MyError::new("No filename specified"))
       },
       Some(v)=>{
        let file = File::open(v);
        if let Ok(f) = file {
            return Ok(f)
        }else{
            return Err(MyError::new("No existed file with this name"));
        }
       }
    }
}