mod arbitrary;
use crate::arbitrary::arbitrary::BigFloat;
use std::time::Instant;
fn main() {
    let mut a = BigFloat::new(false,vec![1,0,0,0,0,0,0,0],3);
    let mut b = BigFloat::new(false, vec![1,0,0,0,0,0],3);
    let mut c = BigFloat::from_str("123412312314123");
    let mut d = BigFloat::from_str("120928018312032902098213093142");
    let mut j = BigFloat::quad_mult(&mut c, &mut d);
   
    
    //println!("{}",BigFloat::greater_than(&c, &d));
    //println!("{}",d.to_string());
    //println!("{}",BigFloat::sub_mut(&mut a,&mut b).to_string());
    //let i1 = Instant::now();
    let mut e = BigFloat::karatsuba(&mut c,&mut d);
    println!("{}",e.to_string())
    //println!("Elapsed: {:?}",i1.elapsed());
    //println!("{}",e.to_string());
    
}
