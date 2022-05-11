#![feature(int_log)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use itertools::{Either, Itertools};

#[derive(Parser)]
#[clap(name = "linelength")]
#[clap(author = "Aiko Wessels <aiko.wessels@gmail.com>")]
struct Opts {
    /// List of files for which to compute the length of the longest line, as well as the line
    /// number.
    #[clap(required = true)]
    files: Vec<String>,
}

struct LineLength<'file> {
    file: &'file str,
    length: usize,
    index: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let (succeeded, failed): (Vec<_>, Vec<_>) =
        opts.files
            .iter()
            .partition_map(|filename| match File::open(filename) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let (len, idx) =
                        reader
                            .lines()
                            .enumerate()
                            .fold((0, 0), |(len, idx), (cur_idx, line)| {
                                let line = line.unwrap();
                                if line.len() < len {
                                    (len, idx)
                                } else {
                                    (line.len(), cur_idx)
                                }
                            });
                    Either::Left(LineLength {
                        file: filename,
                        length: len,
                        index: idx,
                    })
                }
                Err(msg) => Either::Right(format!("Unable to open file {}: {}", filename, msg)),
            });

    let file_padding = succeeded
        .iter()
        .fold(0, |len, ll| std::cmp::max(len, ll.file.len()));
    let file_padding = std::cmp::max(file_padding, 4) as usize;

    let length_padding = succeeded
        .iter()
        .fold(0, |len, ll| std::cmp::max(len, ll.length))
        .log10();
    let length_padding = std::cmp::max(length_padding + 1, 6) as usize;

    let index_padding = succeeded
        .iter()
        .fold(0, |len, ll| std::cmp::max(len, ll.index))
        .log10();
    let index_padding = std::cmp::max(index_padding + 1, 5) as usize;

    println!(
        "{file:<file_padding$} {length:<length_padding$} {index:<index_padding$}",
        file = "File",
        length = "Length",
        index = "Index"
    );
    for LineLength {
        file,
        length,
        index,
    } in &succeeded
    {
        println!("{file:<file_padding$} {length:>length_padding$} {index:>index_padding$}");
    }

    for err in &failed {
        println!("{}", err);
    }

    Ok(())
}
