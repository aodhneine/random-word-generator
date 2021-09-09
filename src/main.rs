use rand;

use std::io::BufRead;
use rand::Rng;

fn main() {
	let file = std::fs::File::open("words-alpha.txt")
		.expect("failed to open the file");
	let buffered = std::io::BufReader::new(file);

	// Get all words from the file and get rid of all long ones.
	let words: Vec<String> = buffered
		.lines()
		.map(|line| line.unwrap())
		.filter(|word| word.len() > 2 && word.len() <= 4)
		.collect();

	let total_words = words.len();
	// Cache generator for faster speed.
	let mut rng = rand::thread_rng();

	// Create a table of 100 random words.
	let table = (0..100).map(|_| {
		let index = rng.gen::<usize>() % total_words;
		words[index].as_str()
	}).collect::<Vec<&str>>();

	// And choose on of them.
	let index = rng.gen::<usize>() % 100;
	println!("{}", table[index]);
}
