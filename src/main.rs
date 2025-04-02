use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

struct Parser {
    open_brackets: usize,
    open_quotes: bool,
    obj: String,
    attr: String,
    items: Vec<String>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            open_brackets: 0,
            open_quotes: false,
            obj: "".into(),
            attr: "".into(),
            items: Vec<String>,
            comment: "".into(),
        }
    }

    fn parse_line(&mut self, &line) {
        let mut cur_item:String = "".into();
        
        for (charno, c) in line.chars().enumerate() {
            match c {
                '"' => { self.open_quotes = !self.open_quotes; },
                '{' => { if !self.open_quotes { self.open_brackets += 1 ; } },
                '}' => { if !self.open_quotes { self.open_brackets -= 1; } },
                '#' => { 
                    self.comment = line[charno+1..].trim();
                    break;
                    },
                ' ' => {
                    let item = cur_item.trim();
                    if self.obj.len() == 0 {
                        self.obj = item;
                    } else if self.attr.is_empty() {
                        self.attr = item;
                    } else {
                        self.items.push(item);
                    }
                    cur_item.clear();
                },
                _ => { cur_item.push(c) },
            }
        
        }
        
    } 

    fn is_complete(&self) -> bool {
        (self.open_brackets = 0) & (! self.open_quotes)
    }

    fn reset(&mut self) {
        self.open_brackets = 0;
        self.open_quotes = false;
        self.obj.clear();
        self.attr.clear();
        self.items.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Please provide a file name as argument...");
        return
    }
    let filename = &args[1];
    let mut data: HashMap<HashMap<String,Vec<String>>>;
    let mut parser: Parser;
    
    if let Ok(lines) = read_lines(filename) {
        for (ii, line) in lines.map_while(Result::ok).enumerate() {
            parser.parse_line(&line);
            if parser.is_complete() {
                obj_hash = match data.get(parser.obj) {
                    Some(obj_hm) => match obj_hm.get_mut(parser.attr) {
                        Some(x) => { x = parser.items },
                        None => {obj_hm.insert(parser.attr, parser.items)},
                    }                             
                }           
            }

        }
    }
}

fn is_line_finished(line: &str) -> bool {
    let mut open_brackets = 0;
    let mut open_quotes = false;
    
    for c in line.chars() {
        match c {
            '"' => { open_quotes = !open_quotes; }
            '{' => { if !open_quotes { open_brackets += 1 ; } }
            '}' => { if !open_quotes { open_brackets -= 1; } }
            _ => (),
        }
        
    }
    (open_brackets == 0) & (!open_quotes)
}

fn parse_line(line: &str, parser_state: &mut Parser) {
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
