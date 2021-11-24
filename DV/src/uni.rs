use rsa::{RSAPrivateKey, RSAPublicKey};
use serde::{Serialize, Deserialize};
use crate::signcryption::*;
use crate::otae::{derive_key_pair, encrypt, decrypt};
use crate::onion::{onion_encrypt, onion_decrypt};

#[derive(Serialize, Deserialize)]
pub struct OnionSender {
    sks: [u8;64],
    pkr: RSAPublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct OnionReceiver {
    skr: [u8;32],
    pks: RSAPrivateKey,
}

#[derive(Serialize, Deserialize)]
pub struct OnionMessage {
    s: [u8],    // s designates the new receiver state
    msg: [u8],  //plaintext
}

#[derive(Serialize, Deserialize)]
pub struct OnionCiphertext {
    ct: [[u8];u8], //2d array
}


// uni.Init
pub fn init() -> (&[u8],&[u8]) {
    let sign_key = generate_sign_key();
    let sks = sign_key.0;
    let skr = sign_key.1;
    let cipher_key = generate_cipher_key();
    let pks = cipher_key.0;
    let pkr = cipher_key.1;

    let mut sender = OnionSender{
        sks,
        pkr,
    };
    let mut receiver = OnionReceiver {
        skr,
        pks,
    };
    let s = serde_json::to_string(&sender).expect("unable to encode onion sender").unwrap().as_bytes();
    let r = serde_json::to_string(&receiver).expect("unable to encode onion receiver").unwrap().as_bytes();
    (s, r)
}

pub fn send(s: &[[u8];u8], hk: &[u8], ad: &[u8], msg: Box<[u8]>) -> (&[u8], &[u8]){
    let u = init();   //(sts, str)
    let us = u.0;      // sks, pkr
    let mut ur = u.1;   // skr, pks
    let n = s.len();


    //One-time symmetric encryption
    //let k: &[u8; 16];
    //let ks : &[[u8]; n];

    let plaintext = OnionMessage{
        s: *ur, //s designates the new receiver state
        msg: *msg,
    };

    let pt = serde_json::to_string(&plaintext).unwrap().as_bytes();  //pt' -< (st_r', pt)


    /**onion.Enc**/
    let ct = onion_encrypt(hk, s, ad, pt).unwrap();
    (us, ct.as_slice())
}

pub fn receive(s: &[[u8];u8], hk: &[u8], ad: &[u8], ct: &[u8]) -> (&[u8], &[u8]){
    let n = s.len();
    let k : &[u8;16] = &[];

    let pt = onion_decrypt(hk, s, ad, ct).expect("unable to decrypt the ciphertext");

    //parser the pt and get str' and pt
    //parse pt' = (str', pt)
    //(str', pt)
    (s, pt.as_slice())
}