use crate::instructions::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AritmeticInstruction {
    ///Dual 16-bit signed saturating addition. See QADD16 on page A4-94.
    QADD {
        destination: Register,
        first_operand: Register,
        second_operand: Register,
    },
    QDADD {
        destination: Register,
        first_operand: Register,
        second_operand: Register,
    },
    QDSUB {
        destination: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///Dual 16-bit signed saturating addition. See QADD16 on page A4-94.
    QSUB {
        destination: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///CLZ on page A4-25.
    CLZ {
        destination: Register,
        source: Register,
    },
}
