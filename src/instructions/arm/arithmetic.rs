use crate::instructions::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AritmeticInstruction {
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
    CLZ{
        destination:Register,
        source:Register
    },
}

