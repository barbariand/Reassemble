
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataProcessingAddressingMode{
    Immediate(),
    Register(),
    LogicalShiftLeftImm()
}
