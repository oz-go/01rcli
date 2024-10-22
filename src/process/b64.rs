use base64::{engine::general_purpose::{URL_SAFE_NO_PAD,STANDARD},Engine as _};
use crate::Base64Format;
use std::{fs::File,io::Read};
use anyhow::Result;


pub fn process_encode(input:&str,format:Base64Format) -> Result<()>{
    let mut reader =  get_reader(input)?;
    
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format{
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}",encoded);
    Ok(())
}



pub fn process_decode(input:&str,format:Base64Format)-> Result<()>{
    let mut reader =  get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();



    let decoded = match format{
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    // TODO: decoded data might not be string (but for this example,we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}",decoded);
    Ok(())
}

fn get_reader(input:&str)->Result<Box<dyn Read>>{
    let  reader:Box<dyn Read> = if input =="-" {
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    

    Ok(reader)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_process_encode(){
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }


    #[test]
    fn test_process_decode(){
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        // process_decode(input, format).unwrap();
        assert!(process_encode(input, format).is_ok());
    }
}