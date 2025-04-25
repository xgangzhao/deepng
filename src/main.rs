mod chunk_type;
mod chunk;
mod png;
mod chunk_type_error;
mod chunk_error;
mod png_error;
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, Command};
use std::{fs, io::Write, str::FromStr};
use std::path::PathBuf;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input = arg!(-f --file "The PNG file to encode into").required(true)
                                            .value_name("FILE")
                                            .action(ArgAction::Set)
                                            .value_parser(value_parser!(PathBuf));
    let ck_type  = arg!(-t --type "The chunk type to use for encoding").required(true)
                                            .value_name("TYPE")
                                            .action(ArgAction::Set)
                                            .value_parser(value_parser!(String));
    let msg_file = Arg::new("msg_file").short('M').long("message-file")
                                           .help("The file contains content to be encoded into the PNG file")
                                           .value_name("FILE")
                                           .action(ArgAction::Set)
                                           .value_parser(value_parser!(PathBuf));
    let message = arg!(-m --message "The message to encode into the PNG file")
                                            .value_name("TEXT")
                                            .action(ArgAction::Set)
                                            .value_parser(value_parser!(String));
    let output = arg!(-o --out "The output file to write the encoded PNG to")
                                            .value_name("FILE")
                                            .action(ArgAction::Set)
                                            .value_parser(value_parser!(PathBuf));
    let encode = Command::new("encode")
                         .arg(input.clone())
                         .arg(ck_type.clone())
                         .arg(msg_file.clone())
                         .arg(message.clone())
                         .arg(output.clone())
                         .group(ArgGroup::new("messages").args(["msg_file", "message"])
                                                             .multiple(false)
                                                             .required(true));
    let decode = Command::new("decode")
                         .arg(input)
                         .arg(ck_type)
                         .arg(msg_file)
                         .arg(output);

    let matches = Command::new("deepng")
                        .subcommand(encode)
                        .subcommand(decode)
                        .get_matches();

    // process argu
    match matches.subcommand() {
        Some(("encode", encode)) => {
            let in_file = encode.get_one::<PathBuf>("file").unwrap();
            let ck_type = encode.get_one::<String>("type").unwrap();
            // let messages = encode.get_one::<PathBuf>("messages");
            let mut data = "".to_string();
            if let Some(msg_file) = encode.get_one::<PathBuf>("msg_file") {
                data = fs::read_to_string(msg_file)?;
            } else if let Some(message) = encode.get_one::<String>("message") {
                data = message.to_string();
            }
            let out_file = encode.get_one::<PathBuf>("out");
            let content = fs::read(&in_file)?;
            let mut png = png::Png::try_from(content.as_ref())?;
            let ck_type = chunk_type::ChunkType::from_str(ck_type)?;
            if ck_type.is_valid_type() == false {
                return Err(Box::from(png_error::PngError::InvalidEncodeType));
            }
            let ck = chunk::Chunk::new(ck_type, data.as_bytes().to_vec());

            png.append_chunk(ck);
            match out_file {
                Some(out_file) => {
                    let mut ofile = fs::File::create(&out_file)?;
                    ofile.write_all(&png.as_bytes())?;
                }
                None => {
                    let mut ofile = fs::File::create(&in_file)?;
                    ofile.write_all(&png.as_bytes())?;
                }
            }
        }

        Some(("decode", decode)) => {
            let in_file = decode.get_one::<PathBuf>("file").unwrap();
            let ck_type = decode.get_one::<String>("type").unwrap();
            let msg_file = decode.get_one::<PathBuf>("msg_file");
            let out_file = decode.get_one::<PathBuf>("out");
            let content = fs::read(in_file)?;
            let mut png = png::Png::try_from(content.as_ref())?;
            let ck = png.remove_last_chunk(ck_type);
            match ck {
                Ok(ck) => {
                    match msg_file {
                        Some(msg_file) => {
                            let mut ofile = fs::File::create(&msg_file)?;
                            ofile.write_all(ck.data())?;
                        }
                        None => {
                            println!("{}", ck.data_as_string()?);
                        }
                    }

                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
            match out_file {
                Some(out_file) => {
                    let mut ofile = fs::File::create(&out_file)?;
                    ofile.write_all(&png.as_bytes())?;
                }
                None => {
                    let mut ofile = fs::File::create(&in_file)?;
                    ofile.write_all(&png.as_bytes())?;
                }
            }
        }
        _ => unreachable!("Unrecognized subcommand")
    }

    return Ok(());
}

