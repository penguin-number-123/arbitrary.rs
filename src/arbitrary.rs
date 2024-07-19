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
      
    pub fn normalize_mut(a:&mut BigFloat, b:&mut BigFloat){
        //Need to consider decimal point and the lenght of the array
        //1,2,3,4,5 pos = 5 and 1,2,3 pos 0 are 12345 and 0.123
        //Then, we need to pad 1,2,3 into 0,0,0,0,0,1,2,3 and 1,2,3,4,5,0,0,0
        //1,2,3,4,5 pos 2 and 1,2,3,4 pos 4 will become 12.345 and 1234
        //We need 0,0,1,2,3,4,5 (+2) and 1,2,3,4,0,0,0 (+3 = length vals - decimal point) 
        let difference_a = a.vals.len() as i64 - a.decimal -1;
        let difference_b = b.vals.len() as i64 - b.decimal -1;
        let pad_zeroes_back:Vec<i8> = vec![0;(difference_a-difference_b).abs() as usize];
        let mut pad_zeroes_front:Vec<i8> = vec![0;(a.decimal-b.decimal).abs() as usize];
        if difference_b<difference_a{
            //b has more numbers after decimal point
            b.vals.extend(pad_zeroes_back);
        }else if difference_b>difference_a{
            a.vals.extend(pad_zeroes_back);
        }
        if b.decimal<a.decimal{
            pad_zeroes_front.extend(b.vals.clone());
            b.decimal += a.decimal-b.decimal;
            b.vals = pad_zeroes_front.clone();
        }else if b.decimal>a.decimal{
            pad_zeroes_front.extend(a.vals.clone());
            a.decimal += b.decimal-a.decimal;
            a.vals = pad_zeroes_front.clone();
            
        }
        
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

      pub fn add_mut(a:&mut BigFloat,b:&mut BigFloat)->BigFloat{
        BigFloat::normalize_mut(a, b);
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()];
        if a.sign == b.sign{
        
            for i in (0..a.vals.len()).rev(){
                new_vals[i] = (a.vals[i] + b.vals[i] + carry )%10;
                if(a.vals[i]+b.vals[i]+carry) >= 10{
                    carry = 1;
                }else{
                    carry = 0;
                }
            }
            if carry == 1{
                new_vals = [Vec::from([1]),new_vals].concat();
                
            }
            return BigFloat::new(a.sign,new_vals,a.decimal+carry as i64);
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
      
      pub fn invert(&mut self){
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
      
      pub fn sub_mut(a:&mut BigFloat,b:&mut BigFloat)->BigFloat{
        BigFloat::normalize_mut(a, b);
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()];
        
        if a.sign == b.sign{
            if a.vals == b.vals{
                return BigFloat::zero();
            }
            if BigFloat::greater_than(b, a){
                //println!("{} - {}",b.to_string(), a.to_string());
                let mut result = BigFloat::sub_mut(b, a);
                result.invert();
                return result;
            }
            for i in (0..a.vals.len()).rev(){
                new_vals[i] = ((a.vals[i] - b.vals[i] + carry )%10+10)%10;
                if (a.vals[i]-b.vals[i]+carry)<0{
                    carry = -1;
                }else{
                    carry = 0;
                }
            }
            

            return BigFloat::new(a.sign,new_vals,a.decimal);
        } else{
            if !b.sign {
                //a-(-b) = a+b
                b.invert();
                return BigFloat::add_mut(a, b);
            }else{
                //-a-b = -(a+b)
                a.invert();
                let mut result = BigFloat::add_mut(a,b);
                result.invert();
                return result;
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
    
    pub fn karatsuba(a:&mut BigFloat,b:&mut BigFloat)-> BigFloat{
        BigFloat::normalize_mut(a, b);
        if a.vals==[] || b.vals==[]{
            return BigFloat::zero();
        }
        if a.is_zero() || b.is_zero(){
            return BigFloat::zero();
        }
        if (a.vals.len()<=30 || b.vals.len()<=30){
            return BigFloat::quad_mult(a, b)
        }
        let mut splice_a = BigFloat::splice_stepped(&a.vals);
        let mut splice_b = BigFloat::splice_stepped(&b.vals);
        
        let mut first_a = BigFloat::new(true,splice_a[0].to_owned(),a.decimal - splice_a[1].len() as i64 );
        let mut first_b = BigFloat::new(true,splice_b[0].to_owned(),b.decimal - splice_b[1].len() as i64);
        let mut sec_b = BigFloat::new(true,splice_b[1].to_owned(),splice_b[1].len() as i64);
        let mut sec_a = BigFloat::new(true,splice_a[1].to_owned(),splice_a[1].len() as i64);
        //println!("First a {}",first_a.to_string());
        //println!("First b {}",first_b.to_string());
        //println!("sec a {}",sec_a.to_string());
        //println!("sec a {}",sec_a.to_string());
        let mut i = BigFloat::add_mut(&mut first_a,&mut sec_a);
        let mut j = BigFloat::add_mut(&mut first_b,&mut sec_b);
        let mut k_2 = BigFloat::karatsuba(&mut first_a,&mut first_b);
        //println!("k2 a {}",k_2.to_string());
        let mut k_0 = BigFloat::karatsuba(&mut sec_a,&mut sec_b);
        let mut ij = BigFloat::karatsuba(&mut BigFloat::add_mut(&mut first_a,&mut sec_a),&mut BigFloat::add_mut(&mut first_b,&mut sec_b));
        let mut k_1 = BigFloat::sub_mut(&mut BigFloat::sub_mut(&mut ij,&mut k_2.clone()) ,&mut k_0);
        k_2.lshift(splice_a[1].len() as i64 *2-1);
        k_1.lshift(splice_a[1].len() as i64);
        //println!("k0 a {}",k_0.to_string());
        //println!("k1 b {}",k_1.to_string());
        //println!("k2 shifted {}",k_2.to_string());
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
                y = BigFloat::quad_mult(&mut a_p, &mut y);
                
                *b = *b-1;
            }
            let mut a_2 = a_p.clone();
            a_p =  BigFloat::quad_mult(&mut a_p, &mut a_2);
            *b = *b/2
        }
        return BigFloat::quad_mult(&mut a_p, &mut y);
        
    }

   
  } 
  
}