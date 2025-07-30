use crate::output::OutputSink;
use colored::Colorize;
use std::io::Write;

const EXPECTED_FORMAT: &str = "gyngy";
pub struct InteractiveOutput<W: Write> {
    writer: W,
}

impl<W: Write> InteractiveOutput<W> {
    pub fn new(writer: W) -> Self {
        InteractiveOutput { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write> OutputSink for InteractiveOutput<W> {
    fn welcome(&mut self, starting_word: &str) -> Result<(), std::io::Error> {
        writeln!(self.writer, "Welcome to Crackle!\r\n")?;
        writeln!(
            self.writer,
            "I will give you a word to try based on positional frequency"
        )?;
        writeln!(
            self.writer,
            "To achieve this, you will need to enter {}, {}, and N for gray",
            "G for green".green(),
            "Y for yellow".yellow()
        )?;
        writeln!(self.writer, "Example: {}", EXPECTED_FORMAT)?;
        writeln!(
            self.writer,
            "Starting game with word: {}",
            starting_word.blue().on_white()
        )?;
        Ok(())
    }

    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        let msg = format!("Fatal error: {msg}");
        writeln!(self.writer, "{msg}")?;
        Ok(())
    }
    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        writeln!(self.writer, "Guess: {guess}\r\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_msg() {
        let buff = Vec::new();
        let mut output = InteractiveOutput::new(buff);
        output.welcome("apple").unwrap();
        let msg = String::from_utf8(output.into_inner()).unwrap();
        assert!(msg.contains("Welcome to Crackle!"));
        assert!(msg.contains("Starting game with word: apple"));
        assert!(msg.contains("Example: gyngy"));
    }

    #[test]
    fn test_fatal_error_msg() {
        let buffer = Vec::new();
        let mut output = InteractiveOutput::new(buffer);
        output.fatal_error("Houstin we have a problem");
        let msg = String::from_utf8(output.into_inner()).unwrap();
        assert!(msg.contains("Fatal error: Houstin we have a problem"));
    }

    #[test]
    fn test_output_guess_msg() {
        let buffer = Vec::new();
        let mut output = InteractiveOutput::new(buffer);
        output.output_guess("apple").unwrap();
        let msg = String::from_utf8(output.into_inner()).unwrap();
        assert!(msg.contains("Guess: apple"));
    }
}
