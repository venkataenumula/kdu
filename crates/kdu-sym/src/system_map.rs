use anyhow::{Context, Result};
use std::{fs::File, io::{BufRead, BufReader}};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Sym {
    pub addr: u64,
    pub kind: char,
    pub name: String,
}

#[derive(Debug)]
pub struct SystemMap {
    syms: Vec<Sym>, // sorted by addr
}

impl SystemMap {
    pub fn load(path: &str) -> Result<Self> {
        let f = File::open(path).with_context(|| format!("open System.map: {}", path))?;
        let mut syms = Vec::new();
        for line in BufReader::new(f).lines() {
            let l = line?;
            // Format: "ffffffff81000000 T _text"
            if l.is_empty() { continue; }
            let mut it = l.split_whitespace();
            let addr = u64::from_str_radix(it.next().unwrap(), 16)?;
            let kind = it.next().unwrap().chars().next().unwrap_or('T');
            let name = it.next().unwrap_or("").to_string();
            syms.push(Sym { addr, kind, name });
        }
        syms.sort_by_key(|s| s.addr);
        Ok(Self { syms })
    }

    /// Nearest symbol at or below `addr`
    pub fn resolve(&self, addr: u64) -> Option<(&Sym, u64)> {
        let i = self.syms.binary_search_by(|p| {
            if p.addr <= addr { Ordering::Less } else { Ordering::Greater }
        });
        let idx = match i {
            Ok(mut k) => {
                while k+1 < self.syms.len() && self.syms[k+1].addr <= addr { k += 1; }
                k
            },
            Err(0) => return None,
            Err(k) => k-1,
        };
        let sym = &self.syms[idx];
        Some((sym, addr.saturating_sub(sym.addr)))
    }
}
