pub mod suprtext;
use std::io::{self, Read};
use clap::Parser;
use colored::CustomColor;
use suprtext::Suprtext;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = 100)]
    ct: u64,
   
    #[arg(short, long, default_value_t = 100)]
    nt: u64,

    #[arg(short, long, default_value_t = 0)]
    r:u8,
    
    #[arg(short, long, default_value_t = 0)]
    g:u8,

    #[arg(short, long, default_value_t = 0)]
    b:u8,
}

fn main() -> io::Result<()> {
   
  
   let args = Args::parse();
   
    let foreground = CustomColor::new(args.r, args.g, args.b);
    
    let mut data:String = String::new();
    std::io::stdin().read_to_string(&mut data)?;
    let suprtext = Suprtext::init(args.ct, args.nt, data, foreground);

    suprtext.animate();

    Ok(())
   
}
