use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;
use crate::opts::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(opts: &GenPassOpts) -> anyhow::Result<()>{
    let mut rng = rand::thread_rng();
    // let mut passwd = String::new();
    let mut passwd = Vec::new();
    let mut chars = Vec::new();
    
    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        passwd.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty!"));
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        passwd.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty!"));
    }
    if opts.numbers{
        chars.extend_from_slice(NUMBER);
        passwd.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty!"));
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        passwd.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty!"));
    }
    
    for _ in 0..(opts.length - passwd.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        let c = chars.choose(&mut rng).expect("chars won't be enpty in this context");
        passwd.push(*c);
    }
    passwd.shuffle(&mut rng);
    
    let password = String::from_utf8(passwd)?;
    println!("{}",password);
    
    let estimate = zxcvbn(&password,&[]);
    eprintln!("{}",estimate.score());
    
    Ok(())
}