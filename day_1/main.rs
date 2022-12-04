/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/03 21:18:14 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// rustc main.rs -o a.out && ./a.out

use std::io::BufRead;

const INPUT_FILE_PATH: &str = "./input";

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(filepath)?;
    Ok(data)
}

fn file_string_demo() {
    let read_result = read_file_string(INPUT_FILE_PATH);

    match read_result {
        Ok(string) => println!("Result: '{}'", string),
        Err(error) => println!("Error: '{}'", error),
    }
}

fn print_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);
    let mut line_count = 1;

    for line in reader.lines() {
        println!("{}: \"{}\"", line_count, line?);
        line_count += 1;
    }

    Ok(())
}

fn print_max_calories() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(INPUT_FILE_PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut elf_calories = 0;
    let mut max_calories = 0;

    for result in reader.lines() {
        let line = match result {
            Ok(string) => string,
            Err(_error) => "0".to_string(),
        };

        if line.eq("") {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
        } else {
            let current: i32 = line.parse().unwrap();
            elf_calories += current;
        }
    }

    println!("max_calories: {}", max_calories);
    Ok(())
}

fn main() {
    // file_string_demo();
    // print_file_line_by_line(INPUT_FILE_PATH);

    match print_max_calories() {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error),
    }
}
