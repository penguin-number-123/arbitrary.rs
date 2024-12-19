use crate::bigfloat::bigfloat::BigFloat;
impl BigFloat {
    /// BigFloat: Basically floating point, but using a vector for the values.
    /// ## Panics
    /// Panics when the decimal point is less than zero.
    /// ## Arguments
    /// * Sign (bool): False for negative numbers, True for positive numbers.
    /// * Vals (vector): Stores the values of the number.
    /// * Decimal: Indicates the position of the decimal point.
    /// 
    /// Brief Example: 12.34 = ```(True, [ 1, 2, 3, 4 ],2)```
    /// ## Example
    /// ```
    /// use Arbitrary::bigfloat::bigfloat::BigFloat;
    /// let a = BigFloat::new(true,vec![1,2,3,4],2);
    /// ```
    /// For faster input, try using ```BigFloat::from_str(&str)``` instead.
    pub fn new(sign: bool, vals: Vec<i8>, decimal:usize) -> BigFloat{
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