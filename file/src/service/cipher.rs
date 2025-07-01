use std::vec;

use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub trait Cipher: Send + Sync {
    fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>);
    fn decrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8>;
}

pub struct Aes256GcmCipher;

impl Cipher for Aes256GcmCipher {
    fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        println!("Encrypting with AES256GCM");
        (data.to_vec(), b"key".to_vec(), vec![])
    }

    fn decrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        println!("Decrypting with AES256GCM");
        data.to_vec()
    }
}

pub struct Aes128GcmCipher;

impl Cipher for Aes128GcmCipher {
    fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        println!("Encrypting with AES256GCM");
        (data.to_vec(), b"key".to_vec(), vec![])
    }

    fn decrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        println!("Decrypting with AES256GCM");
        data.to_vec()
    }
}

pub struct ChaChaCipher;

impl Cipher for ChaChaCipher {
    fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher_text = cipher.encrypt(&nonce, data).unwrap();

        (cipher_text, key.to_vec(), nonce.to_vec())
    }

    fn decrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let key = Key::from_slice(key);
        let nonce = Nonce::from_slice(nonce);
        let cipher = ChaCha20Poly1305::new(key);
        cipher.decrypt(nonce, data).unwrap_or_else(|_| vec![])
    }
}

pub fn get_cipher(method: &str) -> Box<dyn Cipher> {
    match method {
        "aes256gcm" => Box::new(Aes256GcmCipher),
        "aes128gcm" => Box::new(Aes128GcmCipher),
        "chacha20poly1305" => Box::new(ChaChaCipher),
        _ => panic!("Unsupported encryption method"),
    }
}
