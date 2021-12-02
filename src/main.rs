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

	let lex = Lexer::new(String::from("Yoooo"));

	let t = lex.next();

	if t.is_err() {
		println!("Woopsi");
	} else {
		println!("What?!");
	}
}
