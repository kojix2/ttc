use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "token-counter", about = "Count tokens in files.")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(
        short = "e",
        long,
        default_value = "cl100k_base",
        help = "Encoding model (cl100k_base|p50k_base|p50k_edit|r50k_base)"
    )]
    encoding: String,
}

// Get the encoder based on the encoding model name
fn get_encoder(encoding: &str) -> tiktoken_rs::CoreBPE {
    match encoding {
        "cl100k_base" => tiktoken_rs::cl100k_base().unwrap(),
        "p50k_base" => tiktoken_rs::p50k_base().unwrap(),
        "p50k_edit" => tiktoken_rs::p50k_edit().unwrap(),
        "r50k_base" | "gpt2" => tiktoken_rs::r50k_base().unwrap(),
        _ => {
            eprintln!("Error: invalid encoding name");
            process::exit(1);
        }
    }
}

// Count the number of tokens in the given content
fn count_tokens(content: &str, encoder: &tiktoken_rs::CoreBPE) -> usize {
    encoder.encode_with_special_tokens(content).len()
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let encoder = get_encoder(&opt.encoding);

    if opt.files.is_empty() {
        // If no files are provided, read from stdin
        let stdin_content = read_from_stdin()?;
        let token_count = count_tokens(&stdin_content, &encoder);
        println!("{}", token_count);
    } else {
        // If files are provided, count tokens in each file
        count_tokens_in_files(&opt.files, &encoder);
    }

    Ok(())
}

// Read content from standard input
fn read_from_stdin() -> io::Result<String> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    Ok(content)
}

// Count tokens in each file and print the results
fn count_tokens_in_files(file_paths: &[PathBuf], encoder: &tiktoken_rs::CoreBPE) {
    let mut total_tokens = 0;
    let mut file_count = 0;

    for path in file_paths {
        match read_file(&path) {
            Ok(content) => {
                let token_count = count_tokens(&content, &encoder);
                println!("{:8} {}", token_count, path.display());
                total_tokens += token_count;
                file_count += 1;
            }
            Err(e) => eprintln!("Error reading file {}: {}", path.display(), e),
        }
    }

    // Print the total token count if more than one file was processed
    if file_count > 1 {
        println!("{:8} total", total_tokens);
    }
}

// Read the content of a file
fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(&path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}
