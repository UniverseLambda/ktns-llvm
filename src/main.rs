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

mod lexer;
use lexer::Lexer;

fn main() {
	println!("Hello, world!");

	let mut lex = Lexer::new(String::from("test.ktns"));


	loop {
		match lex.next_token() {
			Ok(token) => println!("Token content: `{}'", token.content),
			Err(err) => {
				if let lexer::Error::EndOfFile = err {
					println!("Done :)");
					break;
				}
				println!("Lexer error: {}", err.to_string());
				break
			},
		}
	}

}
