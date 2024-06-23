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
        if decimal as usize> vals.len(){
            panic!("ValueError: Decimal point postion is beyond length of number! Position:{}, Length of number: {}", decimal, vals.len());
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
        let difference_a = a.vals.len() as i64 - a.decimal ;
        let difference_b = b.vals.len() as i64 - b.decimal ;
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
        while (self.vals[i] ==0 || self.vals[self.vals.len()-1-i] == 0) {
            if self.vals[i]==0{
                self.vals.drain(0..0);
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
                println!("{} - {}",b.to_string(), a.to_string());
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
      }
      pub fn splice_stepped(data: &Vec<i8>) -> Vec<Vec<i8>> {
        (0..2).map(|offset|
            data
            .iter()
            .copied()
            .skip(offset)
            .step_by(2)
            .collect()
        ).collect()
    }
    pub fn mul_2(a:&Vec<i8>,b:&Vec<i8>) -> i16{
        let mut a_comp:i16 = 0;
        let mut b_comp:i16 = 0;
        if a.len()==2{
            a_comp = (a[0]*10+a[1]) as i16;
        }
        if a.len()==1{
            a_comp = a[0] as i16;
        }
        if b.len() == 2{
            b_comp = (b[0]*10+b[1]) as i16;
        }
        return a_comp*b_comp;
    }
    pub fn mul_mut(a:&mut BigFloat,b:&mut BigFloat) -> BigFloat{
        BigFloat::normalize_mut(a, b);
        if a.is_zero() || b.is_zero(){
            return BigFloat::zero();
        }
        if (a.vals.len()<=2 && b.vals.len()<=2){
            let mult = BigFloat::mul_2(&a.vals, &b.vals);
            
        }
        println!("{:?}",BigFloat::splice_stepped(&a.vals));
        println!("{:?}",BigFloat::splice_stepped(&b.vals));
        
        return BigFloat::zero();
    }
  } 
  
}