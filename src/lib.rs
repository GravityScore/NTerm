
//
//  NTerm
//

//! Higher level wrapper around ncurses.

extern crate ncurses;

mod event;

use std::char;
use ncurses::{BUTTON1_PRESSED, BUTTON2_PRESSED, BUTTON3_PRESSED,
	BUTTON1_RELEASED, BUTTON2_RELEASED, BUTTON3_RELEASED};

pub use event::{Key, MouseButton, Event};


/// The options for a cell's foreground or background color.
#[derive(Clone, Copy)]
pub enum Color {
	/// The default foreground or background color of the terminal.
	Normal = -1,
	Black = 0,
	Red = 1,
	Green = 2,
	Yellow = 3,
	Blue = 4,
	Magenta = 5,
	Cyan = 6,
	White = 7,
}

/// Attributes associated with a cell on the terminal.
#[derive(Clone, Copy)]
pub struct Attributes {
	pub foreground: Color,
	pub background: Color,
	pub bold: bool,
	pub underline: bool,
}

impl Attributes {
	/// Create a default set of attributes.
	pub fn new() -> Attributes {
		Attributes {
			foreground: Color::White,
			background: Color::Black,
			bold: false,
			underline: false,
		}
	}

	/// Create a set of attributes from a foreground and background color.
	pub fn colors(foreground: Color, background: Color) -> Attributes {
		Attributes {
			foreground: foreground,
			background: background,
			bold: false,
			underline: false,
		}
	}

	/// Create a set of attributes from just a background color (setting the
	/// foreground color to white).
	pub fn background(background: Color) -> Attributes {
		Attributes {
			foreground: Color::White,
			background: background,
			bold: false,
			underline: false,
		}
	}

	/// Set the attributes.
	fn set(&self) {
		ncurses::init_pair(1, self.foreground as i16, self.background as i16);
		ncurses::attron(ncurses::COLOR_PAIR(1));
		if self.bold {
			ncurses::attron(ncurses::A_BOLD());
		}
		if self.underline {
			ncurses::attron(ncurses::A_UNDERLINE());
		}
	}

	/// Unset any attributes.
	fn unset(&self) {
		ncurses::attroff(ncurses::COLOR_PAIR(1) | ncurses::A_BOLD() |
			ncurses::A_UNDERLINE());
	}
}


/// Possible cursor states.
#[derive(Clone, Copy)]
pub enum Cursor {
	Invisible,
	Visible,
	VeryVisible,
}

impl Cursor {
	/// Converts a cursor state into an ncurses state.
	fn to_ncurses(&self) -> ncurses::CURSOR_VISIBILITY {
		match *self {
			Cursor::Invisible =>
				ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE,
			Cursor::Visible =>
				ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE,
			Cursor::VeryVisible =>
				ncurses::CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE,
		}
	}
}


/// Characters in the alternative character set.
#[derive(Clone, Copy)]
pub enum AlternativeCharacter {
	CornerTopLeft,
	CornerTopRight,
	CornerBottomLeft,
	CornerBottomRight,
	TeeRight,
	TeeLeft,
	TeeUp,
	TeeDown,
	LineHorizontal,
	LineVertical,
	Crossover,
	LineHigh,
	LineLow,
	Diamond,
	Stipple,
	Bullet,
	ArrowLeft,
	ArrowRight,
	ArrowUp,
	ArrowDown,
	Block,
}

impl AlternativeCharacter {
	/// Converts an alternative character to an ncurses character.
	fn to_ncurses(&self) -> ncurses::chtype {
		match *self {
			AlternativeCharacter::CornerTopLeft => ncurses::ACS_ULCORNER(),
			AlternativeCharacter::CornerTopRight => ncurses::ACS_URCORNER(),
			AlternativeCharacter::CornerBottomLeft => ncurses::ACS_LLCORNER(),
			AlternativeCharacter::CornerBottomRight => ncurses::ACS_LRCORNER(),
			AlternativeCharacter::TeeRight => ncurses::ACS_RTEE(),
			AlternativeCharacter::TeeLeft => ncurses::ACS_LTEE(),
			AlternativeCharacter::TeeUp => ncurses::ACS_BTEE(),
			AlternativeCharacter::TeeDown => ncurses::ACS_TTEE(),
			AlternativeCharacter::LineHorizontal => ncurses::ACS_HLINE(),
			AlternativeCharacter::LineVertical => ncurses::ACS_VLINE(),
			AlternativeCharacter::Crossover => ncurses::ACS_PLUS(),
			AlternativeCharacter::LineHigh => ncurses::ACS_S1(),
			AlternativeCharacter::LineLow => ncurses::ACS_S9(),
			AlternativeCharacter::Diamond => ncurses::ACS_DIAMOND(),
			AlternativeCharacter::Stipple => ncurses::ACS_CKBOARD(),
			AlternativeCharacter::Bullet => ncurses::ACS_BULLET(),
			AlternativeCharacter::ArrowLeft => ncurses::ACS_LARROW(),
			AlternativeCharacter::ArrowRight => ncurses::ACS_RARROW(),
			AlternativeCharacter::ArrowUp => ncurses::ACS_UARROW(),
			AlternativeCharacter::ArrowDown => ncurses::ACS_DARROW(),
			AlternativeCharacter::Block => ncurses::ACS_BLOCK(),
		}
	}
}


