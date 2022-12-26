use std::fs;

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
        Cmd::Map(_m) => todo!(),
    };

    Ok(())
}

fn swap(a: &str, b: &str) -> std::io::Result<()> {
    mv(a, TMP)?;
    mv(b, a)?;
    mv(TMP, b)?;

    Ok(())
}

fn mv(a: &str, b: &str) -> std::io::Result<()> {
    if std::path::Path::new(b).exists() {
        panic!("whoops - {} exists", b);
    }
    fs::rename(a, b)
}
