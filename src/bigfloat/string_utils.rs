use crate::bigfloat::bigfloat::BigFloat;
//impl fmt::Display for Point {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "({}, {})", self.x, self.y)
//    }
//}
impl BigFloat {
    /// Returns the a string representing the BigFloat number. 
    /// #Example:
    /// ```
    /// use Arbitrary::bigfloat::bigfloat::BigFloat;
    /// let a = BigFloat::new(true,vec![1,9,1,2,0,2,3],2);
    /// assert_eq!(a.to_string(),"19.12023".to_string());
    /// ```
    pub fn to_string(&self) -> String{
        //let's ignore the cursed expression
        //Basically we take the vector ->slice -> iterable -> string each element -> join everything
        let mut neg = "";
        if !self.sign{
            neg = "-";
        }
        if self.decimal ==0{
            return format!("{}0.{}",neg,self.vals.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""));
        }
        if self.decimal as usize == self.vals.len(){
            return format!("{}{}",neg,self.vals.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""))
        }
        let (a,b) = self.vals.split_at(self.decimal as usize);
        format!("{}{}.{}",neg,a.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""),b.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""))
    }
    ///Returns the BigFloat representation of the string.
    /// #Example:
    /// ```
    /// use Arbitrary::bigfloat::bigfloat::BigFloat;
    /// let a = BigFloat::from_str("19.12023");
    /// assert_eq!(a,BigFloat::new(true,vec![1,9,1,2,0,2,3],2));
    /// ```
    pub fn from_str(s: &str) -> BigFloat{
        let sign = s.starts_with('+') || !s.starts_with('-');
        let decimal = s.replace(['-', '+'], "").split('.').collect::<Vec<&str>>()[0].len();
        let vals = s.replace(['.', '-', '+'], "");
        let vals_split = vals.split("").collect::<Vec<&str>>();
        let new_vals = vals_split[1..vals_split.len()-1].iter().map(|&e| e.parse::<i8>().unwrap()).collect::<Vec<i8>>();
        BigFloat::new(sign,new_vals,decimal)
    }
}