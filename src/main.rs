mod arbitrary;
mod large;
use crate::arbitrary::arbitrary::BigFloat;
//use std::time::Instant;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::fs;

fn main() {
    let mut a = BigFloat::new(false,vec![1,0,0,0,0,0,0,0],3);
    let mut b = BigFloat::new(false, vec![1,0,0,0,0,0],3);
    let mut c = BigFloat::from_str("1010101010101010101010101010101010");
    let mut d = BigFloat::from_str("1010101010101010101010101010101010");
    
    let mut j = BigFloat::from_str("1");
    let mut k = BigFloat::from_str("2");
    let pb = ProgressBar::new(6972593);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    //println!("{}",BigFloat::karatsuba(&mut k, &mut j).to_string());
    //for i in (0..6972593)//.progress_with(pb)
    //{
    //    j = BigFloat::karatsuba( j,  k.clone());
    //    println!("{}: {}",i,j.to_string());
    //}
    //j = BigFloat::sub_mut(&mut j, &mut BigFloat::from_str("1"));
    //j = BigFloat::int_exp(&mut k, &mut 6972593);
    //fs::write("./M6972593.txt", j.to_string()).expect("Unable to write file");
    //println!("{}",BigFloat::int_exp(&mut k, 44497).to_string());
    //println!("{}",BigFloat::greater_than(&c, &d));
    //println!("{}",d.to_string());
    //println!("{}",BigFloat::sub_mut(&mut a,&mut b).to_string());
    //let i1 = Instant::now();
    let mut e = BigFloat::karatsuba(  c,  d);
    println!("Final: {}",e.to_string())
    //println!("Elapsed: {:?}",i1.elapsed());
    //println!("{}",e.to_string());
    
}
