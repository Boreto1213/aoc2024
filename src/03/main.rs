use regex::Regex;

fn main() {
    let data = include_str!("./data.txt");

    println!("Result pt1: {}", pt1(data));
    println!("Result pt2: {}", pt2(data));
}

fn pt1(data: &str) -> i32 {
    let mut result = 0;

    Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .captures_iter(data)
        .for_each(|cap| {
            let parts: Vec<&str> = cap.get(0).unwrap().as_str().split(',').collect();
            let left_num = &parts[0][4..].parse::<i32>().expect("Not a valid number");
            let right_num = &parts[1][..&parts[1].len() - 1]
                .parse::<i32>()
                .expect("Not a valid number");

            result += left_num * right_num;
        });

    result
}

fn pt2(data: &str) -> i32 {
    let mut result: i32 = 0;
    let mut execute_command = true;

    Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(data)
        .for_each(|cap| {
            let str_match = cap.get(0).unwrap().as_str();

            execute_command = match str_match {
                "do()" => true,
                "don't()" => false,
                _ => execute_command,
            };

            if str_match.starts_with("mul") && execute_command {
                let parts: Vec<&str> = str_match.split(',').collect();
                let left_num = &parts[0][4..].parse::<i32>().expect("Not a valid number");
                let right_num = &parts[1][..&parts[1].len() - 1]
                    .parse::<i32>()
                    .expect("Not a valid number");

                result += left_num * right_num;
            }
        });

    result
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

    #[test]
    fn pt2_test() {
        let input = include_str!("./ex-pt2.txt");
        let result = pt2(&input);
        assert_eq!(result, 48);
    }
}
