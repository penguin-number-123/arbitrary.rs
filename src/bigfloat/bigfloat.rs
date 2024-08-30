
  #[derive(Debug)]
  //sign: false == negative, true == positive
  pub struct BigFloat{
    pub sign: bool,
    pub vals: Vec<i8>,
    pub decimal: i64,
  }