/// An interface for something that can be drawn to (like a window or the
/// terminal itself).
pub trait Drawable {
	/// Puts a character from the alternative character set at a location with
	/// the specified attributes.
	fn character(&self, ch: AlternativeCharacter, x: u32, y: u32,
		attributes: Attributes);

	/// Puts a whole string at a location with the specified attributes.
	fn write(&self, string: &str, x: u32, y: u32, attributes: Attributes);

	/// Clears the drawable to the specified background color.
	fn clear(&self, background: Color);

	/// Moves the cursor to the specified location within this drawable.
	fn move_cursor(&self, x: u32, y: u32);

	/// Returns the width and height of this drawable.
	fn size(&self) -> (u32, u32);

	/// Returns the position of a window on screen.
	fn position(&self) -> (u32, u32);

	/// Draws all changes made to the internal memory buffer to the actual
	/// terminal window.
	fn draw(&self);
}


/// The terminal itself.
pub struct Terminal;

impl Terminal {
	/// Create a new instance of the termial window. Usually is only called
	/// once upon program start to initialise ncurses.
	pub fn new() -> Terminal {
		ncurses::initscr();
		ncurses::raw();
		ncurses::keypad(ncurses::stdscr(), true);
		ncurses::cbreak();
		ncurses::noecho();
		ncurses::mouseinterval(0);
		ncurses::start_color();
		ncurses::use_default_colors();
		Terminal
	}

	/// Change the cursor state.
	pub fn cursor_state(&self, state: Cursor) {
		ncurses::curs_set(state.to_ncurses());
	}

	/// Returns true if the mouse is supported on this terminal.
	pub fn has_mouse_support(&self) -> bool {
		ncurses::has_mouse()
	}

	/// Waits for an event to occur, returning the event when one does.
	pub fn poll_event(&self) -> Event {
		match ncurses::getch() {
			ncurses::KEY_UP => Event::Key(Key::Up),
			ncurses::KEY_DOWN => Event::Key(Key::Down),
			ncurses::KEY_LEFT => Event::Key(Key::Left),
			ncurses::KEY_RIGHT => Event::Key(Key::Right),
			ncurses::KEY_HOME => Event::Key(Key::Home),
			ncurses::KEY_END => Event::Key(Key::End),
			ncurses::KEY_PPAGE => Event::Key(Key::PageUp),
			ncurses::KEY_NPAGE => Event::Key(Key::PageDown),
			ncurses::KEY_BACKSPACE => Event::Key(Key::Backspace),
			ncurses::KEY_ENTER => Event::Key(Key::Enter),
			ncurses::KEY_EXIT => Event::Key(Key::Escape),
			ncurses::KEY_F0 => Event::Key(Key::F(0)),
			ncurses::KEY_F1 => Event::Key(Key::F(1)),
			ncurses::KEY_F2 => Event::Key(Key::F(2)),
			ncurses::KEY_F3 => Event::Key(Key::F(3)),
			ncurses::KEY_F4 => Event::Key(Key::F(4)),
			ncurses::KEY_F5 => Event::Key(Key::F(5)),
			ncurses::KEY_F6 => Event::Key(Key::F(6)),
			ncurses::KEY_F7 => Event::Key(Key::F(7)),
			ncurses::KEY_F8 => Event::Key(Key::F(8)),
			ncurses::KEY_F9 => Event::Key(Key::F(9)),
			ncurses::KEY_F10 => Event::Key(Key::F(10)),
			ncurses::KEY_F11 => Event::Key(Key::F(11)),
			ncurses::KEY_F12 => Event::Key(Key::F(12)),
			ncurses::KEY_F13 => Event::Key(Key::F(13)),
			ncurses::KEY_F14 => Event::Key(Key::F(14)),
			ncurses::KEY_F15 => Event::Key(Key::F(15)),
			ncurses::KEY_MOUSE => {
				let mut event = ncurses::MEVENT {
					id: 0,
					x: 0,
					y: 0,
					z: 0,
					bstate: 0,
				};
				ncurses::getmouse(&mut event);
				if event.bstate & BUTTON1_PRESSED as u32 == BUTTON1_PRESSED as u32 {
					Event::MouseDown(event.x as u32, event.y as u32,
						MouseButton::Left)
				} else if event.bstate & BUTTON1_RELEASED as u32 == BUTTON1_RELEASED as u32 {
					Event::MouseUp(event.x as u32, event.y as u32,
						MouseButton::Left)
				} else if event.bstate & BUTTON2_PRESSED as u32 == BUTTON2_PRESSED as u32 {
					Event::MouseDown(event.x as u32, event.y as u32,
						MouseButton::Middle)
				} else if event.bstate & BUTTON2_RELEASED as u32 == BUTTON2_RELEASED as u32 {
					Event::MouseUp(event.x as u32, event.y as u32,
						MouseButton::Middle)
				} else if event.bstate & BUTTON3_PRESSED as u32 == BUTTON3_PRESSED as u32 {
					Event::MouseDown(event.x as u32, event.y as u32,
						MouseButton::Right)
				} else if event.bstate & BUTTON3_RELEASED as u32 == BUTTON3_RELEASED as u32 {
					Event::MouseUp(event.x as u32, event.y as u32,
						MouseButton::Right)
				} else {
					self.poll_event()
				}
			},
			ncurses::KEY_RESIZE => {
				let (width, height) = self.size();
				Event::Resize(width, height)
			},
			key => {
				match char::from_u32(key as u32) {
					Some(ch) => Event::Character(ch),
					_ => self.poll_event(),
				}
			},
		}
	}
}

