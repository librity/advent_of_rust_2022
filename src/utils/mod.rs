/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/04 10:19:39 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 10:28:53 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_input_lines(filepath: &str) -> Lines<BufReader<File>> {
    let file = File::open(filepath);
    BufReader::new(file.unwrap()).lines()
}
