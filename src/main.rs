use spell_checker_rust::Vec2D;

fn calculate_distance(s1: &str, s2: &str) -> usize {
    let rows = s1.len() + 1;
    let cols = s2.len() + 1;
    let mut matrix: Vec2D<usize> = Vec2D::new(rows, cols);
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

fn main() {
    // let file_content = read_to_string("data/words.txt").expect("Unable to read words.txt");
    // let words: Vec<&str> = file_content.lines().collect();
    let distance = calculate_distance("appa", "aa");
    println!("Distance: {distance}");
}
