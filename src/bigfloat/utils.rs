use crate::bigfloat::bigfloat::BigFloat;
impl BigFloat{
  /// Returns the vecotr contain the values of the BigFloat number. Could be useful, perhaps.
  /// # Example:
  /// ```
  /// use Arbitrary::bigfloat::bigfloat::BigFloat;
  /// let a = BigFloat::new(true,vec![2,0,2,4,0,1,0,9,1,4,4,4],5);
  /// assert_eq!(a.get_vals(),vec![2,0,2,4,0,1,0,9,1,4,4,4]);
  /// ```
  pub fn get_vals(self) -> Vec<i8> {
    self.vals.clone()
  }

  pub fn clear_zeroes_self(&mut self) ->&mut BigFloat {
    let mut i:usize = 0;
    while self.vals[i] == 0 || self.vals[self.vals.len()-1-i] == 0 {
        //println!("{:?}",self.vals);
        if i==self.vals.len()-1{
            return self
        }
        if self.vals[i]==0{
            self.vals.drain(0..1);
            self.decimal -= 1;
            
        }
        if self.vals[self.vals.len()-1-i] == 0{
            self.vals.pop();

        }
        i+=1;
    }
    if self.vals[self.vals.len()-1] == 0{
        self.vals.pop();
    }
    
    self
  }
  pub fn clean_zeroes(a:BigFloat) ->BigFloat {
    let mut i:usize= 0;
    let mut new_vals:Vec<i8> = a.vals;
    let mut max_size = new_vals.len();
    let mut decimal = a.decimal;
    while new_vals[i] ==0 || new_vals[max_size-1-i] == 0 {
        if new_vals[i]==0{
            new_vals.drain(0..0);
            decimal -= 1;
            max_size -= 1;
        }
        if new_vals[max_size-1-i] == 0{
            new_vals.pop();
        }
        i+=1;
    }
    BigFloat::new(a.sign,new_vals,decimal)
  }
  /// Inverts the sign of a BigFloat, returns the inverted number.
  /// 
  /// Note this is an in-place modifier and returns `()`.
  /// # Example:
  /// ```
  /// use Arbitrary::bigfloat::bigfloat::BigFloat;
  /// let a = BigFloat::new(false,vec![1],1);
  /// let b = BigFloat::invert_sign(a);
  /// assert_eq!(b,BigFloat::new(true,vec![1],1));
  /// ```
  pub fn invert_sign(a: BigFloat) -> BigFloat{
    BigFloat::new(!a.sign,a.vals,a.decimal)
  }
  /*/// Inverts the sign of a mutable BigFloat.
  /// 
  /// Note this is an in-place modifier and returns `()`.
  /// # Example:
  /// ```
  /// use Arbitrary::bigfloat::bigfloat::BigFloat;
  /// let a = BigFloat::new(false,vec![1],1);
  /// &a.invert();
  /// assert_eq!(a,BigFloat::new(true,vec![1],1));
  /// ```
  pub fn invert(mut self){
    self.sign = !self.sign;
  }*/
   /// Does a left shift on the value of the &mutable BigFloat.
  /// 
  /// This is equivalent to the << operator, only for base 10.
  /// # Panics:
  /// If the shifted vector is larger than the maximum permissible size.
  /// 
  /// # Example:
  /// 
  /// use Arbitrary::bigfloat::bigfloat::BigFloat;
  /// let mut a = BigFloat::new(true,vec![2,0,2,4],2);
  /// &a.lshift(5);
  /// assert_eq!(a,BigFloat::new(true,vec![2,0,2,4,0,0,0,0,0],7));
  /// 
  pub fn lshift(&self,digits:usize)->BigFloat{
    let mut vals = self.vals.clone();
    let zeroes  = vec![0;digits as usize];
    vals.extend( zeroes);
    let new_vals = vals.clone();
    return BigFloat::new(self.sign,new_vals,self.decimal+digits);
  }
  pub fn splice_stepped(data: &Vec<i8>) ->[Vec<i8>;2] {
    let mut v0 =data.clone();
    let v = v0.split_off((data.len()+1)/2);
    [v0, v]
}
fn number_to_vec(n: i16) -> Vec<i8> {
    let mut digits:Vec<i8> = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10) as i8);
        n /= 10;
    }
    digits.push(n as i8);
    digits.reverse();
    digits
}
pub fn clone(&self)->BigFloat{
  return BigFloat::new(self.sign,self.vals.clone(),self.decimal)
}
}
