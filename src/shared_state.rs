#[derive(Default)]
pub struct SharedTestState {
    pub guesses: Vec<String>,
}

impl SharedTestState {
    pub fn new() -> Self {
        Self {
            guesses: Vec::new(),
        }
    }
}
