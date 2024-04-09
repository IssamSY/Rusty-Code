use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
	let mut sample_txt = File::create("sample.txt");
    
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        
        if buffer.trim() == "q" {
            break;
        }
        
    }
}
