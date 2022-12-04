/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part1.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/03 23:34:10 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

use crate::day1::INPUT_FILE;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(filepath)?;
    Ok(data)
}

fn file_string_demo() {
    let read_result = read_file_string(INPUT_FILE);

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

fn print_max_calories_v1() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(INPUT_FILE)?;
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

fn result_to_line(option: Option<String>) -> String {
    match option {
        None => "0".to_string(),
        Some(string) => string,
    }
}

fn print_max_calories_v2() -> std::io::Result<()> {
    let file = std::fs::File::open(INPUT_FILE)?;
    let reader = std::io::BufReader::new(file);
    let mut elf_calories = 0;
    let mut max_calories = 0;

    for result in reader.lines() {
        let line = result_to_line(result.ok());

        if line.eq("") {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
            continue;
        }

        let item_calories: i32 = line.parse().unwrap();
        elf_calories += item_calories;
    }

    println!("Day1, Part 1: {} max calories.", max_calories);
    Ok(())
}

fn get_input_buffer() -> BufReader<File> {
    let file = File::open(INPUT_FILE);
    BufReader::new(file.unwrap())
}

pub fn print_max_calories() {
    let reader = get_input_buffer();
    let mut elf_calories = 0;
    let mut max_calories = 0;

    for result in reader.lines() {
        let line = result.unwrap();

        if line.eq("") {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
            continue;
        }

        let current: i32 = line.parse().unwrap();
        elf_calories += current;
    }

    println!("Part 1: {} max calories.", max_calories);
}
