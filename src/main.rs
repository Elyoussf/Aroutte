mod filehandling;
mod screen;


fn main(){
    if let Ok(f) = filehandling::openfile::analyzefilename(){
        screen::fillscreen::fillterminal(f);
        
    }else{
        println!("Error occured");
    }
}



