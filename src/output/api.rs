use std::io::Write;

use crate::output::OutputSink;

pub struct ApiOutput<W: Write> {
    writer: W,
}
#[allow(unused_variables)]
impl<W: Write> OutputSink for ApiOutput<W> {
    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        todo!("output_guess")
    }

    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        todo!("fatal_error")
    }

    fn welcome(&mut self, first_guess: &str) -> Result<(), std::io::Error> {
        todo!("welcome")
    }
}
