
use assert_cmd::prelude::*;
use std::process::Command;
use anyhow::{Result, Context};
use similar::TextDiff;
mod common;
use common::*;

fn run_kdu_md(log_rel: &str, smap_rel: &str) -> Result<String> {
    let bin = std::env::var("KDU_BIN").unwrap_or_else(|_| "kdu".to_string());
    let mut cmd = Command::cargo_bin(&bin)
        .with_context(|| format!("Looking for cargo bin: {}", bin))?;

    let out = cmd
        .arg("analyze")
        .arg("--log").arg(fx(log_rel))
        .arg("--system-map").arg(fx(smap_rel))
        .arg("--format").arg("md")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let s = String::from_utf8(out).context("utf8 decode")?;
    Ok(s.trim().to_string())
}

#[test]
fn markdown_taint_matches_golden_with_diff() -> Result<()> {
    let got = run_kdu_md(
        "data/oops_x86_taint_prop.log",
        "data/System.map-6.1.mock"
    )?;
    let exp = read_string("expected/oops_x86_taint_prop.md")?.trim().to_string();

    if got != exp {
        let diff = TextDiff::from_lines(&exp, &got);
        let mut pretty = String::new();
        for op in diff.ops() {
            for change in diff.iter_changes(&op) {
                let sign = match change.tag() {
                    similar::ChangeTag::Delete => "-",
                    similar::ChangeTag::Insert => "+",
                    similar::ChangeTag::Equal  => " ",
                };
                pretty.push_str(&format!("{}{}", sign, change));
            }
        }
        panic!("Markdown mismatch:\n{}", pretty);
    }
    Ok(())
}
