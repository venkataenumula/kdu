# KDU Report (TTY → Markdown)

**Arch:** x86_64  
**Type:** oops  

**PC:** 0xffffffffa07b900a → `prop_handle+0x3a` (System.map match)  
**Taint:** `P` (proprietary module present), `OE` (externally built/unsigned)  

**Call Trace**
- 0xffffffff810a3cf0 : `do_one_initcall+0x50`

**Code bytes around PC**
`48 8b 07 ff 10 48 8b 0f ff 11 <48> 8b 17 ff 12 c3`

**Conclusion:** Crash occurred inside a proprietary module handler; verify symbol alignment and check for invalid pointer access or ABI mismatch between kernel and out-of-tree module.
