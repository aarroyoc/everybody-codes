use std::fs;


fn main() {
    let content_p1 = fs::read_to_string("everybody_codes_e2024_q01_p1.txt").expect("Could not read the data file");
    let part1_result = part1(content_p1);
    println!("Part 1 result: {part1_result}");

    let content_p2 = fs::read_to_string("everybody_codes_e2024_q01_p2.txt").expect("Could not read the data file");
    let part2_result = part2(content_p2);
    println!("Part 2 result: {part2_result}");
}

fn part1(input: String) -> u64 {
    input.chars().map(|ch|
		      match ch {
			  'A' => 0,
			  'B' => 1,
			  'C' => 3,
			  _   => 0,
		      }).sum()
}

fn points(ch: char) -> Option<u64> {
    match ch {
	'A' => Some(0),
	'B' => Some(1),
	'C' => Some(3),
	'D' => Some(5),
	_ => None,
    }
}

fn part2(input: String) -> u64 {
    let mut sum = 0;
    let mut chars = input.chars();
    let mut i = 0;
    while i < input.len() {
	let fst = points(chars.next().unwrap());
	let snd = points(chars.next().unwrap());
	let points = match (fst, snd) {
	    (Some(a), Some(b)) => a + b + 2,
	    (Some(a), None) => a,
	    (None, Some(b)) => b,
	    _ => 0,
	};
	sum += points;
	i += 2;
    }
    sum
}
	

#[test]
fn test_part1() {
    let content = fs::read_to_string("everybody_codes_e2024_q01_p1.txt").expect("Could not read the data file");
    assert_eq!(part1(content), 1308);
}