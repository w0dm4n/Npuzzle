// **************************************************************************** //
//                                                                              //
//                                                         :::      ::::::::    //
//    parser.rs                                          :+:      :+:    :+:    //
//                                                     +:+ +:+         +:+      //
//    By: frmarinh <frmarinh@student.42.fr>          +#+  +:+       +#+         //
//                                                 +#+#+#+#+#+   +#+            //
//    Created: 2017/10/25 11:09:53 by frmarinh          #+#    #+#              //
//    Updated: 2017/10/25 11:09:56 by frmarinh         ###   ########.fr        //
//                                                                              //
// **************************************************************************** //

use std::env;
use std::fs::File;

mod parser;
use parser::Parser;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let file_path = &args[1];
		if file_path.len() > 0 {
			let parser: Parser = Parser::parse_file(file_path);
			if parser.is_valid() {
				println!("{:?}", file_path);
			} else {
				println!("The file {} is not valid.", parser.get_file());
			}
		} else {
			println!("Invalid file name.");
		}
	} else {
		println!("You must set a file as argument containing a puzzle to solve.");
	}
}
