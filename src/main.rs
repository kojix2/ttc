use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "token-counter", about = "Count tokens in files.")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<std::path::PathBuf>,

    #[structopt(
        short = "e",
        long,
        default_value = "cl100k_base",
        help = "Encoding model (cl100k_base|p50k_base|p50k_edit|r50k_base)"
    )]
    encoding: String,
}

fn get_encoder(encoding: &str) -> tiktoken_rs::CoreBPE {
    match encoding {
        "cl100k_base" => tiktoken_rs::cl100k_base().unwrap(),
        "p50k_base" => tiktoken_rs::p50k_base().unwrap(),
        "p50k_edit" => tiktoken_rs::p50k_edit().unwrap(),
        "r50k_base" | "gpt2" => tiktoken_rs::r50k_base().unwrap(),
        _ => {
            eprintln!("ttc: invalid encoding name");
            process::exit(1);
        }
    }
}

fn count_tokens(content: &str, encoder: &tiktoken_rs::CoreBPE) -> usize {
    encoder.encode_with_special_tokens(content).len()
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let encoder = get_encoder(&opt.encoding);

    if opt.files.is_empty() {
        let tokens = read_from_stdin()?;
        let count = count_tokens(&tokens, &encoder);
        println!("{}", count);
    } else {
        count_tokens_in_files(&opt.files, &encoder);
    }

    Ok(())
}

fn read_from_stdin() -> io::Result<String> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    Ok(content)
}

fn count_tokens_in_files(file_paths: &[PathBuf], encoder: &tiktoken_rs::CoreBPE) { // argument changed
    let mut total = 0;
    let mut file_count = 0;

    for path in file_paths {
        match read_file(&path) {
            Ok(content) => {
                let count = count_tokens(&content, &encoder);
                println!("{:8} {}", count, path.display());
                total += count;
                file_count += 1;
            },
            Err(e) => eprintln!("Failed to read file {}: {}", path.display(), e),
        }
    }
   
    if file_count > 1 {
        println!("{:8} total", total);
    }
}

fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(&path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

