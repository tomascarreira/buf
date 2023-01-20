use std::{env, fs::{File, OpenOptions}, io, path::{Path, PathBuf}};

fn main() {
    let mut args = env::args(); 
    args.next();
    
    let file_arg = args.next().expect("The user has to supply a path to a file");
    let file_path = Path::new(&file_arg);

    let mut file = File::open(file_path).expect("File exists");

    let mut backup_path = file_arg.clone();
    backup_path.push_str(".backup");

    let mut backup = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(backup_path).expect("Able to write file");

   io::copy(&mut file, &mut backup).expect("Able to copy file to backup");
}
