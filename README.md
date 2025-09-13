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
