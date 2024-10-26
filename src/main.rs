

// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate, process_text_sign, process_text_verify, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat,process_http_serve};
use zxcvbn::zxcvbn;
use std::fs;






#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts=Opts::parse();

    match opts.cmd{
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {

                output.clone()
            }else{
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input, output,opts.format)?
        }
        SubCommand::GenPass(opts)=>{
            let password = process_genpass(
                opts.length, 
                opts.uppercase, 
                opts.lowercase, 
                opts.number, 
                opts.symbol)?;
            // println!("{}",password);
            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength:{}",estimate.score());
        }

        SubCommand::Base64(subcmd)=> match subcmd{
            Base64SubCommand::Encode(opts)=>{
                let encoded =  process_encode(&opts.input,opts.format)?;
                println!("{}",encoded);
            }
            Base64SubCommand::Decode(opts)=>{
                let decoded = process_decode(&opts.input,opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}",decoded);
            }
        }
        SubCommand::Text(subcmd) => match  subcmd{ 
            rcli::TextSubCommand::Sign(opts)=>{
                match opts.format{
                    TextSignFormat::Blake3=>{
                        let sig = process_text_sign(&opts.input,&opts.key,opts.format)?;
                        println!("{}",sig);
                    }
                    TextSignFormat::Ed25519 => {
                        let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                        println!("{}",sig);
                    }
                }
            }
            rcli::TextSubCommand::Verify(opts)=>{
                let verified = process_text_verify( &opts.input,&opts.key, opts.format, &opts.sig)?;
                println!("{}",verified)
            }
            rcli::TextSubCommand::Generate(opts)=>{
                let key = process_text_generate(opts.format)?;
                match opts.format{
                    rcli::TextSignFormat::Blake3=>{
                        let name = opts.output.join("blake3.txt");
                        fs::write(name,&key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519=>{
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"),&key[0])?;
                        fs::write(name.join("ed25519.pk"),&key[1])?;


                    }
                }
            }
        }
        SubCommand::Http(cmd)  => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir,opts.port).await?;
            }
        }  

    }
    Ok(())
}

