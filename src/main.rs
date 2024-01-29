use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("data/words.txt").expect("Unable to read words.txt");
    let words: Vec<&str> = file_content.lines().collect();
}
