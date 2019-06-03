// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod pty;
pub use self::pty::{Pty, PtyClass};

mod terminal;
pub use self::terminal::{Terminal, TerminalClass, NONE_TERMINAL};
pub use self::terminal::TerminalExt;

mod enums;
pub use self::enums::CursorBlinkMode;
pub use self::enums::CursorShape;
pub use self::enums::EraseBinding;
#[cfg(any(feature = "v0_50", feature = "dox"))]
pub use self::enums::Format;
pub use self::enums::PtyError;
#[cfg(any(feature = "v0_52", feature = "dox"))]
pub use self::enums::TextBlinkMode;
pub use self::enums::WriteFlags;

mod flags;
pub use self::flags::PtyFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::TerminalExt;
}
