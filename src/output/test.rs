use std::io::Write;

use crate::output::OutputSink;

pub struct TestOutput<W: Write> {
    writer: W,
}

#[allow(unused_variables)]
impl<W: Write> OutputSink for TestOutput<W> {
    fn welcome(&mut self, first_guess: &str) -> Result<(), std::io::Error> {
        todo!("Implement welcome method")
    }

    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        todo!("Implement fatal_error method")
    }

    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        todo!("Implement output_guess method")
    }
}
