use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
   
};
use std::io::{stdout, Stdout};
use std::{ thread::sleep, time::Duration};
use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};


// will try to simulate the file text with a very large string and play with it 











enum Mode{
    Insert,
    Normal
}
struct Cursor {
    x : u16,
    y : u16
}

fn main() {
    // Get the initial content
    let mut cursory = Cursor{
        x :0,
        y:0,
    };
    let mut stdout = stdout();
    let mut text_buffer = get_initial_content();
    execute!(stdout,cursor::MoveTo(cursory.x,cursory.y)).expect("failed to move the cursor ");
    // enter The raw mode in order to get full control over the input (Get rid of the line buffer)
    terminal::enable_raw_mode().expect("Failed to Enable raw mode");
    
    /*
    This editor is inspired from vim and nano 
    So will try to create independent screen
     */
    

   

    execute!(stdout,EnterAlternateScreen).expect("Failed to enter the anlternate screen");
    let mut esc = 0;
    let mut current_mode = Mode::Normal;
    loop {
        execute!(stdout, Clear(ClearType::All)).unwrap();
        render_everything(&text_buffer, &mut stdout);
        
        if let Ok(event) = event::read(){
            match event {
                Event::Key(key_event) =>{
                 
                    if matches!(current_mode ,Mode::Normal) && key_event.code == KeyCode::Char('i'){
                        current_mode = Mode::Insert;
                        esc = 0;
                    }else if key_event.code == KeyCode::Esc{
                        current_mode = Mode::Normal;
                        esc+=1;
                        if esc == 3{ 
                            break;
                        }
                        
                    }else if matches!(current_mode ,Mode::Insert)   {
                        // Handling text insertion
                       
                       match key_event.code{
                        KeyCode::Backspace => {
                            if cursory.x > 0 {
                                (&mut text_buffer)[cursory.y as usize].remove(cursory.x as usize - 1);
                                cursory.x -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            let current_line = text_buffer[cursory.y as usize].split_off(cursory.x as usize);
                            (&mut text_buffer).insert(cursory.y as usize + 1, current_line);
                            cursory.y += 1;
                            cursory.x = 0;
                        }
                        KeyCode::Up => {
                            if cursory.y > 0 {
                                cursory.y -= 1;
                                cursory.x = cursory.x.min(text_buffer[cursory.y as usize].len() as u16);
                            }
                        }
                        KeyCode::Down => {
                            if cursory.y as usize + 1 < text_buffer.len() {
                                cursory.y += 1;
                                cursory.x = (cursory.x).min(text_buffer[cursory.y as usize].len() as u16);
                            }
                        }
                        KeyCode::Left => {
                            if cursory.x > 0 {
                                cursory.x -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if cursory.x < text_buffer[cursory.y as usize].len() as u16 {
                                cursory.x += 1;
                            }
                        }
                        KeyCode::Esc => {
                            esc +=1;
                            
                            
                           
                            }, 
                        KeyCode::Char(c) => {
                            println!("{}",c);
                            (&mut text_buffer)[cursory.y as usize].insert(cursory.x as usize, c);
                            cursory.x += 1;
                            //execute!(stdout,cursor::MoveTo(cursory.x,cursory.y)).expect("failed to move the cursor ");
                        }
                        
                        _ => {}

                       }
                        
                    }
                    
                },
                Event::Resize(width,height, )=>{
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                    render_everything(&text_buffer, &mut stdout);
                },
                _=>{}                
            }
        }
    }
    terminal::disable_raw_mode().expect("failed to disable raw mode");
    execute!(stdout,LeaveAlternateScreen).expect("Failed to leave the anlternate screen");


}


fn get_initial_content() ->  Vec<String>{
    let mut content: Vec<String> = Vec::new(); // Every element is a line 
    content.push("Hello world".to_string());

    return content ;
}

fn render_everything(content : &Vec<String>,stdout : &mut Stdout){
    for line in content.iter(){
        execute!(stdout,Print(format!("{} \n",line))).expect("failed to render the text");
    }
}

