use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;

use handson3::design_course;
use handson3::holiday_planning;

fn main() {
    let folder_path = "TestSet";
    let mut n: usize;
    let mut d: usize;
    for i in 0..5 { //goes through all the test files
        let input_file = format!("{}/input{}.txt", folder_path, i);    //builds the path for the input files
        let output_file = format!("{}/output{}.txt", folder_path, i);  //builds the path for the corresponding output file
        let file_in = match fs::File::open(input_file) {
            Ok(file_in) => file_in,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };
        let mut reader = io::BufReader::new(file_in);
        //read the first line of the input file
        if let Some(Ok(first_line)) = reader.by_ref().lines().next() {
            let numbers: Vec<&str> = first_line.split_whitespace().collect();   //first line = n D
            n = numbers[0].parse::<usize>().unwrap();
            d = numbers[1].parse::<usize>().unwrap();

            // we must build a vector of size n*D containing the values of the cities
            // for each city, since the i-th day can be visited iff all the (i-1) days were visited,
            // we build D values for each city which contains the value=sum of the values up to the i-th day of that city
            let mut values: Vec<i32> = Vec::new();
            //read the remaining lines and fill the values vector
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let line_numbers: Vec<&str> = line_content.split_whitespace().collect();
                    let mut value_sum = 0;
                    for v in 0..d {
                        value_sum = value_sum + line_numbers[v].parse::<i32>().unwrap();
                        values.push(value_sum);
                    }
                } else {
                    eprintln!("Error on the reading of a line");
                    std::process::exit(1);
                }
            }
            // println!("{:?}", values);
            let res = holiday_planning(n, d, values);
            // println!("{:?}", res);
            // println!("");
            // check if the results correspond to the value in the output file
            let file_out = match fs::File::open(output_file) {
                Ok(file_out) => file_out,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    std::process::exit(1);
                }
            };
            let mut reader = io::BufReader::new(file_out);
            if let Ok(line_content) = reader.by_ref().lines().next().unwrap() {
                if line_content.parse::<i32>().unwrap() != res {
                    println!("Test{}: wrong, result must be {} but is {}", i, line_content.parse::<usize>().unwrap(), res);
                    return;
                }
            }
            println!("Test{}: ok", i);
        }
    }
    println!("");
/* --------------------------------------------------- PROBLEM 2 --------------------------------------------------- */
    let folder_path = "TestSet 2";
    let mut n: usize;
    for i in 0..11 { //goes through all the test files
        let input_file = format!("{}/input{}.txt", folder_path, i);    //builds the path for the input files
        let output_file = format!("{}/output{}.txt", folder_path, i);  //builds the path for the corresponding output file
        let file_in = match fs::File::open(input_file) {
            Ok(file_in) => file_in,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };
        let mut reader = io::BufReader::new(file_in);
        //read the first line of the input file
        if let Some(Ok(first_line)) = reader.by_ref().lines().next() {
            n = first_line.parse::<usize>().unwrap();   //first line = n

            // we must build a vector of size n containing pairs (beauty, difficulty)
            let mut values: Vec<(i32, i32)> = Vec::new();
            //read the remaining lines and fill the values vector
             for _i in 0..n {
                if let Ok(line_content) = reader.by_ref().lines().next().unwrap() {
                    let line_numbers: Vec<&str> = line_content.split_whitespace().collect();
                    values.push((line_numbers[0].parse::<i32>().unwrap(), line_numbers[1].parse::<i32>().unwrap()));
                } else {
                    eprintln!("Error on the reading of a line");
                    std::process::exit(1);
                }
             }
            let res = design_course(n, values);
            // println!("{:?}", res);
            // println!("");
            // check if the results correspond to the value in the output file
            let file_out = match fs::File::open(output_file) {
                Ok(file_out) => file_out,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    std::process::exit(1);
                }
            };
            let mut reader = io::BufReader::new(file_out);
            if let Ok(line_content) = reader.by_ref().lines().next().unwrap() {
                if line_content.parse::<usize>().unwrap() != res {
                    println!("Test{}: wrong, result must be {} but is {}", i, line_content.parse::<usize>().unwrap(), res);
                    return;
                }
            }
            println!("Test{}: ok", i);
        }
    }
}
