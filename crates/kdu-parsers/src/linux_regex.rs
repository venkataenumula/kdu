use regex::Regex;

lazy_static::lazy_static! {
    // Examples from the guides: RIP/IP/PC + function+offset form
    pub static ref RE_PC_LINE: Regex = Regex::new(
        r#"(?x)
        (?:
          (?:RIP|IP)\s*:\s*(?:[0-9A-Fa-f]{4}:\s*)?\[\<([0-9A-Fa-f]+)\>\]\s*
          (?:([A-Za-z0-9_\.]+)\+0x([0-9A-Fa-f]+)(?:/\S+)?)?
        )
        |
        (?:PC\s+is\s+at\s+([A-Za-z0-9_\.]+)\+0x([0-9A-Fa-f]+))
        "#
    ).unwrap();

    pub static ref RE_CR2: Regex = Regex::new(r#"CR2:\s*([0-9A-Fa-fx]+)"#).unwrap();
    pub static ref RE_TAINT: Regex = Regex::new(r#"Tainted:\s*([A-Za-z\ \.]+)\s"#).unwrap();
    pub static ref RE_CALLTRACE: Regex = Regex::new(r#"\[\<([0-9A-Fa-f]+)\>\]\s*([A-Za-z0-9_\.]+)\+0x([0-9A-Fa-f]+)"#).unwrap();
    pub static ref RE_CODEHEX: Regex = Regex::new(r#"Code:\s*([0-9A-Fa-f\ \<\>\s]+)"#).unwrap();
}
