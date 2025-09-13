
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use serde_json::Value;
use anyhow::{Result, Context};
mod common;
use common::*;

fn run_kdu_json(log_rel: &str, smap_rel: &str) -> Result<Value> {
    let bin = std::env::var("KDU_BIN").unwrap_or_else(|_| "kdu".to_string());
    let mut cmd = Command::cargo_bin(&bin)
        .with_context(|| format!("Looking for cargo bin: {}", bin))?;

    let out = cmd
        .arg("analyze")
        .arg("--log").arg(fx(log_rel))
        .arg("--system-map").arg(fx(smap_rel))
        .arg("--format").arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let val: Value = serde_json::from_slice(&out)
        .context("deserialize JSON from kdu output")?;
    Ok(val)
}

#[test]
fn json_x86_null_deref_matches_golden() -> Result<()> {
    let got = run_kdu_json(
        "data/oops_x86_null_deref.log",
        "data/System.map-5.15.mock"
    )?;
    let exp_s = read_string("expected/oops_x86_null_deref.json")?;
    let exp: Value = serde_json::from_str(&exp_s)?;

    // Strict deep equality; adjust your renderer if needed
    assert_eq!(got, exp, "JSON does not match golden for x86_64 NULL deref");
    Ok(())
}

#[test]
fn json_arm_null_deref_matches_golden() -> Result<()> {
    let got = run_kdu_json(
        "data/oops_arm_page_fault.log",
        "data/System.map-armv7.mock"
    )?;
    let exp_s = read_string("expected/oops_arm_page_fault.json")?;
    let exp: Value = serde_json::from_str(&exp_s)?;

    assert_eq!(got, exp, "JSON does not match golden for ARMv7 case");
    Ok(())
}
