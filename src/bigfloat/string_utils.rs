use crate::bigfloat::bigfloat::BigFloat;
impl BigFloat {
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
    pub fn from_str(s: &str) -> BigFloat{
        let sign = s.starts_with('+') || !s.starts_with("-");
        println!("{}",!s.starts_with("-"));
        let decimal = s.replace(['-', '+'], "").split('.').collect::<Vec<&str>>()[0].len();
        let vals = s.replace(['.', '-', '+'], "");
        let vals_split = vals.split("").collect::<Vec<&str>>();
        let new_vals = vals_split[1..vals_split.len()-1].iter().map(|&e| e.parse::<i8>().unwrap()).collect::<Vec<i8>>();
        BigFloat::new(sign,new_vals,decimal as i64)
    }
}