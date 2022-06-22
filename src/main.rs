extern crate clap;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use clap::Parser;
use colored::Colorize;

/* For clap */
#[derive(Parser)]
#[clap(
    name = "bincmp",
    author = "Akihiro Saiki <misly.lx00@gmail.com>",
    version = "0.0.1",
    about = "Binary compare app."
)]
struct AppArgs {
    #[clap(short = 'b', long = "base")]
    base: String,
    #[clap(short = 't', long = "target")]
    target: String,
    #[clap(short = 'o', long = "offset", default_value = "0")]
    offset: u64,
    #[clap(short, long)]
    recursive: bool
}

/* Read file to buffer */
fn file_to_buffer(filename: &str, buf: &mut Vec<u8>) -> std::io::Result<()> {
    let file = File::open(filename).expect("File not found.");
    let mut reader = BufReader::new(file);
    reader.read_to_end(buf)?;
    Ok(())
}

/* Compare buffer */
fn cmp_buffer(buf1: &Vec<u8>, buf2: &Vec<u8>, offset: u64, len: u64) -> Option<u64> {
    for n in 0..len {
        let byte1 = buf1[(offset + n) as usize];
        let byte2 = buf2[(offset + n) as usize];
        if byte1 != byte2 {
            return Some(offset + n)
        }
    }
    None
}

/* main */
fn main() {
    /* Parse commandline arguments */
    let args: AppArgs = AppArgs::parse();

    println!("bincmp v0.0.1 by Nanamiiiii\n");

    let base_name = args.base;
    let target_name = args.target;
    
    /* Read binary files to buffer */
    let mut base_buf: Vec<u8> = Vec::new();
    let mut target_buf: Vec<u8> = Vec::new();
    file_to_buffer(base_name.as_str(), &mut base_buf).unwrap();
    file_to_buffer(target_name.as_str(), &mut target_buf).unwrap();

    /* Prepare params */
    let offset: u64 = args.offset;
    let base_len = base_buf.len();
    let target_len = target_buf.len();

    println!("{}:\t{}\t{} byte(s)", "Base binary".bold(), base_name.as_str(), base_len);
    println!("{}:\t{}\t{} byte(s)", "Target binary".bold(), target_name.as_str(), target_len);
    println!("{}:\t{} byte(s)\n", "Offset".bold(), offset);

    let mut same_size: bool = true;
    if base_len != target_len { same_size = false }

    /* Filesize error */
    if offset >= base_len as u64 {
        println!("{}\tinvalid offset value.", "Error".red().bold());
        println!("\tThe value over the base binary size: {} byte(s)", base_len.to_string().bold());
        std::process::exit(1);
    }
    if offset >= target_len as u64 {
        println!("{}\tinvalid offset value.", "Error".red().bold());
        println!("\tThe value over the target binary size: {} byte(s)", target_len.to_string().bold());
        std::process::exit(1);
    }

    /* set size to traverse */
    let mut len = if base_len < target_len { base_len as u64 } else { target_len as u64 };
    len -= offset;

    /* compare and result */
    match cmp_buffer(&base_buf, &target_buf, offset, len) {
        Some(n) => {
            let base_byte = base_buf[n as usize];
            let target_byte = target_buf[n as usize];
            println!("{}", "The binaries unmatched!".magenta().bold());
            println!("Different byte at {}", format!("0x{:X}",n).bold());
            println!("Base byte: {}, Target byte: {}", format!("0x{:X}", base_byte).bold(), format!("0x{:X}", target_byte).bold());
        },
        None => {
            if same_size {
                println!("{}", "The binaries matched!".green().bold());
            } else {
                println!("{}", "The binaries partially matched!".cyan().bold());
                if base_len < target_len {
                    println!("Backward {} byte(s) of target binary not checked because of size difference.", target_len - base_len);
                } else {
                    println!("Backward {} byte(s) of base binary not checked because of size difference.", base_len - target_len); 
                }
            }
        },
    }
}
