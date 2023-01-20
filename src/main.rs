use std::{env, fs::{File, OpenOptions}, io};

fn main() {
    let mut args = env::args(); 
    args.next();
    
    let file_path = args.next().expect("The user has to supply a path to a file");

    let mut file = File::open(&file_path).expect("File exists");

    let mut backup = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(file_path + ".backup").expect("Able to write file");

   io::copy(&mut file, &mut backup).expect("Able to copy file to backup");
}
