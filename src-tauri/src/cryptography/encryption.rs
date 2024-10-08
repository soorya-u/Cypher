use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes128Gcm, Key, KeyInit, Nonce,
};
use typenum;

pub struct Encryption {
    unique_key: GenericArray<u8, typenum::U16>,
    nonce: GenericArray<u8, typenum::U12>,
}

impl Encryption {
    pub fn generate_unique_key() -> String {
        let key = Aes128Gcm::generate_key(&mut OsRng).as_slice().to_vec();
        String::from_utf8(key).unwrap()
    }

    pub fn generate_nonce() -> String {
        let nonce = Aes128Gcm::generate_nonce(&mut OsRng).as_slice().to_vec();
        String::from_utf8(nonce).unwrap()
    }

    pub fn new(key: &str, nonce: &str) -> Self {
        let unique_key = Key::<Aes128Gcm>::from_slice(key.as_bytes()).to_owned();
        let nonce = Nonce::from_slice(nonce.as_bytes()).to_owned();

        return Encryption { nonce, unique_key };
    }

    pub fn encrypt_data(&self, data: &str) -> Result<String, String> {
        let data = data.as_bytes();
        let cipher = Aes128Gcm::new(&self.unique_key);
        match cipher.encrypt(&self.nonce, data) {
            Ok(c) => Ok(String::from_utf8(c).unwrap()),
            Err(e) => Err(format!("Error in encryption: {:?}", e)),
        }
    }

    pub fn decrypt_data(&self, data: &str) -> Result<String, String> {
        let data = data.as_bytes();
        let cipher = Aes128Gcm::new(&self.unique_key);
        match cipher.decrypt(&self.nonce, data) {
            Ok(c) => Ok(String::from_utf8(c).unwrap()),
            Err(e) => Err(format!("Error in encryption: {:?}", e)),
        }
    }
}
