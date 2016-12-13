
//
//  Basic Example
//

extern crate ncurses;
extern crate nterm;

use nterm::{Terminal, Drawable, Attributes, Cursor, Color};

fn main() {
	let term = Terminal::new();
	term.cursor_state(Cursor::Invisible);
	let mut attrs = Attributes::colors(Color::Red, Color::Normal);
	attrs.underline = true;
	term.write("hello!", 0, 0, attrs);
	term.poll_event();
}
