use std::cmp::max;
macro_rules! safeget {
    ($vector:expr,$index:ident) => {
        $vector.get($index).copied().unwrap_or(0)
    };
}
pub mod arbitrary{
  //sign: True -> Positive, False -> Negative
  //Vals: Stores the digits of the number.
  //Decimal: Stores position of the decimal point
  pub struct BigFloat{
    sign: bool,
    vals: Vec<i8>,
    decimal: i64,
  }

  impl PartialEq for BigFloat{
    fn eq(&self, other: &Self)-> bool{
        self.vals == other.vals
    }
  }
  
  impl BigFloat {
    pub fn new(sign: bool, vals: Vec<i8>, decimal: i64) -> BigFloat{
        //Quickly check if the decimal point is valid
        if decimal< 0 {
            panic!("ValueError: Decimal point position is negative! Position:{}", decimal);
        }
        
        BigFloat {
            sign:sign,
            vals:vals,
            decimal:decimal,
        }
    }
    
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
        return format!("{}{}.{}",neg,a.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("").to_string(),b.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""));
      }
      
    pub fn from_str(s: &str) -> BigFloat{
        let sign = s.chars().nth(0).unwrap() =='+' || s.chars().nth(0).unwrap() !='-';
        let decimal = s.replace("-","").replace("+","").split(".").collect::<Vec<&str>>()[0].len();
        let vals = s.replace(".","").replace("-","").replace("+","");
        let vals_split = vals.split("").collect::<Vec<&str>>();
        let new_vals = vals_split[1..vals_split.len()-1].iter().map(|&e| e.parse::<i8>().unwrap()).collect::<Vec<i8>>();
        return BigFloat::new(sign,new_vals,decimal as i64);
      }
      
   
      
    pub fn zero()->BigFloat{
        return BigFloat::new(true,vec![],0);
      }
      
      pub fn get_vals(&mut self) -> Vec<i8> {
        return self.vals.clone();
      }
    
      pub fn clear_zeroes_self(&mut self) ->&mut BigFloat {
        let mut i:usize = 0;
        while ((self.vals[i] == 0 || self.vals[self.vals.len()-1-i] == 0)) {
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
        
        return self
      }
      pub fn clean_zeroes(a:BigFloat) ->BigFloat {
        let mut i:usize= 0;
        let mut new_vals:Vec<i8> = a.vals;
        let mut max_size = new_vals.len();
        let mut decimal = a.decimal;
        while (new_vals[i] ==0 || new_vals[max_size-1-i] == 0) {
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
        return BigFloat::new(a.sign,new_vals,decimal);
      }

      pub fn add(a: BigFloat,b: BigFloat)->BigFloat{
        //BigFloat::normalize_mut(a, b);
        let outlength:usize = std::cmp::max(a.vals.len(),b.vals.len());
        let delta = outlength - std::cmp::min(a.vals.len(),b.vals.len());
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;outlength];
        let alarger = (a.vals.len()>b.vals.len()) ;
        if a.sign == b.sign{
            
            for i in (0..outlength).rev(){
                let mut comp = 0;
                let errf = (i<delta) as i8;
                if(i>delta){
                    comp = (i-delta)
                }
                let aindex = if alarger { i } else {comp};
                let bindex = if alarger {comp} else{ i };
                let era =if alarger { 1 } else { (1-errf)};
                let erb =if alarger { (1-errf) } else { 1 };
                new_vals[i] = (safeget!(a.vals,aindex)*era+ safeget!(b.vals,bindex)*erb + carry )%10;
                if(safeget!(a.vals,aindex)*era+safeget!(b.vals,bindex)*erb+carry) >= 10{
                    carry = 1;
                }else{
                    carry = 0;
                }
                
            }
            if carry == 1{
                new_vals = [Vec::from([1]),new_vals].concat();
                
            }
            return BigFloat::new(a.sign,new_vals,std::cmp::max(a.decimal,b.decimal)+carry as i64);
        }else{
            if a.vals == b.vals{
                return BigFloat::zero();
            }
            return BigFloat::zero();
        }
      }
      
      pub fn invert_sign(a: BigFloat) -> BigFloat{
        return BigFloat::new(!a.sign,a.vals,a.decimal)
      }
      
      pub fn invert(mut self){
        self.sign = !self.sign;
      }
      
      pub fn greater_than(a:&BigFloat,b:&BigFloat)->bool{
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
        return false;
      }
   
      pub fn sub(a: BigFloat,b: BigFloat)->BigFloat{
        let outlength:usize = std::cmp::max(a.vals.len(),b.vals.len());
        let delta = outlength - std::cmp::min(a.vals.len(),b.vals.len());
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()];
        let alarger:usize = (a.vals.len()>b.vals.len()) as usize;
        if a.sign == b.sign{
            if a.vals == b.vals{
                return BigFloat::zero();
            }
            if BigFloat::greater_than(&b, &a){
                //println!("{} - {}",b.to_string(), a.to_string());
                return BigFloat::invert_sign(BigFloat::sub(b, a));
            }
            //println!("d: {}, al {}",delta.to_string(), alarger.to_string());
            for i in (0..outlength).rev(){
                let mut comp = 0;
                let errf = i<delta;
                if(i>delta){
                    comp = i-delta;
                }
                //println!("check: {:?}",(1-errf as i8));
                //println!("bval: {:?}",b.vals.get(comp).copied().unwrap_or(0)* (1-errf as i8));
                //println!("aval: {:?}",a.vals.get(i).copied().unwrap_or(0));
                new_vals[i] = ((a.vals.get(i).copied().unwrap_or(0) - b.vals.get(comp).copied().unwrap_or(0) * (1-errf as i8) + carry )%10+10)%10;
                //println!("carry was: {:?}",carry);
                if (a.vals.get(i).copied().unwrap_or(0)-b.vals.get(comp).copied().unwrap_or(0)+carry)<0{
                    carry = -1;
                }else{
                    carry = 0;
                }
            }
            return BigFloat::new(a.sign,new_vals,a.decimal);
        } else{
            if !b.sign {
                //a-(-b) = a+b
                
                return BigFloat::add(a, b);
            }else{
                //-a-b = -(a+b)
                return BigFloat::invert_sign(BigFloat::add(a,b));
            }
        }
      }
      
      pub fn is_zero(&self)->bool{
        for i in &self.vals{
            if *i != 0{
                return false;
            }
        }
        return true;
      } 
      
      pub fn lshift(&mut self,digits:i64){
        let mut zeroes:Vec<i8> = vec![0;digits as usize];
        self.vals.append(&mut zeroes);
        self.decimal += digits as i64;
      }
      pub fn splice_stepped(data: &Vec<i8>) ->[Vec<i8>;2] {
        let mut v0 =data.clone();
        let v = v0.split_off((data.len()+1)/2);
        return [v0, v];
    }
    pub fn quad_mult(a:&mut BigFloat,b:&mut BigFloat)->BigFloat{
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()+b.vals.len()];
        let mut k = 0;
        let l_max = new_vals.len()-1;
        for j in 0..b.vals.len(){
            let b_index = b.vals.len()-1-j;
            //println!("Index {} is {}",j,b.vals[b_index]);
            if b.vals[b_index] == 0{
                continue;
            }
            let mut k =0;
            for i in 0..a.vals.len(){
                let a_index = a.vals.len()-1-i;
                let t=a.vals[a_index]*b.vals[b_index]+k+new_vals[l_max-(i+j)];
                new_vals[l_max-(i+j)] = t%10;
                k = t/10;
            }
            new_vals[l_max-a.vals.len()-j]=k;
        }
        if new_vals[0] == 0{
            new_vals.drain(0..1);
            return BigFloat::new((a.sign&&b.sign)||(!a.sign&&!b.sign),new_vals,a.decimal+b.decimal-1);
        }
        return BigFloat::new((a.sign&&b.sign)||(!a.sign&&!b.sign),new_vals,a.decimal+b.decimal);
    }
    fn number_to_vec(n: i16) -> Vec<i8> {
        let mut digits:Vec<i8> = Vec::new();
        let mut n = n;
        while n > 9 {
            digits.push((n % 10) as i8);
            n = n / 10;
        }
        digits.push(n as i8);
        digits.reverse();
        digits
    }
    /*
    pub fn karatsuba(u: BigFloat,v: BigFloat)-> BigFloat{
        let mut a = u.clone();
        let mut b = v.clone();
        //BigFloat::normalize_mut(&mut a, &mut b);
        println!("a {}",a.to_string());
        println!("b {}",b.to_string());
        if a.vals==[] || b.vals==[]{
            return BigFloat::zero();
        }
        if a.is_zero() || b.is_zero(){
            return BigFloat::zero();
        }
        if (a.vals.len()<=30 || b.vals.len()<=30){
            return BigFloat::quad_mult(&mut a, &mut b)
        }
        let splice_pos = std::cmp::min(a.vals.len()/2,b.vals.len()/2)+1;
        println!("{}",splice_pos);
        let lesser = std::cmp::min(a.vals.len(),b.vals.len());
        let m = a.vals.split_off(splice_pos);
        let n =b.vals.split_off(splice_pos);
        let lena = a.vals.len();
        let lenb = b.vals.len();
        let lenn = n.len();
        let lenm = m.len();
        let mut first_a = BigFloat::new(true,a.vals, (splice_pos)  as i64 );
        let mut first_b = BigFloat::new(true,b.vals,(splice_pos)  as i64);
        let mut sec_b = BigFloat::new(true,n,b.decimal-splice_pos as i64);
        let mut sec_a = BigFloat::new(true,m,a.decimal-splice_pos as i64);
        println!("First a {}",first_a.to_string());
        println!("First b {}",first_b.to_string());
        println!("sec a {}",sec_a.to_string());
        println!("sec b {}",sec_b.to_string());
        let mut k_2 = BigFloat::karatsuba(first_a.clone(), first_b.clone());
        //println!("k2 a {}",k_2.to_string());
        let mut k_0 = BigFloat::karatsuba( sec_a.clone(), sec_b.clone());
        println!("k0 a {}",k_0.to_string());
        let mut ij = BigFloat::karatsuba( BigFloat::add_mut(&mut first_a,&mut sec_a), BigFloat::add_mut(&mut first_b,&mut sec_b));
        
        let mut k_1 = BigFloat::sub_mut(&mut BigFloat::sub_mut(&mut ij,&mut k_2.clone()) ,&mut k_0.clone());

        
        println!("k1 b {}",k_1.to_string());
        println!("k2 shifted {}",k_2.to_string());
        k_2.lshift( splice_pos as i64 *2+2);
        k_1.lshift( splice_pos as i64+1);
        
        return BigFloat::add_mut(&mut BigFloat::add_mut(&mut k_2,&mut k_1),&mut k_0);
    }
    pub fn clone(&self) -> BigFloat {
        return BigFloat::new(self.sign,self.vals.clone(),self.decimal);
    }  
    pub fn int_exp(a:&mut BigFloat, b:&mut i128) ->BigFloat{
        
        
        let mut a_p = a.clone();
        if *b == 0{
            return BigFloat::new(true,vec![1],1);
        }
        let mut y = BigFloat::new(true,vec![1],1);
        while *b>1{
            println!("b{:b}",b);
            if *b%2==1{
                let mut a_3 = a_p.clone();
                y = BigFloat::karatsuba( a_3,   y);
                
                *b = *b-1;
            }
            let a_2 = a_p.clone();
            a_p =  BigFloat::karatsuba(a_p, a_2);
            println!("{:?}",a_p.to_string());
            *b = *b/2
        }
        return BigFloat::karatsuba( a_p,  y);
        
    }

   
  } 
  */
}
}