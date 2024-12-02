use std::fs;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("everybody_codes_e2024_q02_p1.txt").unwrap();
    //let content = fs::read_to_string("test.txt").unwrap();
    let result1 = part_one(content);
    println!("Part 1: {result1}");
}

fn part_one(input: String) -> u64 {
    let re = Regex::new(r"WORDS:([A-Z,]+)\n\n([A-Z ,.]+)").unwrap();
    let captures = re.captures(&input).unwrap();
    let re_comma = Regex::new(r",").unwrap();
    let words: Vec<&str> = re_comma.split(&captures[1]).collect();
    let text = &captures[2];


    let mut matches = 0;
    for word in words {
	let re_word = Regex::new(word).unwrap();
	matches += re_word.find_iter(text).count();
    }
    matches.try_into().unwrap()
}
