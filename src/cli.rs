use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 's',
        long,
        value_name = "PCI-ADDR",
        help = "Inspect a device (e.g. 0000:00:1f.0)"
    )]
    pub address: Option<String>,

    #[arg(long, help = "Dump the output to stdout instead of starting TUI")]
    pub dump: bool,
}
