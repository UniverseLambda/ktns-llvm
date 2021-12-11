/**
 * This file is part of Katanoisi.
 *
 * Katanoisi is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Katanoisi is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Katanoisi.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::io::Read;
use std::io::BufReader;
use std::fs::File;

pub enum TokenType {
	Unknown,
	IntegerLiteral,
	StringLiteral,
	CharLiteral,
	Keyword,
	Identifier
}

pub struct Token {
	token_type: TokenType,
	content: String,
}

pub enum Error {
	Unknown,
	EndOfFile,
	UnexpectedEndOfFile,
	DecoderError,
	InvalidCodePoint,
}

pub struct Lexer {
	file_path: String,
	reader: BufReader<File>,
	curr_char: char,
	faulty: bool,
}

impl Lexer {
	pub fn new(path: String) -> Lexer  {
		let open_res = File::open(path.clone());

		if let std::io::Result::Ok(f) = open_res {
			let mut instance = Lexer {
				file_path: path,
				reader: BufReader::new(f),
				curr_char: '\0',
				faulty: false,
			};

			instance.faulty = instance.next_char().is_err();

			return instance;
		}
		panic!("Could not open file");
	}

	pub fn new_str(file_path: &str) -> Lexer {
		return Lexer::new(String::from(file_path));
	}

	pub fn next_token(&mut self) -> Result<Token, Error> {
		println!("current char {}", self.curr_char);

		return Err(Error::EndOfFile);
	}

	fn next_char(&mut self) -> Result<(), Error> {
		let mut buffer = [0; 1];

		let res = self.reader.read(&mut buffer);
		let n = res.unwrap();

		if n == 0 {
			return Err(Error::EndOfFile);
		} else {
			let c = buffer[0];

			if (c & 0x80) == 0 {
				self.curr_char = c as char;
			} else {
				let mut cp: u32 = 0x00;
				let mut sup_byte_count = 0;

				if (c & 0x20) == 0 {
					sup_byte_count = 1;
					cp = (c as u32) & 0x1F;
				} else if (c & 0x10) == 0 {
					sup_byte_count = 2;
					cp = (c as u32) & 0x0F;
				} else if (c & 0x08) == 0 {
					sup_byte_count = 3;
					cp = (c as u32) & 0x07;
				} else {
					return Err(Error::DecoderError);
				}

				for _ in 0..sup_byte_count {
					let res = self.reader.read(&mut buffer);
					let n = res.unwrap();

					if n == 0 {
						return Err(Error::UnexpectedEndOfFile);
					}

					cp = (cp << 6) | ((buffer[0] as u32) & 0x3F);
				}

				let fromu32_res = std::char::from_u32(cp);

				if fromu32_res.is_none() {
					return Err(Error::InvalidCodePoint);
				}

				self.curr_char = fromu32_res.unwrap();
			}

			return Ok(());
		}
	}
}
