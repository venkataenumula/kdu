use clap::{Parser, ValueEnum};
use anyhow::Result;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutFmt { Tty, Json, Md }

#[derive(Parser)]
struct Args {
    #[arg(long)] log: Option<String>,
    #[arg(long)] from_dmesg: bool,
    #[arg(long)] system_map: Option<String>,
    #[arg(long)] vmlinux: Option<String>,
    #[arg(long, default_value="tty")] format: OutFmt,
    #[arg(long)] out: Option<String>,
    #[arg(long, default_value="auto")] arch: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // 1) Read log (file/stdin/dmesg)
    // 2) Parse with kdu_parsers
    // 3) Symbolicate with System.map (if provided)
    // 4) Resolve lines with DWARF (if provided)
    // 5) Render via kdu_report
    Ok(())
}

