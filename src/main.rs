// This file is part of suprtext
//
// suprcen is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// suprps is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


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
