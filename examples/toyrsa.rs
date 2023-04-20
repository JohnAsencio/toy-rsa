fn main()
{
    // Private Key: p = 0xed23e6cd q = 0xf050a04d
    let public_key: u128 = /* p * q = */0xde9c5816141c8ba9;
    // Message: 0x12345f
    let encrypted: i64 = 0x6418280e0c4d7675;
    let decrypted: i64 = 0x12345f;
    use toy_rsa::*;
    let key = 36;//= 0xde9c5816141c8ba9;
    let msg = 100;//= 0x12345f;
    println!("{}", msg);
    let p: u32 = 2;//0xed23e6cd;
    let q: u32 = 18;//0xf050a04d;
    let z: u32 = p * q;
    println!("p: {} q: {} z: {}", p ,q, z);
    let emsg = encrypt(key, msg);
    println!("encrypted message {}", emsg);
    println!("this is what its  {}", encrypted);
    let dmsg: u64 = decrypt((p,q), emsg).into();
    println!("decrypted message {}", dmsg);
    println!("this is what its  {}", decrypted);
}
