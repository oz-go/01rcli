use anyhow::{Ok, Result};
use rand::rngs::OsRng;
use std::io::Read;
use std::path::Path;
use std::fs;
use crate::{TextSignFormat,get_reader};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD,Engine as _};
use ed25519_dalek::{Signer,SigningKey,VerifyingKey,Signature,Verifier};

use super::process_genpass;

pub trait TextSigner {
    // signer could sign any input data
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    // verifier could verify any input data
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path:impl AsRef<Path>)-> Result<Self>
    where 
        Self:Sized;
}

pub trait KeyGenerator{
    fn generate()->Result<Vec<Vec<u8>>>;
}

pub struct Blake3{
    key:[u8;32],
}

pub struct Ed25519Signer{
    key:SigningKey,
}
pub struct Ed25519Verifier {
    key: VerifyingKey,
}




pub fn process_text_sign(input:&str,key:&str,format:TextSignFormat)->Result<String>{
    let mut reader = get_reader(input)?;
    let signed = match format{
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 =>{
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    // println!("{}",signed); 
    Ok(signed)
}

pub fn process_text_verify(input:&str,key:&str,format:TextSignFormat,sig:&str)->Result<bool>{
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3=>{
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };
    // println!("{}",verified);
    Ok(verified)
}


pub fn process_text_generate(format:TextSignFormat)->Result<Vec<Vec<u8>>>{
    match format{
        TextSignFormat::Blake3=> Blake3::generate(),
        TextSignFormat::Ed25519=> Ed25519Signer::generate(),
    }
}

impl TextSigner for Blake3{
    fn sign(&self,reader:&mut dyn Read)-> Result<Vec<u8>>{
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key,&buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3{
    fn verify(&self,reader:&mut dyn Read,sig:&[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key,&buf);
        let hash = hash.as_bytes();
        Ok(hash==sig)
    }
} 

impl KeyGenerator for Blake3{
    fn generate()->Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}


impl KeyGenerator for Ed25519Signer{
    fn generate()->Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk,pk])
    }
}


impl TextSigner for Ed25519Signer{
    fn sign(&self,reader:&mut dyn Read) -> Result<Vec<u8>>{
        let mut buf=Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier{
    fn verify(&self,reader:&mut dyn Read,sig:&[u8])->Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}


impl KeyLoader for Blake3{
    fn load(path:impl AsRef<Path>)-> Result<Self>{
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
        
    
}

impl Blake3{
    pub fn new(key:[u8; 32])->Self{
        Self { key }
    }

    
    pub fn try_new(key:&[u8])-> Result<Self>{
        let key = &key[..32];
        let key = key.try_into()?;
        let singer = Blake3::new(key);
        Ok(singer)
    }
}

impl KeyLoader for Ed25519Signer{
    fn load(path:impl AsRef<Path>)-> Result<Self>{
        let key = fs::read(path)?;
        Self::try_new(&key)
        
    }
}

impl KeyLoader for Ed25519Verifier{
    fn load(path:impl AsRef<Path>)-> Result<Self>{
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}



impl Ed25519Signer{
    pub fn new(key:SigningKey) -> Self{
        Self{key}
    }

    pub  fn try_new(key:&[u8]) -> Result<Self>{
        let key = SigningKey::from_bytes(key.try_into()?);
        let singer = Ed25519Signer::new(key);
        Ok(singer)
    }

    
}


impl Ed25519Verifier {
    pub fn new(key:VerifyingKey)->Self{
        Self{key}
    }

    // pub fn try_new(key:&[u8]) -> Result<Self>{
    //     let key = VerifyingKey::from_bytes(key.try_into()?)?;
    //     let verifier = Ed25519Verifier::new(key);
    //     Ok(verifier)
    // }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    use crate::process::text::TextVerifier;

    use super::Blake3;

    #[test]
    fn test_blake3_sign_verify()->Result<()>{
        let blake3 = Blake3::load("fixtures/blake3.txt")?;


        let data = b"hello!";
        let sig = blake3.sign(&mut &data[..]).unwrap();
        assert!(blake3.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify()->Result<()>{
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}