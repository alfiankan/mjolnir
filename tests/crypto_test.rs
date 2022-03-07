use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use regex::internal::Char;


#[test]
fn generate_pair_key() {
    
    let pass_phrase = "tersanjung";
    let rsa = Rsa::generate(2048).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), pass_phrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());

    assert_eq!(4, 4);
}

#[test]
fn hash_test() {
       let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,43371B6CECDB096AC2A362FD33BF4B07

aIs3x9UBN95VJJFsd1ddYxmwAKQdFE5BJwZVYtidV+cZ4Qpmg9tdBLm8AhF5bVGR
FzAVMxTEFQgwT4o2jH2UxRkRmChwNy6aqdGteDIK6yXQK7//GMmxhbvqMmFzwdof
2E7Jkq3BQQEqMFu2CxRUPUFYRIebEIZSDCD3PoJ6p7a77qwm/KCXCbad/DqtOGkJ
wOkPH5AXLIu02MJfs+vcLswXFMlq7aaUrAv5WGt1SpKz9Co6bplSYDG7JE+906Uw
MIg4XDJTJDKCKyDaPkMydw6StvyNuZfIYUNIofulLci7yoNEGvwQHsHCaHr6n4bt
I4iC9CbkEcPbf06HAWGFfsexeLGf9mU0HVsZi83QdMhWMbOREakFU755AMvTeB8w
IMCNn55nzJlSHooKuvJAmbqBBb4+wqgwnoYQEVZmTDZxqT/eR08Zl9d1QeKB+1fw
gjZmY/10kFLnTKlWGIaLIu60ehbXxZeFbW+m1pF9uHEiIkWgkrHNjKfzWh5EyfhY
vXxWuZH92ZP/nioGzVQr00oSEPLwW1RSoAx3jPuu1EILNu7lFL896CsDZpa1Oigf
OMxk0GhMuKs4H6TlHmx5a0TOAcGYWEbnqXi+KUw7pMPFiEs1/2crFI6QfQx8R7dL
/ohKFvksPExsB196RZ1PFyMdryOr/mCqI4nBT+KzPz4zJF2iTMGq3NFQI2MvW/4g
WMwsyQtIJQviFJpYlQpOVBFaeB69oHJMxfauM8OdEU8yomFl3sAVagNxPfiWsGt4
LRsReK2BDT/pnhhZG96qSsNPwQlrwffBleTy9BGSuHHox6A7GKyVAAOMND/TY1ak
-----END RSA PRIVATE KEY-----";



    let data = "name:joko;diognose:[tbc,typhus,diaphra];";

    // Encrypt with public key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), "rust_by_example".as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    println!("Encrypted: {:?}", hex::encode(buf.clone()));
    // println!("Encrypted: {:?}", buf);

    assert_eq!(4, 4);
}



#[test]
fn decrypt_test() {
    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";

    let data = hex::decode("be6ab32885859b2c291e3c5fa7dcdc6bba53f05c92644b530580eb7f4ced51b0d0eb8c8032d9189c31b2f4a948b787338d477619c2152d4d6f9055a55665a1e7174470e74b012965ba2b6f7aacf9bfce0601b0fd618984fbbb34a3bd1c0ca1d205d7f8e1379b31ebab5aa0a2a5c132f8bb9641e65967013561771db5809f71c8");
    //println!("{:?}", data);

    //Decrypt with private key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_decrypt(&data.unwrap(), &mut buf, Padding::PKCS1).unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
    assert_eq!(4, 4);
}

#[test]
fn trim_mql() {
    let mql = "insert to f545 'dfffdf fsf fsf'";

    // let mut mql_char: Vec<Char> = Vec::new();
    // for x in mql.chars() {
    //     mql_char.push(Char::from(x));
    // }

    // char_vector.iter().cloned().collect::<String>();
    println!("{}",  &mql[1..3]);

}
