

mod tests {
    use crate::bigfloat::bigfloat::BigFloat;
    
    #[test]
    fn test_str_to_bf(){
        let string = "0.123456789";
        assert_eq!(BigFloat::from_str(string),BigFloat::new(true,vec![0,1,2,3,4,5,6,7,8,9],1));
        let string2 = "-0.123456789";
        assert_eq!(BigFloat::from_str(string2),BigFloat::new(false,vec![0,1,2,3,4,5,6,7,8,9],1));
    }
    #[test]
    fn test_add(){
        let a = BigFloat::new(false,vec![0,1,2,3,4,5,6,7,8,9],1);
        let b =  BigFloat::new(false,vec![0,1,2,3,4,5,6,7,8,9],1);
        assert_eq!(BigFloat::add(a,b),BigFloat::new(false,vec![0,2,4,6,9,1,3,5,7,8],1));
        
    }
    #[test]
    fn test_add_shifting(){
        let c = BigFloat::from_str("1231");
        let d = BigFloat::from_str("129");
        let e: BigFloat = BigFloat::add(d, c);
        assert_eq!(e,BigFloat::new(true,vec![1,3,6,0],4));
    }
    #[test]
    fn test_sub_shifting(){
        let c = BigFloat::from_str("1231");
        let d = BigFloat::from_str("129");
        let e: BigFloat = BigFloat::sub(d, c);
        assert_eq!(e,BigFloat::new(false,vec![1,1,0,2],4));
    }
    #[test]
    fn test_add_carry(){
        let a = BigFloat::new(false,vec![0,1,2,3,4,5,6,7,8,9],1);
        let b =  BigFloat::new(false,vec![0,9,2,3,4,5,6,7,8,9],1);
        assert_eq!(BigFloat::add(a,b),BigFloat::new(false,vec![1,0,4,6,9,1,3,5,7,8],1));
    }
    #[test]
    fn test_sub_overflow(){
        let a = BigFloat::new(false,vec![0,1,2,3,4,5,6,7,8,9],1);
        let b =  BigFloat::new(false,vec![0,9,2,3,4,5,6,7,8,9],1);
        assert_eq!(BigFloat::sub(a,b),BigFloat::new(true,vec![0,8,0,0,0,0,0,0,0,0],1));
    }
    #[test]
    fn test_quad_mul(){
        let a = BigFloat::new(true,vec![1,2,3,4,5,6,7,8],5);
        let b = BigFloat::new(true,vec![7,6,5,4,3,2,1,8],5);
        let c = BigFloat::quad_mult(a, b);
        assert_eq!(c,BigFloat::new(true,vec![9,4,4,9,7,7,9,2,2,5,1,1,8,0,4],9));
        println!("{}",c.to_string());
    }
}