use crate::bigfloat::bigfloat::BigFloat;
macro_rules! safeget {
    ($vector:expr,$index:ident) => {
        $vector.get($index).copied().unwrap_or(0)
    };
}
impl BigFloat{
pub fn sub(a: BigFloat,b: BigFloat)->BigFloat{
        let outlength:usize = std::cmp::max(a.vals.len(),b.vals.len());
        let delta = outlength - std::cmp::min(a.vals.len(),b.vals.len());
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()];
        let alarger = a.vals.len()>b.vals.len() ;
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
                let errf = (i<delta) as i8;
                if i>delta {
                    comp = i-delta;
                }
                let aindex = if alarger { i } else { comp };
                let bindex = if alarger { comp } else { i };
                let era = if alarger {1} else {1-errf};
                let erb = if alarger {1-errf} else { 1 };
                //See this is really just a complicated method for shifting the digits such that zero padding can be avoided.
                //123+1231 = 0123+1231, but instead of adding the 0, we just try to access that digit (which cannot be done), and so 0 is returned by safeget.
                //(1-errf) is a flag to force 0 whenever the position is invalid, i.e. -ve.
                //the above if statements is just a way to reverse the logic in the case that b is longer than a.
                new_vals[i] = ((safeget!(a.vals,aindex)*era - safeget!(b.vals,bindex) * erb + carry )%10+10)%10;
                //println!("carry was: {:?}",carry);
                if (a.vals.get(i).copied().unwrap_or(0)-b.vals.get(comp).copied().unwrap_or(0)+carry)<0{
                    carry = -1;
                }else{
                    carry = 0;
                }
            }
            //subtraction generats loads of leading or trailing decimals. We should attempt to eliminate this.

            BigFloat::new(a.sign,new_vals,a.decimal)
        } else if !b.sign {
            //a-(-b) = a+b
            
            BigFloat::add(a, b)
        }else{
            //-a-b = -(a+b)
            BigFloat::invert_sign(BigFloat::add(a,b))
        }
      }
    }