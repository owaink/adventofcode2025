use std::fs::read_to_string;

fn main() {
    let file_content: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut dial: i16 = 50;
    let mut zero_count: i16 = 0;
    for line in file_content {
        let clicks: &i16 = &line[1..].parse::<i16>().unwrap();

        if line.into_bytes()[0] == b'R' {
            dial = (dial + clicks) % 100;
        } else {
            dial = dial - clicks;
            if dial < 0 {
                dial = 100 + dial;
            }
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    println!("Zero Count: {}", zero_count);
}
