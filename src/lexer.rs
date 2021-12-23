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

#[derive(Debug)]
pub enum TokenType {
	Unknown,
	IntegerLiteral,
	StringLiteral,
	CharLiteral,
	Keyword,
	Identifier,
	Operator
}

pub struct Token {
	pub token_type: TokenType,
	pub content: String,
}

#[derive(Debug)]
pub enum Error {
	Unknown,
	InternalError,
	EndOfFile,
	UnexpectedEndOfFile,
	DecoderError,
	InvalidCodePoint,
	InvalidCharacter,
	InvalidToken,
}

enum LexerMode {
	// Error,
	Word,
	Number,
	String(bool, bool),
	Operator,
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

	// pub fn new_str(file_path: &str) -> Lexer {
	// 	return Lexer::new(String::from(file_path));
	// }

	pub fn next_token(&mut self) -> Result<Token, Error> {
		let mut buff = String::default();

		while self.curr_char.is_whitespace() {
			let res = self.next_char();

			if res.is_err() {
				return Err(res.unwrap_err());
			}
		}

		let mut mode: LexerMode = if self.curr_char.is_alphabetic() || self.curr_char == '_' {
			LexerMode::Word
		} else if self.curr_char.is_numeric() {
			LexerMode::Number
		} else if self.curr_char == '"' {
			LexerMode::String(false, false)
		} else if is_operator(self.curr_char) {
			LexerMode::Operator
		} else {
			return Err(Error::InvalidCharacter);
		};

		buff.push(self.curr_char);

		loop {
			if let Result::Err(error) = self.next_char() {
				if let Error::EndOfFile = error {
					break;
				}

				return Err(error);
			}

			let res = match mode {
				LexerMode::Word => self.handle_word(&mut buff),
				LexerMode::Number => self.handle_number(&mut buff),
				LexerMode::String(_, _) => self.handle_string(&mut buff, &mode),
				LexerMode::Operator => self.handle_operator(&mut buff, &mut mode),
			};

			if let Result::Ok(complete) = res {
				if complete {
					break;
				}
			}
		}

		let res = match mode {
			LexerMode::Word => self.finalize_word(&mut buff),
			LexerMode::Number => self.finalize_number(&mut buff),
			LexerMode::String(_, _) => self.finalize_string(&mut buff),
			LexerMode::Operator => self.finalize_operator(&mut buff),
		};

		if let Ok(token) = res {
			if token.content == "//" {
				while self.curr_char != '\n' {
					if let Result::Err(error) = self.next_char() {
						return Err(error);
					}
				}
				return self.next_token();
			}
			Ok(token)
		} else {
			res
		}
	}

	fn handle_word(&mut self, buff: &mut String) -> Result<bool, Error> {
		let c = self.curr_char;

		if !c.is_alphanumeric() && c != '_' {
			return Ok(true);
		}

		buff.push(self.curr_char);

		return Ok(false);
	}

	// TODO: handle different base (ie: other than base 10)
	fn handle_number(&mut self, buff: &mut String) -> Result<bool, Error> {
		let c = self.curr_char;

		if !c.is_numeric() {
			if c.is_alphabetic() || c == '_' {
				return Err(Error::InvalidCharacter);
			}
			return Ok(true);
		}

		buff.push(self.curr_char);

		Ok(false)
	}

	fn handle_string(&mut self, buff: &mut String, mode: &LexerMode) -> Result<bool, Error> {
		let c = self.curr_char;

		if let LexerMode::String(complete, escape) = mode {
			if *complete {
				Ok(true)
			} else if c != '"' || *escape {
				buff.push(c);
				Ok(false)
			} else {
				Ok(true)
			}
		} else {
			Err(Error::InternalError)
		}
	}

	fn handle_operator(&mut self, buff: &mut String, mode: &mut LexerMode) -> Result<bool, Error> {
		let c = self.curr_char;

		if buff.len() > 2 {
			return Ok(true);
		}

		if buff.len() == 2 {
			if buff == ">>" && c == '>' {
				buff.push(c);
				return Ok(false);
			}
			return Ok(true);
		}

		// TODO: handle more radix
		if buff.starts_with('-') && c.is_numeric() {
			buff.push(c);
			*mode = LexerMode::Number;
			return Ok(false);
		}

		if buff.starts_with(c) {
			return match c {
				'-' | '+' | '=' | '/' | '&' | '|' => {
					buff.push(c);
					Ok(false)
				},
				_ => Ok(true)
			}
		}

		if (buff.starts_with('<') && c == '=') || (buff.starts_with('>') && c == '=') || (buff.starts_with('/') && c == '*') {
			buff.push(c);
			return Ok(false);
		}
		Ok(true)
	}

	fn finalize_word(&mut self, buff: &mut String) -> Result<Token, Error> {
		let mut tk_type = TokenType::Identifier;
		let keywords = ["fn", "true", "false", "u8", "u16", "u32", "u64", "bool", "return", "if", "else", "void"];

		for k in keywords {
			if buff == k {
				tk_type = TokenType::Keyword;
			}
		}

		Ok(Token { content: buff.clone(), token_type: tk_type })
	}

	fn finalize_number(&mut self, buff: &mut String) -> Result<Token, Error> {
		Ok(Token { content: buff.clone(), token_type: TokenType::IntegerLiteral })
	}

	fn finalize_string(&mut self, buff: &mut String) -> Result<Token, Error> {
		Ok(Token { content: buff.clone(), token_type: TokenType::StringLiteral })
	}

	fn finalize_operator(&mut self, buff: &mut String) -> Result<Token, Error> {
		Ok(Token { content: buff.clone(), token_type: TokenType::Operator })
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
				let mut cp: u32;
				let sup_byte_count: u32;

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

// TODO: Lexer: probably more operators?
fn is_operator(c: char) -> bool {
	match c {
		'+' | '-' | '*' | '/'
			| ',' | '.'
			| '=' | '>' | '<' | '|' | '&'
			| '?' | ':'
			| ';' | '(' | ')' | '[' | ']' | '{' | '}' => true,
		_ => false
	}
}
