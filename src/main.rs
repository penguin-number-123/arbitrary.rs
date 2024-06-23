mod arbitrary;
use crate::arbitrary::arbitrary::BigFloat;
use std::time::Instant;
fn main() {
    let mut a = BigFloat::new(false,vec![1,0,0,0,0,0,0,0],3);
    let mut b = BigFloat::new(false, vec![1,0,0,0,0,0],3);
    let mut c = BigFloat::from_str("123");
    let mut d = BigFloat::from_str("456");
    println!("{}",BigFloat::greater_than(&c, &d));
    println!("{}",d.to_string());
    println!("{}",BigFloat::sub_mut(&mut a,&mut b).to_string());
    //let i1 = Instant::now();
    let e = BigFloat::sub_mut(&mut c, &mut d);
    //println!("Elapsed: {:?}",i1.elapsed()/1000);
    println!("{}",e.to_string());
    BigFloat::mul_mut(&mut c, &mut d);
}
