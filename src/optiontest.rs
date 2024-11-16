pub fn test_option_type() ->Option<u8>{
  let  mut opt1 :Option<u8> = None;
  opt1 = Some(10);
  return opt1;
}
pub fn test_char() ->Option<CharacterType>{
  let mut opt1 :Option<CharacterType> =None;
  //opt1 = Some(CharacterType::Actor);
  return opt1;
}
pub enum CharacterType{
  Actor,
  Warrior,
}
impl ToString for CharacterType{
  fn to_string(&self) ->String{
    match self{
      CharacterType::Actor =>"Actor",
      CharacterType::Warrior =>"Warrior",
    }.to_string()
  }
}