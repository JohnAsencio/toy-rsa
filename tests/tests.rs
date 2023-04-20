//use super::*;

#[cfg(test)]
mod tests {
    use toy_rsa::*;

    #[test]
    fn static_tests() {
        let msg = 54;
        let key = 36;
        let a: u32 = 2;
        let b: u32 = 18;
        let emsg = encrypt(key, msg);
        assert_eq!(54, decrypt((a,b), emsg))
    }
}
