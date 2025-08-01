use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{output::OutputSink, shared_state::SharedTestState};
#[allow(dead_code)]
pub struct TestOutput {
    writer: Option<Box<dyn Write>>,
    shared_state: Rc<RefCell<SharedTestState>>,
}

impl TestOutput {
    pub fn new(shared_state: Rc<RefCell<SharedTestState>>) -> Self {
        TestOutput {
            writer: None,
            shared_state,
        }
    }
}

#[allow(unused_variables)]
impl OutputSink for TestOutput {
    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        todo!("Implement fatal_error method")
    }

    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        self.shared_state
            .borrow_mut()
            .guesses
            .push(guess.to_string());
        Ok(())
    }

    fn out_of_guesses(&mut self) -> Result<(), std::io::Error> {
        todo!("Implement out_of_guesses method")
    }
}
