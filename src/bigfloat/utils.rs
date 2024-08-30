use crate::bigfloat::bigfloat::BigFloat;
impl BigFloat{
  pub fn get_vals(&mut self) -> Vec<i8> {
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
  pub fn invert_sign(a: BigFloat) -> BigFloat{
    BigFloat::new(!a.sign,a.vals,a.decimal)
  }
  
  pub fn invert(mut self){
    self.sign = !self.sign;
  }
  pub fn lshift(&mut self,digits:i64){
    let mut zeroes:Vec<i8> = vec![0;digits as usize];
    self.vals.append(&mut zeroes);
    self.decimal += digits as i64;
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
}
