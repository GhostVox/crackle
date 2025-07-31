use std::io::Read;

use crate::input::InputSource;

#[allow(dead_code)]
pub struct TestInput<R: Read> {
    reader: R,
}

impl<R: Read> TestInput<R> {
    pub fn new(reader: R) -> Self {
        TestInput { reader }
    }
}

impl<R: Read> InputSource for TestInput<R> {
    fn get_feedback(&mut self) -> Result<String, std::io::Error> {
        Ok(String::new())
    }

    fn has_next_game(&self) -> bool {
        false
    }

    fn next_game(&self) {
        todo!()
    }

    fn is_interactive(&self) -> bool {
        false
    }
}
