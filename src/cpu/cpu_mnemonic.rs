#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mnemonic {
    InvalidOpcode,
    NoOpcode,
    NOP,
    AAA,
    AAD,
    AAM,
    AAS,
    ADC,
    ADD,
    AND,
    CALL,
    CALLF,
    CBW,
    CLC,
    CLD,
    CLI,
    CMC,
    CMP,
    CMPSB,
    CMPSW,
    CWD,
    DAA,
    DAS,
    DEC,
    DIV,
    ESC,
    FWAIT,
    HLT,
    IDIV,
    IMUL,
    IN,
    INC,
    INT,
    INT3,
    INTO,
    IRET,
    JB,
    JBE,
    JCXZ,
    JL,
    JLE,
    JMP,
    JMPF,
    JNB,
    JNBE,
    JNL,
    JNLE,
    JNO,
    JNP,
    JNS,
    JNZ,
    JO,
    JP,
    JS,
    JZ,
    LAHF,
    LDS,
    LEA,
    LES,
    LOCK,
    LODSB,
    LODSW,
    LOOP,
    LOOPNE,
    LOOPE,
    MOV,
    MOVSB,
    MOVSW,
    MUL,
    NEG,
    NOT,
    OR,
    OUT,
    POP,
    POPF,
    PUSH,
    PUSHF,
    RCL,
    RCR,
    REP,
    REPNE,
    REPE,
    RETF,
    RETN,
    ROL,
    ROR,
    SAHF,
    SALC,
    SAR,
    SBB,
    SCASB,
    SCASW,
    SETMO,
    SETMOC,
    SHL,
    SHR,
    STC,
    STD,
    STI,
    STOSB,
    STOSW,
    SUB,
    TEST,
    XCHG,
    XLAT,
    XOR,
}

impl Default for Mnemonic {
    fn default() -> Self { Mnemonic::InvalidOpcode }
}


