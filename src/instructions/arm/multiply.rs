use crate::instructions::{Register, ShifterOperand};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplyInstruction {
    ///Multiply Accumulate. See MLA on page A4-66.
    MLA{
        destination:Register,
        s:bool,
        first_operand:Register,
        second_operand:Register,
        add_operand:Register,
    },
    ///Multiply. See MUL on page A4-80.
    MUL{
        destination:Register,
        s:bool,
        shifter_operand:ShifterOperand,
    },
    ///<x><y>
    ///Signed halfword Multiply Accumulate. See SMLA<x><y> on page A4-141.
    SMLA,
    ///Signed halfword Multiply Accumulate, Dual. See SMLAD on page A4-144.
    SMLAD,
    ///Signed Multiply Accumulate Long. See SMLAL on page A4-146.
    SMLAL,
    ///SMLAL <x><y> Signed halfword Multiply Accumulate Long. See SMLAL<x><y> on page A4-148.

    ///Signed halfword Multiply Accumulate Long, Dual. See SMLALD on page A4-150.
    SMLALD,
    ///<y> Signed halfword by word Multiply Accumulate. See SMLAW<y> on page A4-152.
    SMLAW,
    ///Signed halfword Multiply Subtract, Dual. See SMLAD on page A4-144.
    SMLSD,
    ///Signed halfword Multiply Subtract Long Dual. See SMLALD on page A4-150.
    SMLSLD,
    ///Signed Most significant word Multiply Accumulate. See SMMLA on page A4-158.
    SMMLA,
    ///Signed Most significant word Multiply Subtract. See SMMLA on page A4-158.
    SMMLS,
    ///Signed Most significant word Multiply. See SMMUL on page A4-162.
    SMMUL,
    ///Signed halfword Multiply, Add, Dual. See SMUAD on page A4-164.
    SMUAD,
    ///<x><y>
    ///Signed halfword Multiply. See SMUL<x><y> on page A4-166.
    SMUL,
    ///Signed Multiply Long. See SMULL on page A4-168.
    SMULL,
    ///<y> Signed halfword by word Multiply. See SMULW<y> on page A4-170.
    SMULW,
    ///Signed halfword Multiply, Subtract, Dual. See SMUSD on page A4-172.
    SMUSD,
    ///Unsigned Multiply Accumulate significant Long. See UMAAL on page A4-247.
    UMAAL,
    ///Unsigned Multiply Accumulate Long. See UMLAL on page A4-249.
    UMLAL,
    ///Unsigned Multiply Long. See UMULL on page A4-251.
    UMULL,
}

