
use std::fs::File;
use std::io::Error;
use std::io::Read;

fn main() {

    let argv : Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        eprintln!("[ERROR] No Input File passed to program.");
        return;
    }

    for i in 1..argv.len(){

        let mut buffer : String = String::new();
        let file : Result<File,Error> = File::open(&argv[i]);
        match file {
            Ok(_) => {
            },
            Err(e) => {
                eprintln!("[ERROR] Failed to open file \"{}\". {}",&argv[i], e);
                return;
            },
        };
        let Ok(mut file) = file else{
            eprintln!("[ERROR] Failed for some reason.\n");
            return;
        };

        if let Err(e) = file.read_to_string(&mut buffer){
            eprintln!("[ERROR] Failed to buffer file. {}", e);
            return;
        }

        print!("{}", buffer);
    }
}
