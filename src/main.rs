
use Arbitrary::bigfloat::bigfloat::BigFloat;


use std::time::Instant;



fn main() {

    let a = BigFloat::new(true,vec![75, 88, 55, 3, 60, 25, 67, 54, 18, 32, 79, 14, 80, 73, 52, 93, 70, 72, 90, 71, 90, 17, 15, 4, 74, 20, 0, 48, 89, 89, 22, 25, 54, 25, 94, 86, 40, 82, 84, 56, 96],2);
    let b = BigFloat::new(true,vec![75, 88, 55, 3, 60, 25, 67, 54, 18, 32, 79, 14, 80, 73, 52, 93, 70, 72, 90, 71, 90, 17, 15, 4, 74, 20, 0, 48, 89, 89, 22, 25, 54, 25, 94, 86, 40, 82, 84, 56, 96],2);
    let t = Instant::now();
    let c = BigFloat::quad_mult(&a, &b);
    let t2 = t.elapsed();
    println!("{:?}",t2); 
    println!("{:?}",c)
    //println!("{:?}",c);
    //println!("{}",e.to_string());
    //let pb = ProgressBar::new(6972593);
    //pb.set_style(
    //    ProgressStyle::with_template(
    //        "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    //    )
    //    .unwrap(),
    //);
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
    //let mut e = BigFloat::karatsuba(  c,  d);
    //println!("Final: {}",e.to_string())
    //println!("Elapsed: {:?}",i1.elapsed());
    //println!("{}",e.to_string());
    
}
