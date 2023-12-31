/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 14:56:42 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 16:16:03 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::io::Write;

mod exs;
mod tests;

fn main() {
	let mut lines = io::stdin().lines();

	println!("Hi!");
	show_prompt();
	while let Some(line) = lines.next() {
		let cmd = line.unwrap();

		if cmd == "00" || cmd == "0" {
			tests::ex00::ex00();
		} else if cmd == "01" || cmd == "1" {
			tests::ex01::ex01();
		} else if cmd == "02" || cmd == "2" {
			tests::ex02::ex02();
		} else if cmd == "03" || cmd == "3" {
			tests::ex03::ex03();
		} else if cmd == "04" || cmd == "4" {
			tests::ex04::ex04();
		} else if cmd == "05" || cmd == "5" {
			tests::ex05::ex05();
		} else if cmd == "06" || cmd == "6" {
			tests::ex06::ex06();
		} else if cmd == "07" || cmd == "7" {
			tests::ex07::ex07();
		} else if cmd == "08" || cmd == "8" {
			tests::ex08::ex08();
		} else if cmd == "09" || cmd == "9" {
			tests::ex09::ex09();
		} else if cmd == "10" {
			tests::ex10::ex10();
		} else if cmd == "11" {
			tests::ex11::ex11();
		} else if cmd == "Q" || cmd == "q" || cmd == "quit" || cmd == "exit" {
			println!("Exiting...");
			break;
		} else {
			println!("Bad input");
		}
		show_prompt();
	}
	println!("Bye!");
}

fn show_prompt() {
	println!("\nChoisir exercice à tester: (00-11)");
	print!("> ");
	io::stdout().flush().unwrap();
}
