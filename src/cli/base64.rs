
use clap::Parser;

#[derive(Debug,Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts)
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts{
    
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts{
    
}




// -------------------------------------------- CSV -----------------------------------------------
