use crate::bigfloat::bigfloat::BigFloat;
impl PartialEq for BigFloat{
    fn eq(&self, other: &Self)-> bool{
        self.vals == other.vals && self.sign == other.sign && self.decimal == other.decimal
    }
    
}
impl BigFloat{
    pub fn greater_than(a:&BigFloat,b:&BigFloat)->bool{
        if a.sign!=b.sign {
            return a.sign;
        }
        if a.decimal>b.decimal{
            return true;
        }
        if a.decimal<b.decimal{
            return false;
        }
        for i in 0..a.vals.len(){
            if a.vals[i]>b.vals[i]{
                return true;
            }
            if a.vals[i]<b.vals[i]{
                return false;
            }
        }
        false
      }
      pub fn is_zero(&self)->bool{
        for i in &self.vals{
            if *i != 0{
                return false;
            }
        }
        true
      } 
}