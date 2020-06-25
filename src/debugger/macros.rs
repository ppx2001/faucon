use std::error::Error;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Prints a success message.
macro_rules! ok {
    ($title:expr, $msg:expr) => {
        $crate::debugger::macros::print($title, $msg, termcolor::Color::Green).unwrap();
    };

    ($title:expr, $msg:expr, $($arg:tt)*) => {
        ok!($title, format!($msg, $($arg)*).as_str())
    };
}

/// Prints an info message.
macro_rules! info {
    ($title:expr, $msg:expr) => {
        $crate::debugger::macros::print($title, $msg, termcolor::Color::Cyan).unwrap();
    };

    ($title:expr, $msg:expr, $($arg:tt)*) => {
        info!($title, format!($msg, $($arg)*).as_str())
    };
}

/// Prints an error message.
macro_rules! error {
    ($title:expr, $msg:expr) => {
        $crate::debugger::macros::print($title, $msg, termcolor::Color::Red).unwrap();
    };

    ($title:expr, $msg:expr, $($arg:tt)*) => {
        error!($title, format!($msg, $($arg)*).as_str())
    };
}

/// Prints a colored message.
pub(super) fn print(title: &str, msg: &str, color: Color) -> Result<(), Box<dyn Error>> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    stdout.set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;

    write!(stdout, "{:<15}", title)?;

    stdout.reset()?;
    writeln!(stdout, " {}", msg)?;
    stdout.flush()?;

    Ok(())
}
