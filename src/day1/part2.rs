/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/03 23:34:16 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::day1::INPUT_FILE;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input_buffer() -> BufReader<File> {
    let file = File::open(INPUT_FILE);
    BufReader::new(file.unwrap())
}

pub fn print_top_three_sum() {
    let reader = get_input_buffer();
    let mut elf_calories = 0;
    let mut calories: Vec<i32> = Vec::new();

    for result in reader.lines() {
        let line = result.unwrap();

        if line.eq("") {
            calories.push(elf_calories);
            elf_calories = 0;
            continue;
        }

        let current: i32 = line.parse().unwrap();
        elf_calories += current;
    }

    calories.sort_by(|a, b| b.cmp(a));
    // let top_three_sum = calories[0] + calories[1] + calories[2];
    let top_three_sum: i32 = calories[0..3].iter().sum();
    println!("Part 2: {:?} total calories.", top_three_sum);
}
