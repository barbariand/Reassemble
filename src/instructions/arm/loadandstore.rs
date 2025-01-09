use crate::instructions::{Register, RegisterList};

use super::{AddressingMode, PostIndexedAddressingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadAndStoreInstruction {
    ///Load Word. See LDR on page A4-43.
    LDR{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Load Byte. See LDRB on page A4-46.
    LDRB{
        destination:Register,
            adressing_mode:AddressingMode,
    },
    ///Load Byte with User Mode Privilege. See LDRBT on page A4-48.
    LDRBT{
        destination:Register,
        adressing_mode:PostIndexedAddressingMode,
    },
    ///Load Doubleword. See LDRD on page A4-50.
    LDRD{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Load Exclusive. See LDREX on page A4-52.
    LDREX{
        destination:Register,
        adress:Register,
    },
    ///Load Unsigned Halfword. See LDRH on page A4-54.
    LDRH{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Load Signed Byte. See LDRSB on page A4-56.
    LDRSB{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Load Signed Halfword. See LDRSH on page A4-58.
    LDRSH{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Load Word with User Mode Privilege. See LDRT on page A4-60.
    LDRT{
        destination:Register,
        adressing_mode:AddressingMode,
    },
    ///Store Word. See STR on page A4-193.
    STR,
    ///Store Byte. See STRB on page A4-195.
    STRB,
    ///Store Byte with User Mode Privilege. See STRBT on page A4-197.
    STRBT,
    ///Store Doubleword. See STRD on page A4-199.
    STRD,
    ///Store Exclusive. See STREX on page A4-202.
    STREX,
    ///Store Halfword. See STRH on page A4-204.
    STRH,
    ///Store Word with User Mode Privilege. See STRT on page A4-206.
    STRT,
    Multiple(LoadAndStoreMultiple),
}
impl From<LoadAndStoreMultiple> for LoadAndStoreInstruction {
    fn from(value: LoadAndStoreMultiple) -> Self {
        Self::Multiple(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadAndStoreMultiple {
    ///Load Multiple. See LDM (1) on page A4-36.
    LDM {
        adressing_mode: AddressingMode,
        base: Register,
        write: bool,
        registers: RegisterList,
    },
    ///User Registers Load Multiple. See LDM (2) on page A4-38.
    LDMR{
        adressing_mode: AddressingMode,
        base: Register,
        registers: RegisterList,
    },
    ///Load Multiple with Restore CPSR. See LDM (3) on page A4-40.
    LDMC{
        adressing_mode: AddressingMode,
        base: Register,
        registers: RegisterList,
    },
    ///Store Multiple. See STM (1) on page A4-189.
    STM,
    ///User Registers Store Multiple. See STM (2) on page A4-191.
    STMR,
}
