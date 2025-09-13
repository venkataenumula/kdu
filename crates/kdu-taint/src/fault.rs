pub struct PfErrorBits {
    pub present: bool,   // bit0: 0=no page, 1=protection fault
    pub write: bool,     // bit1: 0=read, 1=write
    pub user: bool,      // bit2: 0=kernel, 1=user
}

pub fn decode_pf(bits: u32) -> PfErrorBits {
    PfErrorBits {
        present: (bits & 0b001) != 0,
        write:   (bits & 0b010) != 0,
        user:    (bits & 0b100) != 0,
    }
}
