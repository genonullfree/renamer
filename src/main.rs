use std::fs;
use std::io::Read;

use clap::Parser;

static TMP: &str = "tmp.file";

#[derive(Debug, Parser)]
enum Cmd {
    Swap(SwapArgs),
    Map(MapArgs),
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Parser)]
struct SwapArgs {
    #[arg(short)]
    a: String,

    #[arg(short)]
    b: String,
}

#[derive(Debug, Parser)]
struct MapArgs {
    #[arg(short, long)]
    map: String,
}

fn main() -> std::io::Result<()> {
    // Process arguments
    let opt = Args::parse();

    match opt.cmd {
        Cmd::Swap(a) => swap(&a.a, &a.b)?,
        Cmd::Map(m) => map(&m.map)?,
    };

    Ok(())
}

fn map(mapfile: &str) -> std::io::Result<()> {
    let mut map = String::new();
    fs::File::open(mapfile)?.read_to_string(&mut map)?;

    for line in map.lines() {
        let split = line.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("incorrectly formatted map file on line: {:?}", line);
        }
        swap(split[0], split[1])?;
    }

    Ok(())
}

fn swap(a: &str, b: &str) -> std::io::Result<()> {
    mv(a, TMP)?;
    mv(b, a)?;
    mv(TMP, b)?;

    Ok(())
}

fn mv(a: &str, b: &str) -> std::io::Result<()> {
    if !std::path::Path::new(a).exists() {
        panic!("whoops - {} does not exist", a);
    }
    if std::path::Path::new(b).exists() {
        panic!("whoops - {} exists", b);
    }
    fs::rename(a, b)
}
