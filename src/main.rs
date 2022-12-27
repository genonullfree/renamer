use std::fs;
use std::io::Read;

use clap::Parser;

static TMP: &str = "tmp.file";

#[derive(Debug, Parser)]
enum Cmd {
    Swap(SwapArgs),
    Map(MapArgs),
    Name(NameArgs),
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

#[derive(Debug, Parser)]
struct NameArgs {
    #[arg(short, long)]
    prefix: String,

    #[arg(short, long, default_value_t = 1)]
    start_num: usize,

    #[arg(short, long, num_args = ..)]
    files: Vec<String>,

    #[arg(short, long)]
    reverse: bool,
}

fn main() -> std::io::Result<()> {
    // Process arguments
    let opt = Args::parse();

    match opt.cmd {
        Cmd::Swap(a) => swap(&a.a, &a.b)?,
        Cmd::Map(m) => map(&m.map)?,
        Cmd::Name(n) => name(n)?,
    };

    Ok(())
}

fn name(opt: NameArgs) -> std::io::Result<()> {
    let mut count = opt.start_num;
    if opt.reverse {
        for item in opt.files.iter().rev() {
            do_move(&opt.prefix, count, item)?;
            count += 1;
        }
    } else {
        for item in opt.files.iter() {
            do_move(&opt.prefix, count, item)?;
            count += 1;
        }
    }

    Ok(())
}

fn do_move(prefix: &str, count: usize, item: &str) -> std::io::Result<()> {
    let ext = detect_extension(item);
    let dest = format!("{}{:02}.{}", prefix, count, ext);

    if item == dest {
        println!("skipping move {item}");
        return Ok(());
    }

    println!("{item} => {dest}");
    mv(item, &dest)?;

    Ok(())
}

fn detect_extension(input: &str) -> &str {
    input.split('.').collect::<Vec<&str>>().pop().unwrap()
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
    if a == b {
        println!("skipping swap: {a}");
        return Ok(());
    }
    println!("{a} <=> {b}");

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
