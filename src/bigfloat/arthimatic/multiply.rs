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
    /// # Example:
    /// ```
    /// use Arbitrary::bigfloat::bigfloat::BigFloat;
    /// let a = BigFloat::new(true,vec![1,2,7,8,9,9,2],5);
    /// let b = BigFloat::new(false,vec![3,1,4,1,5,9,2,6],6);
    /// assert_eq!(BigFloat::quad_mult(a,b),BigFloat::new(false,vec![4,0,1,8,0,7,1,8,0,2,6,5,9,2],10));
    /// ```
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
    /// Karatsuba algorithm for multiplication.
    /// Note: it automatically switches to quadratic multiplication for digit length < 20. This reduces excess memory.
    /// #Example:
    /// ```
    /// use Arbitrary::bigfloat::bigfloat::BigFloat;
    /// let a = BigFloat::new(true,vec![1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9],0);
    /// let b = BigFloat::new(true,vec![8,7,6,5,4,3,2,1,9,8,7,6,5,4,3,2,1,9,8,7,6,5,4,3,2,1],0);
    /// let c = BigFloat::new(true,vec![1,0,8,2,1,5,2,1,1,4,5,3,8,9,4,2,1,3,7,9,9,5,7,3,2,1,4,8,1,7,7,1,0,6,9,3,4,7,2,0,3,1,6,9,1,1,2,6,3,5,2,6,9],53);
    /// assert_eq!(BigFloat::k_mul(a,b),c);
    /// ```
    pub fn k_mul(a: BigFloat, b: BigFloat)  -> BigFloat {
        //Check if it's worth it to do karatsuba.
        if(a.vals.len() >20 && b.vals.len() >20){
            //Modified to choose the split point as the smaller of the length of a and b /2
            let mut new_vals: Vec<i8> = vec![0;a.vals.len()+b.vals.len()];
            let mut a_prime = a.vals.clone();
            let mut b_prime = b.vals.clone(); // Let's not care about the memory issues now.
            let m = std::cmp::min(a.vals.len(),b.vals.len());
            let split_index = m/2;
            println!("si: {}",split_index);
            let a_2 = a_prime.split_off(a.vals.len() - split_index);
            let b_2 = b_prime.split_off(b.vals.len() - split_index);
            let a_2_len = a_2.len();
            let b_2_len = b_2.len();
            let bf_a2 = BigFloat::new(true,a_2,a_2_len as i64);
            let bf_b2 = BigFloat::new( true,b_2,b_2_len as i64);
            let a_prime_len = a_prime.len();
            let b_prime_len = b_prime.len();
            let bf_a_prime = BigFloat::new(true,a_prime,a_prime_len as i64);
            let bf_b_prime = BigFloat::new(true,b_prime,b_prime_len as i64);
            
            println!("a_p: {:?}",bf_a_prime);
            println!("a2: {:?}",bf_a2);
            println!("b_p: {:?}",bf_b_prime);
            println!("b2: {:?}",bf_b2);
            //There is definitely a more elegant way to do this. 
            let z0 = BigFloat::k_mul(bf_a2.clone(),bf_b2.clone());
            let z2 = BigFloat::k_mul(bf_a_prime.clone(),bf_b_prime.clone()); 
            let z1 = BigFloat::k_mul(BigFloat::add(bf_a_prime,bf_a2),BigFloat::add(bf_b_prime,bf_b2));
            println!("z0: {:?}",z0);
            println!("z1: {:?}",z1);
            println!("z2: {:?}",z2);
            return BigFloat::add(BigFloat::add(z2.lshift((split_index) as i64*2),BigFloat::sub(BigFloat::sub(z1,z2),z0.clone()).lshift((split_index) as i64)), z0);
        }   
        //It wasn't worth it.
        return BigFloat::quad_mult(a, b);
    }
   pub fn mul(a: BigFloat, b: BigFloat) -> BigFloat {
        let a_decimal = a.vals.len() - a.decimal as usize;
        let a_sign = a.sign;
        let b_decimal = a.vals.len() - b.decimal as usize;
        let b_sign = b.sign;
        let mut c = BigFloat::k_mul(a,b);
        c.sign = (a_sign&&b_sign)||(!a_sign&&!b_sign);
        c.decimal = (c.vals.len() - a_decimal - b_decimal ) as i64;
        return c;
    }
}