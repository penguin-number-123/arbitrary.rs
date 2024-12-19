use crate::bigfloat::bigfloat::BigFloat;
/// This macro allows us to not have to make sure arrays are the same size, at the expense of more difficult algorithm design
/// 
/// Basically, get the index of the array, if it exists; otherwise, return 0.
macro_rules! safeget {
    ($vector:expr,$index:ident) => {
        $vector.get($index).copied().unwrap_or(0)
    };
}
impl BigFloat{
    /// Adds two numbers together, returns the result.
    /// 
    /// Does not support more than one addend.
    /// 
    /// If you want more than one you will need to chain them together.
    pub fn add(a: BigFloat,b: BigFloat)->BigFloat{
        let outlength:usize = std::cmp::max(a.vals.len(),b.vals.len());
        let delta = outlength - std::cmp::min(a.vals.len(),b.vals.len());
        let mut carry:i8 = 0;
        let mut new_vals:Vec<i8> = vec![0;outlength];
        let alarger = a.vals.len()>b.vals.len() ;
        if a.sign == b.sign{
            
            for i in (0..outlength).rev(){
                let mut comp = 0;
                let errf = (i<delta) as i8;
                if i>delta {
                    comp = i-delta
                }
                let aindex = if alarger { i } else {comp};
                let bindex = if alarger {comp} else{ i };
                let era =if alarger { 1 } else { 1-errf};
                let erb =if alarger { 1-errf } else { 1 };
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
            BigFloat::new(a.sign,new_vals,std::cmp::max(a.decimal,b.decimal)+carry as usize)
        }else{
            if a.vals == b.vals{
                return BigFloat::zero();
            }
            if a.sign {
                //a + (-b)
                BigFloat::sub(a,b)
            }else{
                //-a +b
                BigFloat::sub(b,a)
            }
        }
      }
}