use riscv::read_write_csr;

read_write_csr! {
    /// Global Interrupt Enable Register
    Corecfgr: 0x800,
    mask: 0xFF,
}
