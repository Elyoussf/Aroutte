mod filehandling;

fn main(){
    let res = filehandling::openfile::analyzefilename();
    if let Ok(rs) = res{
        println!("Got the file");
    }else{
        println!("{:?}",res);
    }
}