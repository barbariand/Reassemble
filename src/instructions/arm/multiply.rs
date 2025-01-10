use crate::instructions::{Register, ShifterOperand};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplyInstruction {
    ///Multiply Accumulate. See MLA on page A4-66.
    MLA {
        destination: Register,
        s: bool,
        first_operand: Register,
        second_operand: Register,
        add_operand: Register,
    },
    ///Multiply. See MUL on page A4-80.
    MUL {
        destination: Register,
        s: bool,
        first_operand: Register,
        second_operand: Register,
    },
    ///<x><y>
    ///Signed halfword Multiply Accumulate. See SMLA<x><y> on page A4-141.
    SMLA {
        x: bool,
        y: bool,
        destination: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///SMLAL <x><y>,Signed Multiply Accumulate Long. See SMLAL on page A4-146.
    SMLAL {
        s: bool,
        high: Register,
        low: Register,
        first_operand: Register,
        second_operand: Register,
    },
    SMLAL2 {
        x: bool,
        y: bool,
        high: Register,
        low: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///<y> Signed halfword by word Multiply Accumulate. See SMLAW<y> on page A4-152.
    SMLAW {
        y: bool,
        destination: Register,
        source: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///<x><y>
    ///Signed halfword Multiply. See SMUL<x><y> on page A4-166.
    SMUL {
        x: bool,
        y: bool,
        destination: Register,
        high: Register,
        low: Register,
    },
    ///Signed Multiply Long. See SMULL on page A4-168.
    SMULL {
        s: bool,
        high: Register,
        low: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///<y> Signed halfword by word Multiply. See SMULW<y> on page A4-170.
    SMULW {
        y: bool,
        destination: Register,
        source: Register,
        first_operand: Register,
    },

    ///Unsigned Multiply Accumulate Long. See UMLAL on page A4-249.
    UMLAL {
        s: bool,
        high: Register,
        low: Register,
        first_operand: Register,
        second_operand: Register,
    },
    ///Unsigned Multiply Long. See UMULL on page A4-251.
    UMULL {
        s: bool,
        high: Register,
        low: Register,
        first_operand: Register,
        second_operand: Register,
    },
}
