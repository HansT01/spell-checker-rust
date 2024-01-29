use std::fs::read_to_string;

use spell_checker_rust::Vec2D;

fn main() {
    let file_content = read_to_string("data/words.txt").expect("Unable to read words.txt");
    let words: Vec<&str> = file_content.lines().collect();

    let mut test_v2d = Vec2D::from_slice(2, 2, &[1, 2, 3, 4]);

    for x in test_v2d.iter() {
        println!("{x:?}");
    }
}
