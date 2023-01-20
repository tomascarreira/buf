use std::{fs::{File, OpenOptions}, io};
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
            ArgGroup::new("actions")
                .required(false)
                .args(["backup", "restore", "swap"]),
            ))]
struct Cli {
   #[arg(short, long)]
    backup: bool,

    #[arg(short, long)]
    restore: bool,

    #[arg(short, long)]
    swap: bool,

    file: String,
}

fn backup_file(path: &str) {
    
    let mut file = File::open(path).expect("File exists");

    let mut backup = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path.to_owned() + ".backup").expect("Able to write file");

   io::copy(&mut file, &mut backup).expect("Able to copy file to backup");
}

fn main() {
    let cli = Cli::parse();
    
    let path = cli.file;
    let (backup, restore, swap) = (cli.backup, cli.restore, cli.swap);

    match (backup, restore, swap) {
        (true, _, _) => backup_file(&path),
        (_, true, _) => todo!(),
        (_, _, true) => todo!(),
        _ => backup_file(&path),
    };
}
