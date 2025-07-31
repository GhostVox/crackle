use std::io::Read;

use crate::input::InputSource;

pub struct ApiInput<I: Read> {
    reader: I,
}

impl<I: Read> InputSource for ApiInput<I> {
    fn get_feedback(&mut self) -> Result<String, std::io::Error> {
        unimplemented!()
    }

    fn has_next_game(&self) -> bool {
        unimplemented!()
    }

    fn next_game(&self) {
        unimplemented!()
    }

    fn is_interactive(&self) -> bool {
        false
    }
}
