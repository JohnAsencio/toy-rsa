//use super::*;

#[cfg(test)]
mod tests {
    use toy_rsa::*;

    #[test]
    fn static_tests() {
        let key1 = 0xde9c5816141c8ba9;
        let msg1 = 0x12345f;
        let p: u32 = 0xed23e6cd;
        let q: u32 = 0xf050a04d;
        let emsg1 = encrypt(key1, msg1);
        assert_eq!(msg1, decrypt((p,q), emsg1));


    }

    #[test]
    fn random_tests() {

        let msgs: [u32; 10] = [10, 21111111, 312313, 4138917271, 5218638, 6129638, 70298, 82173, 91927, 10000];

        let mut i = 0;
        while i < msgs.len().try_into().unwrap()
        {
            let pq = genkey();
            let key = (pq.0 as u128) * (pq.1 as u128);
            let emsg = encrypt(key.try_into().unwrap(), msgs[i as usize]);
            assert_eq!(msgs[i as usize], decrypt(pq, emsg));
            i+=1;

        }

        let msg = 5400;
        let pq = genkey();
        let key = (pq.0 as u128) * (pq.1 as u128);
        let emsg = encrypt(key.try_into().unwrap(), msg);
        assert_eq!(msg, decrypt(pq, emsg));



    }

}
