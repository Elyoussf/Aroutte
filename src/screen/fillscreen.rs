use std::{fs:: File, io::BufRead};

use std::io::{stdout, Write};
use crossterm::style::Print;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
  
};
pub fn fillterminal(f : File){
   
    let reader = std::io::BufReader::new(f);
    let mut whole_text = String::new();
    for line in reader.lines(){
        if let Ok(l) = line{
            
            whole_text.push_str(&l);
            whole_text.push_str("\n");
            whole_text.push_str("\n");
        }else{
            println!("Error occured while reading a line from the buffer");
        }
        stdout().flush().unwrap()
    }
    write_into_terminal(whole_text);
    
}

fn write_into_terminal(whole_text : String)  {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    ).unwrap();
    for (i,line) in whole_text.split("\n").enumerate(){
        execute!(
            stdout,
            cursor::MoveTo(0,i as u16),
            Print(format!("{}",line)),
        ).unwrap()
    }
   
    
    
    // execute!(
    //     stdout,
    //     cursor::MoveTo(0, 11),
    //     Print("Press Enter to exit..."),
    // ).unwrap();
    // let _ = std::io::stdin().read_line(&mut String::new());
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();
   
}
