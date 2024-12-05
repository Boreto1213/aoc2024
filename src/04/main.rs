fn main() {
    let data = include_str!("./data.txt");

    println!("Result pt1: {}", pt1(data));
}

fn pt1(data: &str) -> i32 {
    let mut matrix: Vec<Vec<char>> = data
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut word_char_cords: Vec<(usize, usize)> = Vec::new();

    // Collect positions first
    let positions: Vec<(usize, usize, char)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &char)| (x, y, char)))
        .collect();

    // Mutate matrix after collecting positions
    for (x, y, char) in positions {
        check_for_xmas(x, y, char, &mut matrix, &mut word_char_cords);
    }

    0
}
fn check_for_xmas(
    x: usize,
    y: usize,
    char: char,
    matrix: &mut Vec<Vec<char>>,
    word_char_cords: &mut Vec<(usize, usize)>,
) {
    let target_word = match char {
        'X' => "xmas",
        'S' => "samx",
        _ => return, // Exit the outer function if the character doesn't match 'X' or 'S'
    };

    let mut current_word_cords: Vec<(usize, usize)> = Vec::new();

    // Check if the target word fits in the row
    if x + target_word.len() > matrix[y].len() {
        return;
    }

    for (i, target_char) in target_word.chars().enumerate() {
        if matrix[y][x + i] != target_char {
            return;
        }

        current_word_cords.push((y, x + i));
    }

    // If the word matches, update the matrix and store the coordinates
    for &(y, x) in &current_word_cords {
        matrix[y][x] = '?'; // Replace matched characters
    }

    word_char_cords.extend(current_word_cords);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./ex-pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 161);
    }
}

