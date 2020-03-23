use clap::{App, Arg, ArgMatches};
use std::error::Error;
use tokio::fs;
#[allow(unused)]
use tokio::prelude::*;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    init_logger();
    let m = App::new("gen-input")
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .default_value("target/input.txt"),
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(true)
                .default_value("1000000"),
        )
        .arg(
            Arg::with_name("per-line")
                .takes_value(true)
                .default_value("63"),
        )
        .get_matches();

    let file = m.value_of("file").unwrap();
    debug!("creating file: {:?}", file);
    let file = fs::File::create(file).await?;

    gen_input(file, m).await
}

async fn gen_input(mut f: fs::File, m: ArgMatches<'_>) -> Result<(), Box<dyn Error + 'static>> {
    use rand::Rng;
    let line_number = m.value_of("lines").unwrap().parse::<u64>().unwrap();
    let char_number = m.value_of("per-line").unwrap().parse::<u64>().unwrap();
    let mut rng = rand::thread_rng();

    for idx in 0..line_number {
        let n = rng.gen_range(0, char_number);
        let mut line: String = rng
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(n as usize)
            .collect();

        debug!("line {}: {}",idx, line);
        line.push_str("\n");
        f.write_all(line.as_bytes()).await?;
    }
    Ok(())
}

fn init_logger() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting subscriber failed");
}
