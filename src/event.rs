
//
//  Event
//

/// All possible keys to press.
pub enum Key {
	Up,
	Down,
	Left,
	Right,
	Home,
	End,
	PageUp,
	PageDown,
	Backspace,
	Enter,
	Escape,
	Tab,
	F(u32),
}

/// Possible mouse buttons that can be pressed.
pub enum MouseButton {
	Left,
	Middle,
	Right,
}

/// All events that can occur.
pub enum Event {
	/// When a non-character key is pressed.
	Key(Key),

	/// When a character is key is pressed.
	Character(char),

	/// When the terminal window is resized. Passed the new width and height of
	/// the terminal.
	Resize(u32, u32),

	/// When a mouse button is pressed.
	MouseDown(u32, u32, MouseButton),

	/// When a mouse button is released after being pressed.
	MouseUp(u32, u32, MouseButton),
}
