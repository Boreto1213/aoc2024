use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let (mut list1, mut list2) = load_data("./data.txt")?;

    println!(
        "Total difference: {}",
        calculate_diff(&mut list1, &mut list2)
    );
    println!(
        "Simiarity score: {}",
        calculate_similarity_score(&mut list1, &mut list2)
    );

    Ok(())
}

fn calculate_diff(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> i32 {
    vec1.sort();
    vec2.sort();

    let mut total_diff = 0;

    for (l, r) in vec1.iter().zip(vec2.iter()) {
        total_diff += (l - r).abs();
    }

    total_diff
}

fn calculate_similarity_score(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> i32 {
    let mut similarity_score: i32 = 0;

    for num1 in vec1.iter() {
        let mut num_count: i32 = 0;

        for num2 in vec2.iter() {
            if num1 == num2 {
                num_count += 1;
            }
        }

        similarity_score += num1 * num_count;
    }

    similarity_score
}

fn load_data(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let num1: i32 = line[0..5].trim().parse().expect("Not a valid number");
        let num2: i32 = line[8..13].trim().parse().expect("Not a valid number");

        vec1.push(num1);
        vec2.push(num2);
    }

    Ok((vec1, vec2))
}