impl Drawable for Terminal {
	fn character(&self, ch: AlternativeCharacter, x: u32, y: u32,
			attributes: Attributes) {
		attributes.set();
		ncurses::mvaddch(y as i32, x as i32, ch.to_ncurses());
		attributes.unset();
	}

	fn write(&self, string: &str, x: u32, y: u32, attributes: Attributes) {
		attributes.set();
		ncurses::mvaddstr(y as i32, x as i32, string);
		attributes.unset();
	}

	fn clear(&self, background: Color) {
		let attributes = Attributes::background(background);
		attributes.set();
		ncurses::bkgd(ncurses::COLOR_PAIR(1));
		attributes.unset();
	}

	fn move_cursor(&self, x: u32, y: u32) {
		ncurses::mv(y as i32, x as i32);
	}

	fn size(&self) -> (u32, u32) {
		let mut width = 0;
		let mut height = 0;
		ncurses::getmaxyx(ncurses::stdscr(), &mut height, &mut width);
		(width as u32, height as u32)
	}

	fn position(&self) -> (u32, u32) {
		(0, 0)
	}

	fn draw(&self) {
		ncurses::refresh();
	}
}

impl Drop for Terminal {
	fn drop(&mut self) {
		ncurses::endwin();
	}
}


/// A window which can be drawn to.
pub struct Window {
	backend: ncurses::WINDOW,
}

impl Window {
	/// Creates a new window of the specified size, starting at the given
	/// location.
	pub fn new(x: u32, y: u32, width: u32, height: u32) -> Window {
		Window {
			backend: ncurses::newwin(height as i32, width as i32, y as i32, x as i32),
		}
	}

	/// Moves a window to a new location.
	pub fn set_position(&self, x: u32, y: u32) {
		ncurses::mvwin(self.backend, y as i32, x as i32);
	}
}

impl Drawable for Window {
	fn character(&self, ch: AlternativeCharacter, x: u32, y: u32,
			attributes: Attributes) {
		attributes.set();
		ncurses::mvwaddch(self.backend, y as i32, x as i32, ch.to_ncurses());
		attributes.unset();
	}

	fn write(&self, string: &str, x: u32, y: u32, attributes: Attributes) {
		attributes.set();
		ncurses::mvwaddstr(self.backend, y as i32, x as i32, string);
		attributes.unset();
	}

	fn clear(&self, background: Color) {
		let attributes = Attributes::background(background);
		attributes.set();
		ncurses::wbkgd(self.backend, ncurses::COLOR_PAIR(1));
		attributes.unset();
	}

	fn move_cursor(&self, x: u32, y: u32) {
		ncurses::wmove(self.backend, y as i32, x as i32);
	}

	fn size(&self) -> (u32, u32) {
		let mut width = 0;
		let mut height = 0;
		ncurses::getmaxyx(self.backend, &mut height, &mut width);
		(width as u32, height as u32)
	}

	fn position(&self) -> (u32, u32) {
		let mut x = 0;
		let mut y = 0;
		ncurses::getyx(self.backend, &mut y, &mut x);
		(x as u32, y as u32)
	}

	fn draw(&self) {
		ncurses::wrefresh(self.backend);
	}
}
