//! using https://documentation-service.arm.com/static/5f8dacc8f86e16515cdb865a?token=#E3.Ciaeiijh
use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CRegister {
    Cr1,
    Cr2,
    Cr3,
    Cr4,
    Cr5,
    Cr6,
    Cr7,
    Cr8,
    Cr9,
    Cr10,
    Cr11,
    Cr12,
    Cr13,
    Cr14,
    Cr15,
}
pub enum BranchInstructions {
    ///Branch. See B on page A4-10.
    B,
    ///Branch with Link. See BL on page A4-10.
    BL,
    ///Branch with Link and Exchange. See BLX (1) on page A4-16 and BLX (2) on page A4-18.
    BLX,
    ///Branch and Exchange Instruction Set. See BX on page A4-20.
    BX,
    ///Branch and change to Jazelle state. See BXJ on page A4-21.
    BXJ,
}
pub enum DataProssessingInstructions {
    ///Add with Carry. See ADC on page A4-4.
    ADC,
    ///Add. See ADD on page A4-6.
    ADD,
    ///Logical AND. See AND on page A4-8.
    AND,
    ///Logical Bit Clear. See BIC on page A4-12.
    BIC,
    ///Compare Negative. See CMN on page A4-26.
    CMN,
    ///Compare. See CMP on page A4-28.
    CMP,
    ///Logical EOR. See EOR on page A4-32.
    EOR,
    ///Move. See MOV on page A4-68.
    MOV,
    ///Move Not. See MVN on page A4-82.
    MVN,
    ///Logical OR. See ORR on page A4-84.
    ORR,
    ///Reverse Subtract. See RSB on page A4-115.
    RSB,
    ///Reverse Subtract with Carry. See RSC on page A4-117.
    RSC,
    ///Subtract with Carry. See SBC on page A4-125.
    SBC,
    ///Subtract. See SUB on page A4-208.
    SUB,
    ///Test Equivalence. See TEQ on page A4-228.
    TEQ,
    ///Test. See TST on page A4-230.
    TST,
}
pub enum MultiplyInstructions {
    ///Multiply Accumulate. See MLA on page A4-66.
    MLA,
    ///Multiply. See MUL on page A4-80.
    MUL,
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
pub enum AritmaticInstructions {
    ///Dual 16-bit signed saturating addition. See QADD16 on page A4-94.
    QADD16,
    ///Quad 8-bit signed saturating addition. See QADD8 on page A4-95.
    QADD8,
    ///16-bit exchange, signed saturating addition, subtraction. See QADDSUBX on page A4-97.
    QADDSUBX,
    ///Dual 16-bit signed saturating subtraction. See QSUB16 on page A4-104.
    QSUB16,
    ///Quad 8-bit signed saturating subtraction. See QSUB8 on page A4-105.
    QSUB8,
    ///16-bit exchange, signed saturating subtraction, addition. See QSUBADDX on page A4-107.
    QSUBADDX,
    ///Dual 16-bit signed addition. See SADD16 on page A4-119.
    SADD16,
    ///Quad 8-bit signed addition. See SADD8 on page A4-121.
    SADD8,
    ///16-bit exchange, signed addition, subtraction. See SADDSUBX on page A4-123.
    SADDSUBX,
    ///Dual 16-bit signed subtraction. See SSUB16 on page A4-180.
    SSUB16,
    ///Quad 8-bit signed subtraction. See SSUB8 on page A4-182.
    SSUB8,
    ///16-bit exchange, signed subtraction, addition. See SSUBADDX on page A4-184.
    SSUBADDX,
    ///Dual 16-bit signed half addition. See SHADD16 on page A4-130.
    SHADD16,
    ///Quad 8-bit signed half addition. See SHADD8 on page A4-131.
    SHADD8,
    ///16-bit exchange, signed half addition, subtraction. See SHADDSUBX on page A4-133.
    SHADDSUBX,
    ///Dual 16-bit signed half subtraction. See SHSUB16 on page A4-135.
    SHSUB16,
    ///Quad 8-bit signed half subtraction. See SHSUB8 on page A4-137.
    SHSUB8,
    ///16-bit exchange, signed half subtraction, addition. See SHSUBADDX on page A4-139.
    SHSUBADDX,
    ///Dual 16-bit unsigned addition. See UADD16 on page A4-232.
    UADD16,
    ///Quad 8-bit unsigned addition. See UADD8 on page A4-233.
    UADD8,
    ///16-bit exchange, unsigned addition, subtraction. See UADDSUBX on page A4-235.
    UADDSUBX,
    ///Dual 16-bit unsigned subtraction. See USUB16 on page A4-269.
    USUB16,
    ///Quad 8-bit unsigned subtraction. See USUB8 on page A4-270.
    USUB8,
    ///16-bit exchange, unsigned subtraction, addition. See USUBADDX on page A4-272.
    USUBADDX,
    ///Dual 16-bit unsigned half addition. See UHADD16 on page A4-237.
    UHADD16,
    ///Quad 8-bit unsigned half addition. See UHADD8 on page A4-238.
    UHADD8,
    ///16-bit exchange, unsigned half addition, subtraction. See UHADDSUBX on page A4-240.
    UHADDSUBX,
    ///Dual 16-bit unsigned half subtraction. See UHSUB16 on page A4-242.
    UHSUB16,
    ///Quad 8-bit unsigned half subtraction. See UHSUB16 on page A4-242.
    UHSUB8,
    ///16-bit exchange, unsigned half subtraction, addition. See UHSUBADDX on page A4-245.
    UHSUBADDX,
    ///Dual 16-bit unsigned saturating addition. See UQADD16 on page A4-253.
    UQADD16,
    ///Quad 8-bit unsigned saturating addition. See UQADD8 on page A4-254.
    UQADD8,
    ///16-bit exchange, unsigned saturating addition, subtraction. See UQADDSUBX on page A4-255.
    UQADDSUBX,
    ///Dual 16-bit unsigned saturating subtraction. See UQSUB16 on page A4-257.
    UQSUB16,
    ///Quad 8-bit unsigned saturating subtraction. See UQSUB8 on page A4-258.
    UQSUB8,
    ///16-bit exchange, unsigned saturating subtraction, addition. See UQSUBADDX on page A4-259.
    UQSUBADDX,
    ///CLZ on page A4-25.
    CLZ,
}

pub enum LoadAndStore {
    ///Load Word. See LDR on page A4-43.
    LDR,
    ///Load Byte. See LDRB on page A4-46.
    LDRB,
    ///Load Byte with User Mode Privilege. See LDRBT on page A4-48.
    LDRBT,
    ///Load Doubleword. See LDRD on page A4-50.
    LDRD,
    ///Load Exclusive. See LDREX on page A4-52.
    LDREX,
    ///Load Unsigned Halfword. See LDRH on page A4-54.
    LDRH,
    ///Load Signed Byte. See LDRSB on page A4-56.
    LDRSB,
    ///Load Signed Halfword. See LDRSH on page A4-58.
    LDRSH,
    ///Load Word with User Mode Privilege. See LDRT on page A4-60.
    LDRT,
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
    Multiple()
}
pub enum LoadAndStoreMultiple{
LDM Load Multiple. See LDM (1) on page A4-36.
LDM User Registers Load Multiple. See LDM (2) on page A4-38.
LDM Load Multiple with Restore CPSR. See LDM (3) on page A4-40.
STM Store Multiple. See STM (1) on page A4-189.DateIDTM User Registers Store Multiple. See STM (2) on page A4-191
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // Branch Instructions
    B(u32),
    Bl(u32),
    Bx(Register),
    // Data Processing Instructions
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    And(Register, Register, Register),
    Xor(Register, Register, Register),
    Or(Register, Register, Register),
    Mov(Register, Register),
    Cmp(Register, Register),
    Mvn(Register, Register),
    // Load/Store Instructions
    Ldr(Register, Register),
    Str(Register, Register),
    Ldrb(Register, Register),
    Strb(Register, Register),
    Ldrh(Register, Register),
    Strh(Register, Register),
    Ldm(Register, Vec<Register>),
    Stm(Register, Vec<Register>),
    // Multiply Instructions
    Mul(Register, Register, Register),
    Mla(Register, Register, Register, Register),
    Umull(Register, Register, Register, Register),
    Umulal(Register, Register, Register, Register),
    Smlal(Register, Register, Register, Register),
    Smull(Register, Register, Register, Register),
    // Data Processing Instructions (with S flag)
    AddS(Register, Register, Register),
    SubS(Register, Register, Register),
    AndS(Register, Register, Register),
    EorS(Register, Register, Register),
    OrS(Register, Register, Register),
    MovS(Register, Register),
    CmpS(Register, Register),
    // Load/Store Multiple Instructions
    Ldmda(Register, Vec<Register>),
    Ldmfa(Register, Vec<Register>),
    Ldmdb(Register, Vec<Register>),
    Ldmib(Register, Vec<Register>),
    Stmda(Register, Vec<Register>),
    Stmfa(Register, Vec<Register>),
    Stmdb(Register, Vec<Register>),
    Stmib(Register, Vec<Register>),
    // Media Instructions
    Sadd16(Register, Register, Register),
    Sub16(Register, Register, Register),
    Shl16(Register, Register, Register),
    Asr16(Register, Register, Register),
    Rrx(Register, Register),
    // Miscellaneous Instructions
    Swp(Register, Register, Register),
    Swpb(Register, Register, Register),
    // Coprocessor Instructions
    Cdcc(u8, u8, u8, u8),
    Cdc(u8, u8, u8, u8),
    Mrc(u8, u8, u8, Register, Register, u8),
    Mrc2(u8, u8, u8, Register, Register, u8),
    // Coprocessor Data Transfer Instructions
    Ldc(u8, u8, Register, Register),
    Ldc2(u8, u8, Register, Register),
    Stc(u8, u8, Register, Register),
    Stc2(u8, u8, Register, Register),
}
impl TryFrom<u32> for Instruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!()
    }
}
