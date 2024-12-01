use std::fs::File;
use std::i32;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    println!("Opening the file");

    let path = "real_input.txt";
    //    let path = "example_input.txt";

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

    let (mut left_nums, mut right_nums): (Vec<i32>, Vec<i32>) = list.iter().cloned().unzip();

    left_nums.sort();
    right_nums.sort();

    println!("Left Numbers: {:?}", left_nums);
    println!("Right Numbers: {:?}", right_nums);

    let mut total_sum: i32 = 0;
    for (index, num1) in left_nums.iter().enumerate() {
        let num2: i32 = right_nums[index];
        let sum: i32;

        if *num1 > num2 {
            sum = *num1 - num2;
        } else {
            sum = num2 - *num1;
        }
        println!("index: {index} » {sum} = {num2} - {num1}");

        total_sum += sum;
    }

    println!("{total_sum}");

    Ok(())
}
