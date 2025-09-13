# kdu
Rust-based crash-dump/Oops analyzer you can actually build and run.  Kernel Debug Utility (Rust) — “kdu”

# Key dependencies
        regex (parsing), lazy_static
        anyhow, thiserror
        object, gimli, addr2line (DWARF from vmlinux)
        memmap2 (fast file IO), serde+serde_json
        clap (CLI)
        rayon (optional, for faster symbol map loading/bsearch)
        similar (optional diff/highlighting of instruction region)

# Command-line (initial)
                kdu analyze \
                  --log ./oops.txt \
                  --system-map /boot/System.map-$(uname -r) \
                  --vmlinux /usr/lib/debug/boot/vmlinux-$(uname -r) \
                  --arch x86_64 \
                  --out report.md
        Shortcuts:

                --from-dmesg (runs dmesg -T), --stdin
                --arch auto (deduce from log CPU/Machine lines)
                --format json|md|tty (default: tty)

# Core flow

        1. Parse
           Extract: header (Oops vs Panic), CPU/Pid/comm, RIP/IP/PC, CR2, registers, EFLAGS/PSR, error code, taint flags, stack + call trace, “Code:” bytes. (The guides show the exact fields we’ll parse.)
        2. Symbolicate (System.map)
           Load symbols, binary-search the nearest ≤ address to get symbol+offset, then confirm by matching PC math (as in the add_range example).
        3. Resolve to source (DWARF)
           If vmlinux provided, use addr2line-equivalent via gimli/addr2line to get file:line for the function+offset, akin to gdb list *(func+0xoff)
        4. Explain
           Decode page-fault error bits (present/protection, read/write, user/kernel), and taint flags (e.g., P proprietary module, F forced, S SMP mismatch, etc.),
        5. Render report
           TTY summary + optional Markdown/JSON—including a “Likely root cause” section (e.g., NULL deref at *(int*)0 style patterns), with the exact function/line and call-trace mapping.

# Dev tips

        Make sure vmlinux matches the exact running kernel build/config, same as the System.map matching rule emphasized in the guide (addresses change per build/config).
        For distro kernels, install matching debuginfo to get DWARF for vmlinux


# kdu Test Fixtures

This folder contains **sample logs** and **golden expected outputs** to exercise the Rust-based Kernel Debug Utility (kdu).  
Use these to write unit/integration tests without relying on a live kernel.

## Structure
- `tests/data/` — input fixtures (Oops/Panic logs and mock System.map files)
- `tests/expected/` — golden outputs (JSON or Markdown) that the renderer should produce

## Included Cases
1. **x86_64 NULL dereference**  
   - Input: `tests/data/oops_x86_null_deref.log`  
   - Symbols: `tests/data/System.map-5.15.mock`  
   - Expected: `tests/expected/oops_x86_null_deref.json`  
   - Notes: Fault code `0002` ⇒ present=0, write=1, user=0. PC should resolve to `my_oops_init+0x12`.

2. **ARMv7 worker NULL deref**  
   - Input: `tests/data/oops_arm_page_fault.log`  
   - Symbols: `tests/data/System.map-armv7.mock`  
   - Expected: `tests/expected/oops_arm_page_fault.json`  
   - Notes: `PC is at faulting_fn+0x8`. No x86-style `CR2`/error bits.

3. **x86_64 proprietary taint**  
   - Input: `tests/data/oops_x86_taint_prop.log`  
   - Symbols: `tests/data/System.map-6.1.mock`  
   - Expected: `tests/expected/oops_x86_taint_prop.md`  
   - Notes: Verifies taint parsing and Markdown rendering.

## Suggested Rust Tests (pseudo)
```rust
// crates/kdu-parsers/tests/oops_x86.rs
#[test]
fn parse_x86_null_deref() {
    let log = std::fs::read_to_string("tests/data/oops_x86_null_deref.log").unwrap();
    let smap = kdu_sym::SystemMap::load("tests/data/System.map-5.15.mock").unwrap();
    let rec = kdu_parsers::parse_oops(&log).unwrap();
    let (sym, off) = smap.resolve(rec.pc_addr).unwrap();
    assert_eq!(sym.name, "my_oops_init");
    assert_eq!(off, 0x12);
    let fault = kdu_taint::fault::decode_pf(0x0002);
    assert_eq!((fault.present, fault.write, fault.user), (false,true,false));
}
```

## Running your tool against fixtures
```bash
kdu analyze       --log tests/data/oops_x86_null_deref.log       --system-map tests/data/System.map-5.15.mock       --format json > /tmp/out.json

diff -u tests/expected/oops_x86_null_deref.json /tmp/out.json
```
