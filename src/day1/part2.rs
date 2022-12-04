/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/03 22:46:48 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::day1::INPUT_FILE_PATH;
use std::io::BufRead;

// fn get_input_buffer() -> Result<std::io::BufReader<&str>, std::io::Error> {
//     let file = std::fs::File::open(day1::INPUT_FILE_PATH)?;
//     Ok(std::io::BufReader::new(file))
// }

fn result_to_line(option: Option<String>) -> String {
    match option {
        None => "0".to_string(),
        Some(string) => string,
    }
}

pub fn print_top_three() -> std::io::Result<()> {
    let file = std::fs::File::open(INPUT_FILE_PATH)?;
    let reader = std::io::BufReader::new(file);
    let (mut elf_calories, mut max_calories) = (0, 0);

    for result in reader.lines() {
        let line = result_to_line(result.ok());

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

    println!("max_calories: {}", max_calories);
    Ok(())
}
