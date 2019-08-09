fn main() {
    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
        let s0 = s0.parse::<i32>()?;
        let s1 = s1.parse::<i32>()?;
        Ok(s0 + s1)
    }

    assert_eq!(add0("3", "127"), Ok(130));
    assert!(add0("3", "abc").is_err());
}
