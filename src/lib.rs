pub mod database;
pub struct Word {
    word: String,
    times_seen: i32,
    probability: f32,
}
 impl Word{
    pub fn new(word: &str, times_seen: i32, probability: f32) -> Word{
        Word{
            word: String::from(word),
            times_seen,
            probability,
        }
    }
    pub fn get_times_seen(&self)-> i32{
        self.times_seen
    }
    pub fn get_probability(&self)-> f32{
        self.probability
    }
    pub fn get_word(&self)-> String{
        self.word.clone()
    }
    pub fn increment_times_seen(&mut self){
        self.times_seen += 1;
    }
    pub fn increment_probability(&mut self){
        self.probability += 1.0;
    }
}
pub struct Character{
    character: String,
    times_seen: i32,
    probability: f32,
}

impl Character{
   pub   fn new(character: &str, times_seen: i32, probability: f32) -> Character{
        Character{
            character: String::from(character),
            times_seen,
            probability,
        }
    }
    pub fn get_times_seen(&self)-> i32{
        self.times_seen
    }
     pub fn get_probability(&self)-> f32{
        self.probability
    }
    pub  fn get_character(&self)-> String{
        self.character.clone()
    }   
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn word_increment_times_seen_test() {
        let mut word = Word::new("hello", 1, 1.0);
        word.increment_times_seen();
        assert_eq!(word.get_times_seen(), 2);
    }
    
    #[test]
    fn test_times_seen(){
        let word = Word::new("hello", 1, 1.0);
        assert_eq!(word.get_times_seen(), 1);
    }
}


