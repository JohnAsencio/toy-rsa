pub fn add(left: usize, right: usize) -> usize {
    left + right
}
/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

pub fn yuy(p: u32, q: u32) -> u64
{
    lcm((p-1).into(), (q-1).into())
}

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32)
{
    let e: u64 = EXP;
    let mut p = rsa_prime();
    let mut q = rsa_prime();
    while (e < yuy(p, q).into()) && (gcd(e, yuy(p, q).into()) == 1)
    {
        p = rsa_prime();
        q = rsa_prime();
    }
    (p, q)
}
/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64
{
    modexp(msg.into(), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32
{
    let p = key.0;
    let q = key.1;
    let z = (p as u128) * (q as u128);
    println!("p: {} q: {} z: {}", p ,q, z);
    let d = modinverse(EXP, yuy(p, q).into());
    modexp(msg, d, z.try_into().unwrap()).try_into().unwrap()
}

/// functions from the toy_rsa_lib
use toy_rsa_lib::*;


//toy_rsa_lib::pub fn lcm(n: u64, m: u64) -> u64


//toy_rsa_lib::pub fn modexp(x: u64, y: u64, m: u64) -> u64


//toy_rsa_lib::pub fn modinverse(a: u64, m: u64) -> u64


//toy_rsa_lib::pub fn rsa_prime() -> u32


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
