use anyhow::{Result, Context};
use std::fs::File;
use memmap2::Mmap;
use object::{Object, ObjectSection};
use addr2line::Context as A2lContext;

pub struct DwarfCtx<'a> {
    ctx: A2lContext<gimli::EndianSlice<'a, gimli::RunTimeEndian>>,
    data: Mmap,
}

impl<'a> DwarfCtx<'a> {
    pub fn load(path: &str) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("open vmlinux: {}", path))?;
        let data = unsafe { Mmap::map(&file)? };
        let obj = object::File::parse(&*data)?;
        let ctx = addr2line::Context::new(&obj)?;
        Ok(Self { ctx, data })
    }

    pub fn addr_to_line(&self, addr: u64) -> Result<Option<(String, u32)>> {
        if let Ok(frames) = self.ctx.find_frames(addr) {
            for f in frames {
                if let Ok(f) = f {
                    if let (Some(file), Some(line)) = (f.file, f.line) {
                        return Ok(Some((file.to_string_lossy().to_string(), line)));
                    }
                }
            }
        }
        Ok(None)
    }
}
