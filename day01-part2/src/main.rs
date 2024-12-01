use std::fs::File;
use std::i32;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    println!("Opening the file");

    let path = "real_input.txt";

    // let path = "example_input.txt";

    let file_name = File::open(&path)?;
    let reader = io::BufReader::new(file_name);

    let mut list: Vec<(i32, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if nums.len() == 2 {
            list.push((nums[0], nums[1]));
        } else {
            eprintln!("Skip malformed line: {}", line);
        }
    }

    println!("{:?}", list);

    let (left_nums, right_nums): (Vec<i32>, Vec<i32>) = list.iter().cloned().unzip();

    println!("Left Numbers: {:?}", left_nums);
    println!("Right Numbers: {:?}", right_nums);

    let mut total_sum: i32 = 0;
    for (_, num1) in left_nums.iter().enumerate() {
        let mut num: i32 = 0;
        for num2 in &right_nums {
            if num1 == num2 {
                num += 1
            }
        }

        let sum = *num1 * num;
        println!("{sum} = {num1} * {num}");
        total_sum += sum;
    }

    println!("{total_sum}");

    Ok(())
}
