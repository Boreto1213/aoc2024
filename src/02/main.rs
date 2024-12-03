use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let levels = load_data("./data.txt")?;

    println!("Safe reports count: {}", count_safe_reports(&levels));
    println!("Safe reports count: {}", count_safe_reports_pt2(&levels));

    Ok(())
}

fn count_safe_reports(levels: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports_count = 0;

    for level in levels.iter() {
        let mut increasing = false;
        let mut decreasing = false;

        for (index, num) in level.iter().enumerate() {
            if index + 1 == level.len() {
                safe_reports_count += 1;
                break;
            }

            let diff = (*num - level[index + 1]).abs();

            if diff > 3 || diff < 1 {
                break;
            }

            if *num < level[index + 1] {
                increasing = true;
            }

            if *num > level[index + 1] {
                decreasing = true;
            }

            if increasing && decreasing {
                break;
            }
        }
    }

    safe_reports_count
}

fn count_safe_reports_pt2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports_count = 0;

    for report in reports.iter() {
        let mut safe = false;

        for (i, level) in report.iter().enumerate() {
            let mut report_copy = report.clone();
            report_copy.remove(i);

            let mut increasing = false;
            let mut decreasing = false;

            for (index, num) in report_copy.iter().enumerate() {
                if index + 1 == report_copy.len() {
                    safe = true;
                    break;
                }

                let diff = (*num - report_copy[index + 1]).abs();

                if diff > 3 || diff < 1 {
                    break;
                }

                if *num < report_copy[index + 1] {
                    increasing = true;
                }

                if *num > report_copy[index + 1] {
                    decreasing = true;
                }

                if increasing && decreasing {
                    break;
                }
            }

            if safe {
                safe_reports_count += 1;
                break;
            }
        }
    }

    safe_reports_count
}

fn load_data(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let level: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Not a valid number"))
            .collect();

        matrix.push(level);
    }

    Ok(matrix)
}
