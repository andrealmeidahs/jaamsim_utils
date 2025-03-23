use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    if let Ok(lines) = read_lines(filename) {
        for (ii, line) in lines.map_while(Result::ok).enumerate() {
            let mut new_line: &str;
            println!("{}: {}", ii, line);
            match line.find('#') {
                Some(comment_idx) => { 
                    new_line = &line[0..comment_idx].trim();
                    if new_line.len() > 0 {
                        println!("   {}", new_line);
                    }
                },
                None => new_line = &line
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
