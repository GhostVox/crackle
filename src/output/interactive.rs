use crate::output::OutputSink;
use std::io::Write;

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
    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        let msg = format!("Fatal error: {msg}");
        writeln!(self.writer, "{msg}")?;
        Ok(())
    }
    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        writeln!(self.writer, "Guess: {guess}\r\n")?;
        Ok(())
    }

    fn out_of_guesses(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.writer, "Out of guesses!")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fatal_error_msg() {
        let buffer = Vec::new();
        let mut output = InteractiveOutput::new(buffer);
        output.fatal_error("Houstin we have a problem").unwrap();
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

    #[test]
    fn test_out_of_guesses_msg() {
        let buffer = Vec::new();
        let mut output = InteractiveOutput::new(buffer);
        output.out_of_guesses().unwrap();
        let msg = String::from_utf8(output.into_inner()).unwrap();
        assert!(msg.contains("Out of guesses!"));
    }
}
