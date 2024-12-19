#[derive(Debug)]

pub struct BigFloat{
  pub sign: bool,
  pub vals: Vec<i8>,
  pub decimal: usize,
}
/* 
//Free space for renting
/////////////////////////////////////
///               |
///               @
/// 
///      what do you want to do today?
/// (^)>
////////////////////////////////////
/// */