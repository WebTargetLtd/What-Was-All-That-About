use chrono::Utc;
use console::{style, Term};

///
/// Prints a styled message to the terminal with a timestamp.
///
pub fn say(message: &str) -> Result<(), std::io::Error> {
    let term = Term::stdout();
    let now = Utc::now();

    let lead = style(format!("[ {} ] :: ", now.to_rfc2822(),)).color256(208);

    let msg = format!("{}{}", lead, message);
    term.write_line(&msg)?;
    Ok(())
}

/// Writes a padding line to the terminal, preceded and followed by a blank line.
pub fn paddingline() -> Result<(), std::io::Error> {

    if let Some((width, _)) = term_size::dimensions() {
        let line = style("-".repeat(width)).magenta();
        
        let blankline = ""; //  " ".repeat(width);
        let term = Term::stdout();

        term.write_line(blankline)?;
        term.write_line(format!("{}", line).as_str())?;
        term.write_line(blankline)?
    
    } 
    Ok(())
}

// Shorthand for creating a Timer.
#[macro_export]
macro_rules! say {
    ($e:expr) => { wolves_cli_helper::verbose::say($e).unwrap() };
}