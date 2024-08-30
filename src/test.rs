
mod tests {
    
    use crate::bigfloat::bigfloat::BigFloat;


    #[test]
    fn test_str_to_BF(){
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

}