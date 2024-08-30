use crate::bigfloat::bigfloat::BigFloat;
macro_rules! safeget {
    ($vector:expr,$index:ident) => {
        $vector.get($index).copied().unwrap_or(0)
    };
}
impl BigFloat{
    ///Implementation of the quadratic multiplication algorithm.
    /// 
    /// Runs in O(n^2) time, used for small numbers <20 digits in length
    /// 
    /// For larger numbers use karatsuba multiplication, or ~Schonehage~ Schönhage–Strassen algorithm.
    pub fn quad_mult(a: BigFloat,b: BigFloat)->BigFloat{
        let mut new_vals:Vec<i8> = vec![0;a.vals.len()+b.vals.len()];
        let _k = 0;
        let l_max = new_vals.len()-1;
        for j in 0..b.vals.len(){
            let b_index = b.vals.len()-1-j;
            //println!("Index {} is {}",j,b.vals[b_index]);
            if safeget!(b.vals,b_index) == 0{
                continue;
            }
            let mut k =0;
            for i in 0..a.vals.len(){
                let a_index = a.vals.len()-1-i;
                let t=safeget!(a.vals,a_index)*safeget!(b.vals,b_index)+k+new_vals[l_max-(i+j)];
                new_vals[l_max-(i+j)] = t%10;
                k = t/10;
            }
            new_vals[l_max-a.vals.len()-j]=k;
        }
        if new_vals[0] == 0{
            new_vals.drain(0..1);
            return BigFloat::new((a.sign&&b.sign)||(!a.sign&&!b.sign),new_vals,a.decimal+b.decimal-1);
        }
        BigFloat::new((a.sign&&b.sign)||(!a.sign&&!b.sign),new_vals,a.decimal+b.decimal)
    }
    pub fn k_mul(a: BigFloat, b: BigFloat)  -> BigFloat {
    //Modified to choose the split point as the smaller of the length of a and b /2

    }
}