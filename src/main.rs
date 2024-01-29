use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    time::Instant,
};

use spell_checker_rust::Array2D;

const LEN_DIFF_LIMIT: usize = 2;

fn calculate_distance(s1: &str, s2: &str) -> usize {
    if s1.len().abs_diff(s2.len()) > 2 {
        return usize::MAX;
    }
    let rows = s1.len() + 1;
    let cols = s2.len() + 1;
    let mut matrix: Array2D<usize> = Array2D::new(rows, cols);
    for i in 0..rows {
        matrix[i][0] = i
    }
    for i in 0..cols {
        matrix[0][i] = i
    }
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    for i in 1..rows {
        for j in 1..cols {
            let cost = if c1[i - 1] == c2[j - 1] { 0 } else { 1 };
            let insertion = matrix[i][j - 1] + 1;
            let deletion = matrix[i - 1][j] + 1;
            let substitution = matrix[i - 1][j - 1] + cost;
            let distance = insertion.min(deletion).min(substitution);
            matrix[i][j] = distance;
        }
    }
    return matrix[rows - 1][cols - 1];
}

fn top_k_matches<'a>(
    word_to_check: &'a str,
    word_map: &'a HashMap<usize, Vec<&str>>,
    k: usize,
) -> Vec<(usize, &'a str)> {
    let mut top_k_heap: BinaryHeap<(usize, &str)> = BinaryHeap::new();
    for i in word_to_check.len() - LEN_DIFF_LIMIT..word_to_check.len() + LEN_DIFF_LIMIT {
        for &word in word_map.get(&i).unwrap_or(&Vec::new()).iter() {
            let distance = calculate_distance(word_to_check, word);
            top_k_heap.push((distance, word));
            if top_k_heap.len() > k {
                top_k_heap.pop();
            }
        }
    }
    let mut matches: Vec<(usize, &str)> = top_k_heap.into();
    matches.sort();
    matches
}

fn main() {
    let file_content = read_to_string("data/words.txt").expect("Unable to read words.txt");
    let word_list: Vec<&str> = file_content.lines().collect();
    let mut word_map: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in word_list.iter() {
        let vec = word_map.entry(word.len()).or_insert(Vec::new());
        vec.push(word)
    }

    let word_to_check = "catalyt";
    let k = 10;

    let start_time = Instant::now();
    let matches = top_k_matches(word_to_check, &word_map, k);
    let elapsed_time = start_time.elapsed();

    println!("Results: {matches:?}");
    println!("Time elapsed: {elapsed_time:.2?}");
}
