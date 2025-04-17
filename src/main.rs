mod chunk_type;
mod chunk;
mod png;
mod chunk_type_error;
mod chunk_error;
mod png_error;
use clap::{arg, Arg, Command, ArgAction, command, value_parser};
use std::{fs, io::Write, process::Output, str::FromStr};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let png_file = arg!(-f --file "The PNG file to encode into").required(true)
                                            .num_args(1)
                                            .value_name("FILE")
                                            .hide_default_value(true)
                                            .value_parser(value_parser!(String));
    let ck_type  = arg!(-t --type "The chunk type to use for encoding").required(true)
                                            .num_args(1)
                                            .value_name("TYPE")
                                            .hide_default_value(true)
                                            .value_parser(value_parser!(String));
    let message = arg!(-m --message "The message to encode into the PNG file").required(true)
                                            .num_args(1)
                                            .value_name("MESSAGE")
                                            .hide_default_value(true)
                                            .value_parser(value_parser!(String));
    let output = arg!(-o --out "The output file to write the encoded PNG to")
                                            .num_args(1)
                                            .value_name("FILE")
                                            .hide_default_value(true)
                                            .value_parser(value_parser!(String));
    let encode = Command::new("encode")
                         .arg(png_file.clone())
                         .arg(ck_type.clone())
                         .arg(message.clone())
                         .arg(output.clone());
    let decode = Command::new("decode")
                         .arg(png_file)
                         .arg(ck_type)
                         .arg(output);

    let matches = Command::new("deepng")
                        .subcommand(encode)
                        .subcommand(decode)
                        .get_matches();

    // process argu
    match matches.subcommand() {
        Some(("encode", encode)) => {
            let in_file = encode.get_one::<String>("png_file").unwrap();
            let ck_type = encode.get_one::<String>("ck_type").unwrap();
            let message = encode.get_one::<String>("message").unwrap();
            let out_file = encode.get_one::<String>("output").unwrap_or(&String::new());
            let content = fs::read(in_file)?;
            let mut png = png::Png::try_from(content.as_ref())?;
            let ck_type = chunk_type::ChunkType::from_str(ck_type)?;
            let ck = chunk::Chunk::new(ck_type, message.as_bytes().to_vec());
            png.append_chunk(ck);
            if !out_file.is_empty() {
                let mut ofile = fs::File::create(out_file)?;
                ofile.write_all(&png.as_bytes())?;
            }
        }
    }

    return Ok(());
}

