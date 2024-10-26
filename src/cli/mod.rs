mod csv;
mod genpass;
mod base64;
mod text;
mod http;

use clap::Parser;
use std::path::{Path, PathBuf};

use self::{csv::CsvOpts,genpass::GenPassOptos};

pub use self::{
    csv::OutputFormat,
    base64::{Base64Format,Base64SubCommand},
    text::{TextSignFormat,TextSubCommand},
    http::HttpSubCommand,
};

#[derive(Debug,Parser)]
#[command(name = "rcli", version,author,about,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd:  SubCommand,
} 

#[derive(Debug,Parser)]
pub enum SubCommand{
    #[command(name="csv",about="Show CSV,or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name="genpass",about="Generate a random password")]
    GenPass(GenPassOptos),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),

}

fn verify_file(filename:&str) -> Result<String,&'static str>{
    // if input is "-"
    if filename == "-" || Path::new(filename).exists(){

        Ok(filename.into())
    }else{ 
        Err("File does not exist")
    }}


fn verify_path(path:&str)->Result<PathBuf,&'static str>{
    // if path == "-" || Path::new(path).exists(){
    let p = Path::new(path);
    if p.exists() && p.is_dir(){
        Ok(path.into())
    }else {
        Err("Path does not exist or is not a directory")
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_verify_input_file(){
        assert_eq!(verify_file("-"),Ok("-".into()));
        assert_eq!(verify_file("*"),Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"),Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"),Err("File does not exist"));

    }
}










