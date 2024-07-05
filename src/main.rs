use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lopdf::{xref::XrefType, Object};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Textify {
        input_path: PathBuf,
        output_path: PathBuf,
    },
    Splat {
        input_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Textify {
            input_path,
            output_path,
        } => {
            let mut doc = lopdf::Document::load(input_path).unwrap();

            doc.reference_table.cross_reference_type = XrefType::CrossReferenceTable;

            for ((_id, _gen), object) in &mut doc.objects {
                if let Object::Stream(ref mut stream) = object {
                    stream.decompress();

                    let content = std::mem::replace(&mut stream.content, Vec::new());

                    // if !stream.content.is_ascii() {
                    if let Ok(string) = String::from_utf8(content) {
                        stream.content = string.into_bytes();
                    } else {
                        stream.dict.set("Filter", "ASCIIHexDecode");
                        stream.set_content(hex::encode(&stream.content).into_bytes());
                    }
                }
            }

            doc.save(output_path).unwrap();
        }
        Command::Splat { input_path } => {
            let doc = lopdf::Document::load(&input_path).unwrap();

            let dir_path = input_path.with_extension("");

            std::fs::create_dir(&dir_path).unwrap();

            for ((id, _gen), object) in doc.objects {
                if let Object::Stream(stream) = object {
                    let mut file =
                        std::fs::File::create(dir_path.join(format!("{}.dict", id))).unwrap();
                    write!(file, "{:#?}", stream.dict).unwrap();

                    if let Ok(c) = stream.decompressed_content() {
                        let mut file =
                            std::fs::File::create(dir_path.join(format!("{}.cstream", id)))
                                .unwrap();
                        file.write_all(&c).unwrap();
                    } else {
                        let mut file =
                            std::fs::File::create(dir_path.join(format!("{}.stream", id))).unwrap();
                        file.write_all(&stream.content).unwrap();
                    }
                } else {
                    let mut file = std::fs::File::create(dir_path.join(format!("{}", id))).unwrap();
                    write!(file, "{:#?}", object).unwrap();
                }
            }

            let mut file = std::fs::File::create(dir_path.join("trailer")).unwrap();
            write!(file, "{:?}", doc.trailer).unwrap();
        }
    }
}
