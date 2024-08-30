use crate::bigfloat::bigfloat::BigFloat;
impl BigFloat {
    pub fn new(sign: bool, vals: Vec<i8>, decimal: i64) -> BigFloat{
        //Quickly check if the decimal point is valid
        if decimal< 0 {
            panic!("ValueError: Decimal point position is negative! Position:{}", decimal);
        }
        BigFloat {
            sign,
            vals,
            decimal,
        }
    }
    pub fn zero()->BigFloat{
        BigFloat::new(true,vec![],0)
      }
}