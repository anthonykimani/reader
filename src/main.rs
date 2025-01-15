use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env::args;

fn main() -> Result<(), std::io::Error> {
	if args().len() != 3 {
		eprintln!("Usage: `source` `target`");
		return Ok(());
	}
//1. open and define file to be read
	let file = File::open(args().nth(1).unwrap()).unwrap();
//2. create instance of BufReader
	let reader = BufReader::new(file);
//5. print line read
	// for line in reader.lines() {
		
	// 	println!("First word is {:?} and is {:?} bytes long", line, len);
	// }
	let search_collection: Vec<String> = reader.lines().filter_map(|l| l.ok().filter(|l| l.contains(&args().nth(2).unwrap()))).collect();
	println!("{:?}", search_collection);
	Ok(())
}
