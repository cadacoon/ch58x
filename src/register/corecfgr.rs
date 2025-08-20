use riscv::read_write_csr;

read_write_csr! {
    /// Microprocessor configuration register
    Corecfgr: 0xBC0,
    mask: 0xFF,
}
