use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env::args;

fn main() -> Result<(), std::io::Error> {
	if args().len() != 2 {
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
	let readline1 = reader.lines().map(|l| l.unwrap());
	println!("{:?}", readline1.for_each(|f| println!("{} are made up of {:?} Letters", f, f.len())));
	Ok(())
}
