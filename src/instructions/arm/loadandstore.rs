use crate::instructions::{Register, RegisterList};

use super::{AddressingMode, PostIndexedAddressingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadAndStoreInstruction {
    ///Load Word. See LDR on page A4-43.
    LDR(LoadAndStoreGenericInsturction),
    ///Load Byte. See LDRB on page A4-46.
    LDRB(LoadAndStoreGenericInsturction),
    ///Load Byte with User Mode Privilege. See LDRBT on page A4-48.
    LDRBT(LoadAndStorePostIndexInstruction),
    ///Load Doubleword. See LDRD on page A4-50.
    LDRD(LoadAndStoreGenericInsturction),
    ///Load Exclusive. See LDREX on page A4-52.
    LDREX(LDREX),
    ///Load Unsigned Halfword. See LDRH on page A4-54.
    LDRH(LoadAndStoreGenericInsturction),
    ///Load Signed Byte. See LDRSB on page A4-56.
    LDRSB(LoadAndStoreGenericInsturction),
    ///Load Signed Halfword. See LDRSH on page A4-58.
    LDRSH(LoadAndStoreGenericInsturction),
    ///Load Word with User Mode Privilege. See LDRT on page A4-60.
    LDRT(LoadAndStoreGenericInsturction),
    ///Store Word. See STR on page A4-193.
    STR(LoadAndStoreGenericInsturction),
    ///Store Byte. See STRB on page A4-195.
    STRB(LoadAndStoreGenericInsturction),
    ///Store Byte with User Mode Privilege. See STRBT on page A4-197.
    STRBT(LoadAndStorePostIndexInstruction),
    ///Store Doubleword. See STRD on page A4-199.
    STRD(LoadAndStoreGenericInsturction),
    ///Store Halfword. See STRH on page A4-204.
    STRH(LoadAndStoreGenericInsturction),
    ///Store Word with User Mode Privilege. See STRT on page A4-206.
    STRT(LoadAndStoreGenericInsturction),
    Multiple(LoadAndStoreMultiple),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadAndStoreGenericInsturction {
    destination: Register,
    adressing_mode: AddressingMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadAndStorePostIndexInstruction {
    destination: Register,
    adressing_mode: PostIndexedAddressingMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LDREX {
    destination: Register,
    adress: Register,
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
    LDMR {
        adressing_mode: AddressingMode,
        base: Register,
        registers: RegisterList,
    },
    ///Load Multiple with Restore CPSR. See LDM (3) on page A4-40.
    LDMC {
        adressing_mode: AddressingMode,
        base: Register,
        registers: RegisterList,
    },
    ///Store Multiple. See STM (1) on page A4-189.
    STM {
        adressing_mode: AddressingMode,
        base: Register,
        write: bool,
        registers: RegisterList,
    },
    ///Store Multiple. See STM (1) on page A4-189.
    STM2 {
        adressing_mode: AddressingMode,
        base: Register,
        write: bool,
        registers: RegisterList,
    },
}
