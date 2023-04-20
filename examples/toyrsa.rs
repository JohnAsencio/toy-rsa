fn main()
{
    // Private Key: p = 0xed23e6cd q = 0xf050a04d
    let public_key: u128 = /* p * q = */0xde9c5816141c8ba9;
    // Message: 0x12345f
    let encrypted: i64 = 0x6418280e0c4d7675;
    let decrypted: i64 = 0x12345f;
    use toy_rsa::*;
    let key = 0xde9c5816141c8ba9;
    let msg = 0x12345f;
    println!("{}", msg);
    let p: u32 = 0xed23e6cd;
    let q: u32 = 0xf050a04d;
    let z = (p as u128) * (q as u128);
    println!("p: {} q: {} z: {}, key: {}", p ,q, z, public_key);
    let emsg = encrypt(key, msg);
    println!("encrypted message {}", emsg);
    println!("this is what its  {}", encrypted);
    let dmsg = decrypt((p,q), emsg);
    println!("decrypted message {}", dmsg);
    println!("this is what its  {}", decrypted);
}
