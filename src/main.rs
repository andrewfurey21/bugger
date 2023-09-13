use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    create: Option<String>,

    #[arg(short, long, default_value_t=true)]
    resolve: Option<bool>,
    #[arg(short, long, default_value_t=true)]
    temp: Option<bool>,
    #[arg(short, long, default_value_t=true)]
    unsolved: Option<bool>,

    #[arg(short, long)]
    delete: Option<u32>,

    #[arg(short, long)]
    desc: Option<String>,
    #[arg(short, long)]
    append: Option<String>,

}

fn main() {
    let args = Args::parse();

    match args.create {
        Some(desc) => println!("{}", desc),
        None => (),
    }

    for (i, tag) in args.tags.iter().enumerate() {
        println!("index: {}, tag: {}", i, tag);
    }

}
