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
}

pub struct Lexer {
	file_path: String,
}

impl Lexer {
	pub fn next(&self) -> Result<Token, Error> {
		return Err(Error::EndOfFile);
	}

	pub fn new(file_path: String) -> Lexer {
		return Lexer {file_path: file_path};
	}

	pub fn new_str(file_path: &str) -> Lexer {
		return Lexer::new(String::from(file_path));
	}
}
