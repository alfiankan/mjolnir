use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;


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
    
    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";

    let data = "name:joko;diognose:[tbc,typhus];";

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    println!("Encrypted: {:?}", hex::encode(buf.clone()));
    println!("Encrypted: {:?}", buf);

    assert_eq!(4, 4);
}



#[test]
fn decrypt_test() {
    let passphrase = "rust_by_example";

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

    let data = hex::decode("a014994ef5e2fd0cd6c1d9a128fbc767c9e30ca5c6f82ca19b0211cf05eb4519a83f0134cf35d26a11e5cfb33a4d5c567011ea675a33da816d4104122ccef2807cc0eb3a4231ce52cccdd4eafda799cd55039df8dc57912388cee3083debaffabd16a7ee7b367602746b4334ebf466a6db030366a78b61481f3ee0e827373f7a");
    //println!("{:?}", data);

    //Decrypt with private key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(&data.unwrap(), &mut buf, Padding::PKCS1).unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
    assert_eq!(4, 4);
}

