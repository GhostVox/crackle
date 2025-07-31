use std::io::Read;

use crate::input::InputSource;

pub struct ApiInput<R: Read> {
    reader: R,
}

impl<R: Read> InputSource for ApiInput<R> {
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
