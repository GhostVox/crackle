use super::check_input;
use crate::input::InputSource;
use std::io::BufRead;
pub struct InteractiveInput<R: BufRead> {
    reader: R,
}

impl<R: BufRead> InputSource for InteractiveInput<R> {
    fn get_feedback(&mut self) -> Result<String, std::io::Error> {
        let mut input = String::new();
        self.reader.read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input == "exit" {
            println!("Exiting game");
            std::process::exit(0);
        }
        let input_ok = check_input(&input);
        match input_ok {
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )),
            Ok(_) => Ok(input),
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
