use std::fs;
use std::env;
use std::path::Path;
use chrono::DateTime;
use chrono::offset::Local;

fn dir_mode(dir: &str){
    let files_res = fs::read_dir(dir);
    if files_res.is_err() { 
        println!("File not found");
        return;
    }

    let path = Path::new(dir);
    println!("");
    println!("Directory of {}", path.to_str().unwrap());
    println!("");

    let files = files_res.unwrap();

    for path in files {
        let file = path.unwrap();
        let name = file.file_name().into_string().unwrap();
        let attr = file.metadata().unwrap();
        
        let datetime: DateTime<Local> = attr.modified().unwrap().into();
        let dir_stat = if attr.is_dir() { "<DIR>" } else { "" };
        let len = if attr.is_dir() { String::from("") } else {attr.len().to_string()};
        println!("{} \t {} \t {: >10} {}", datetime.format("%d/%m/%Y %T"), dir_stat, len, name);
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let dir_path_buf = env::current_dir().unwrap();

    // println!("{}", dir_path_buf.to_str().unwrap());

    let mut dir = String::from(dir_path_buf.to_str().unwrap());
    
    //let mut hidden = false;
    let mut first = true;
    for arg in args{
        if first {
            first = false;
            continue;
        }
        if arg == "/A"{
        //    hidden = true;
        }else {
            dir = String::from(arg);
        }
    }
    dir_mode(&dir)
}

