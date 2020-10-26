pub fn calculate(x:u32,y:u32)->u32{
    x+y
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_calculate() {
        assert_eq!(4,calculate(2, 2))
        
    }
}