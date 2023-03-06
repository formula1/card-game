pub trait Tokenizer {
  fn token_type(&mut self)->String;
  fn matchesChar(&mut self, c: char)->bool;
  fn handlChar(
    &mut self,
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  )->Result<(), String>;
}
