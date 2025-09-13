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
