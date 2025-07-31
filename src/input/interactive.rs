use super::check_input;
use crate::input::InputSource;
use std::io::BufRead;
pub struct InteractiveInput<R: BufRead> {
    reader: R,
}

impl<R: BufRead> InteractiveInput<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> InputSource for InteractiveInput<R> {
    fn get_feedback(&mut self) -> Result<String, std::io::Error> {
        loop {
            let mut input = String::new();
            self.reader.read_line(&mut input)?;

            let input = input.trim().to_lowercase();
            if input == "exit" {
                println!("Exiting game");
                return Err(std::io::Error::other("Exiting game"));
            }

            let input_ok = check_input(&input);
            match input_ok {
                Ok(_) => return Ok(input),
                Err(e) => {
                    println!("Invalid input: {e}. Please try again.");
                    continue;
                }
            }
        }
    }

    fn has_next_game(&self) -> bool {
        unimplemented!()
    }

    fn next_game(&self) {
        unimplemented!()
    }
    fn is_interactive(&self) -> bool {
        true
    }
}
