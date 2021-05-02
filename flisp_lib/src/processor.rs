use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
enum FlispError {
	InvalidOpCode,
}

#[derive(Debug, Clone, PartialEq)]
struct Flisp {
	A: u8,
	Y: u8,
	X: u8,
	CC: u8,
	SP: u8,
	mem: [u8; 256],
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
	ADCA(AdcaAddr),
	ADDA(AddaAddr),
	ANDA(AndaAddr),
	ANDCC,
	ASLA,
	ASL(AslAddr),
	ASRA,
	ASR(AsrAddr),
	BITA(BitaAddr),
	BLE,
	BLS,
	BLT,
	BMI,
	BNE,
	BPL,
	BRA,
	BSR,
	BVC,
	BVS,
	BCC,
	BCS,
	BEQ,
	BGE,
	BGT,
	BHI,
	CLRA,
	CLR(ClrAddr),
	CMPA(CmpaAddr),
	CMPX(CmpxAddr),
	CMPY(CmpyAddr),
	CMPSP(CmpspAddr),
	COMA,
	COM(ComAddr),
	DECA,
	DEC(DecAddr),
	EORA(EoraAddr),
	EXG(ExgAddr),
	INCA,
	INC(IncAddr),
	JMP(JmpAddr),
	JSR(JsrAddr),
	LDA(LdaAddr),
	LDX(LdxAddr),
	LDY(LdyAddr),
	LDSP(LdspAddr),
	LEAX(LeaxAddr),
	LEAY(LeayAddr),
	LEASP(LeaspAddr),
	LSRA,
	LSR(LsrAddr),
	NEGA,
	NEG(NegAddr),
	NOP,
	ORA(OraAddr),
	ORCC,
	PSHA,
	PSHX,
	PSHY,
	PSHCC,
	PULA,
	PULX,
	PULY,
	PULCC,
	ROLA,
	ROL(RolAddr),
	RORA,
	ROR(RorAddr),
	RTS,
	RTI,
	SBCA(SbcaAddr),
	STA(StaAddr),
	STX(StxAddr),
	STY(StyAddr),
	STSP(StspAddr),
	SUBA(SubaAddr),
	TFR(TfrAddr),
	TSTA,
	TST(TstAddr),
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LdaAddr {
	Data,
	Addr,
	nSP,
	nX,
	AX,
	Xplus,
	Xminus,
	plusX,
	minusX,
	nY,
	AY,
	Yplus,
	Yminus,
	plusY,
	minusY,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum StaAddr {
	Addr,
	nSP,
	nX,
	AX,
	Xplus,
	Xminus,
	plusX,
	minusX,
	nY,
	AY,
	Yplus,
	Yminus,
	plusY,
	minusY,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum ExgAddr {
	XY,
	ACC,
	XSP,
	YSP,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LeaxAddr {
	nX,
	nSP,
}
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LeayAddr {
	nY,
	nSP,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LeaspAddr {
	nX,
	nY,
	nSP,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum StxAddr {
	Addr,
	nSP,
	nX,
	nY,
	AX,
	AY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum StyAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum StspAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum AdcaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum DecAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum IncAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum EoraAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum AddaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum SubaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum TstAddr {
	Addr,
	nSP,
	nX,
	nY,
	AX,
	AY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum AndaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum BitaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum AslAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LsrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum ClrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum AsrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum CmpaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum CmpxAddr {
	Data,
	Addr,
	nSP,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum CmpyAddr {
	Data,
	Addr,
	nSP,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum CmpspAddr {
	Data,
	Addr,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum ComAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum JmpAddr {
	Addr,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum JsrAddr {
	Addr,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LdxAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LdyAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum LdspAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum NegAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum OraAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum RolAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum RorAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum SbcaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum TfrAddr {
	ACC,
	CCA,
	XY,
	YX,
	XSP,
	SPX,
	YSP,
	SPY,
}

impl TryFrom<u8> for Instruction {
	type Error = FlispError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		let res = match value {
			0x95 => Instruction::ADCA(AdcaAddr::Data),
			0xA5 => Instruction::ADCA(AdcaAddr::Addr),
			0xB5 => Instruction::ADCA(AdcaAddr::nSP),
			0xC5 => Instruction::ADCA(AdcaAddr::nX),
			0xD5 => Instruction::ADCA(AdcaAddr::nY),

			0x96 => Instruction::ADDA(AddaAddr::Data),
			0xA6 => Instruction::ADDA(AddaAddr::Addr),
			0xB6 => Instruction::ADDA(AddaAddr::nSP),
			0xC6 => Instruction::ADDA(AddaAddr::nX),
			0xD6 => Instruction::ADDA(AddaAddr::nY),

			0x99 => Instruction::ANDA(AndaAddr::Data),
			0xA9 => Instruction::ANDA(AndaAddr::Addr),
			0xB9 => Instruction::ANDA(AndaAddr::nSP),
			0xC9 => Instruction::ANDA(AndaAddr::nX),
			0xD9 => Instruction::ANDA(AndaAddr::nY),

			0x01 => Instruction::ANDCC,

			0x0B => Instruction::ASLA,

			0x3B => Instruction::ASL(AslAddr::Addr),
			0x4B => Instruction::ASL(AslAddr::nSP),
			0x5B => Instruction::ASL(AslAddr::nX),
			0x6B => Instruction::ASL(AslAddr::AX),
			0x7B => Instruction::ASL(AslAddr::nY),
			0x8B => Instruction::ASL(AslAddr::AY),

			0x0F => Instruction::ASRA,

			0x3F => Instruction::ASR(AsrAddr::Addr),
			0x4F => Instruction::ASR(AsrAddr::nSP),
			0x5F => Instruction::ASR(AsrAddr::nX),
			0x6F => Instruction::ASR(AsrAddr::AX),
			0x7F => Instruction::ASR(AsrAddr::nY),
			0x8F => Instruction::ASR(AsrAddr::AY),

			0x29 => Instruction::BCC,
			0x28 => Instruction::BCS,
			0x24 => Instruction::BEQ,
			0x2D => Instruction::BGE,
			0x2C => Instruction::BGT,
			0x2A => Instruction::BHI,

			0x98 => Instruction::BITA(BitaAddr::Data),
			0xA8 => Instruction::BITA(BitaAddr::Addr),
			0xB8 => Instruction::BITA(BitaAddr::nSP),
			0xC8 => Instruction::BITA(BitaAddr::nX),
			0xD8 => Instruction::BITA(BitaAddr::nY),

			0x2E => Instruction::BLE,
			0x2B => Instruction::BLS,
			0x2F => Instruction::BLT,
			0x22 => Instruction::BMI,
			0x25 => Instruction::BNE,
			0x23 => Instruction::BPL,
			0x21 => Instruction::BRA,
			0x20 => Instruction::BSR,
			0x27 => Instruction::BVC,
			0x26 => Instruction::BVS,

			0x05 => Instruction::CLRA,

			0x35 => Instruction::CLR(ClrAddr::Addr),
			0x45 => Instruction::CLR(ClrAddr::nSP),
			0x55 => Instruction::CLR(ClrAddr::nX),
			0x65 => Instruction::CLR(ClrAddr::AX),
			0x75 => Instruction::CLR(ClrAddr::nY),
			0x85 => Instruction::CLR(ClrAddr::AY),

			0x97 => Instruction::CMPA(CmpaAddr::Data),
			0xA7 => Instruction::CMPA(CmpaAddr::Addr),
			0xB7 => Instruction::CMPA(CmpaAddr::nSP),
			0xC7 => Instruction::CMPA(CmpaAddr::nX),
			0xD7 => Instruction::CMPA(CmpaAddr::nY),

			0x9C => Instruction::CMPX(CmpxAddr::Data),
			0xAC => Instruction::CMPX(CmpxAddr::Addr),
			0xBC => Instruction::CMPX(CmpxAddr::nSP),

			0x9D => Instruction::CMPY(CmpyAddr::Data),
			0xAD => Instruction::CMPY(CmpyAddr::Addr),
			0xBD => Instruction::CMPY(CmpyAddr::nSP),

			0x9E => Instruction::CMPSP(CmpspAddr::Data),
			0xAE => Instruction::CMPSP(CmpspAddr::Addr),

			0x0A => Instruction::COMA,

			0x3A => Instruction::COM(ComAddr::Addr),
			0x4A => Instruction::COM(ComAddr::nSP),
			0x5A => Instruction::COM(ComAddr::nX),
			0x6A => Instruction::COM(ComAddr::AX),
			0x7A => Instruction::COM(ComAddr::nY),
			0x8A => Instruction::COM(ComAddr::AY),

			0x08 => Instruction::DECA,

			0x38 => Instruction::DEC(DecAddr::Addr),
			0x48 => Instruction::DEC(DecAddr::nSP),
			0x58 => Instruction::DEC(DecAddr::nX),
			0x68 => Instruction::DEC(DecAddr::AX),
			0x78 => Instruction::DEC(DecAddr::nY),
			0x88 => Instruction::DEC(DecAddr::AY),

			0x9B => Instruction::EORA(EoraAddr::Data),
			0xAB => Instruction::EORA(EoraAddr::Addr),
			0xBB => Instruction::EORA(EoraAddr::nSP),
			0xCB => Instruction::EORA(EoraAddr::nX),
			0xDB => Instruction::EORA(EoraAddr::nY),

			0x9F => Instruction::EXG(ExgAddr::ACC),
			0xAF => Instruction::EXG(ExgAddr::XY),
			0xBF => Instruction::EXG(ExgAddr::XSP),
			0xCF => Instruction::EXG(ExgAddr::YSP),

			0x07 => Instruction::INCA,

			0x37 => Instruction::INC(IncAddr::Addr),
			0x47 => Instruction::INC(IncAddr::nSP),
			0x57 => Instruction::INC(IncAddr::nX),
			0x67 => Instruction::INC(IncAddr::AX),
			0x77 => Instruction::INC(IncAddr::nY),
			0x87 => Instruction::INC(IncAddr::AY),

			0x33 => Instruction::JMP(JmpAddr::Addr),
			0x53 => Instruction::JMP(JmpAddr::nX),
			0x63 => Instruction::JMP(JmpAddr::AX),
			0x73 => Instruction::JMP(JmpAddr::nY),
			0x83 => Instruction::JMP(JmpAddr::AY),

			0x34 => Instruction::JSR(JsrAddr::Addr),
			0x54 => Instruction::JSR(JsrAddr::nX),
			0x64 => Instruction::JSR(JsrAddr::AX),
			0x74 => Instruction::JSR(JsrAddr::nY),
			0x84 => Instruction::JSR(JsrAddr::AY),

			0xF0 => Instruction::LDA(LdaAddr::Data),
			0xF1 => Instruction::LDA(LdaAddr::Addr),
			0xF2 => Instruction::LDA(LdaAddr::nSP),
			0xF3 => Instruction::LDA(LdaAddr::nX),
			0xF4 => Instruction::LDA(LdaAddr::AX),
			0xF5 => Instruction::LDA(LdaAddr::Xplus),
			0xF6 => Instruction::LDA(LdaAddr::Xminus),
			0xF7 => Instruction::LDA(LdaAddr::plusX),
			0xF8 => Instruction::LDA(LdaAddr::minusX),
			0xF9 => Instruction::LDA(LdaAddr::nY),
			0xFA => Instruction::LDA(LdaAddr::AY),
			0xFB => Instruction::LDA(LdaAddr::Yplus),
			0xFC => Instruction::LDA(LdaAddr::Yminus),
			0xFD => Instruction::LDA(LdaAddr::plusY),
			0xFE => Instruction::LDA(LdaAddr::minusY),

			0x90 => Instruction::LDX(LdxAddr::Data),
			0xA0 => Instruction::LDX(LdxAddr::Addr),
			0xB0 => Instruction::LDX(LdxAddr::nSP),
			0xC0 => Instruction::LDX(LdxAddr::nX),
			0xD0 => Instruction::LDX(LdxAddr::nY),

			0x91 => Instruction::LDY(LdyAddr::Data),
			0xA1 => Instruction::LDY(LdyAddr::Addr),
			0xB1 => Instruction::LDY(LdyAddr::nSP),
			0xC1 => Instruction::LDY(LdyAddr::nX),
			0xD1 => Instruction::LDY(LdyAddr::nY),

			0x92 => Instruction::LDSP(LdspAddr::Data),
			0xA2 => Instruction::LDSP(LdspAddr::Addr),
			0xB2 => Instruction::LDSP(LdspAddr::nSP),
			0xC2 => Instruction::LDSP(LdspAddr::nX),
			0xD2 => Instruction::LDSP(LdspAddr::nY),

			0xCC => Instruction::LEAX(LeaxAddr::nX),
			0xDC => Instruction::LEAX(LeaxAddr::nSP),

			0xCD => Instruction::LEAY(LeayAddr::nY),
			0xDD => Instruction::LEAY(LeayAddr::nSP),

			0xBE => Instruction::LEASP(LeaspAddr::nSP),
			0xCE => Instruction::LEASP(LeaspAddr::nX),
			0xDE => Instruction::LEASP(LeaspAddr::nY),

			0x0C => Instruction::LSRA,

			0x3C => Instruction::LSR(LsrAddr::Addr),
			0x4C => Instruction::LSR(LsrAddr::nSP),
			0x5C => Instruction::LSR(LsrAddr::nX),
			0x6C => Instruction::LSR(LsrAddr::AX),
			0x7C => Instruction::LSR(LsrAddr::nY),
			0x8C => Instruction::LSR(LsrAddr::AY),

			0x06 => Instruction::NEGA,

			0x36 => Instruction::NEG(NegAddr::Addr),
			0x46 => Instruction::NEG(NegAddr::nSP),
			0x56 => Instruction::NEG(NegAddr::nX),
			0x66 => Instruction::NEG(NegAddr::AX),
			0x76 => Instruction::NEG(NegAddr::nY),
			0x86 => Instruction::NEG(NegAddr::AY),

			0x00 => Instruction::NOP,

			0x9A => Instruction::ORA(OraAddr::Data),
			0xAA => Instruction::ORA(OraAddr::Addr),
			0xBA => Instruction::ORA(OraAddr::nSP),
			0xCA => Instruction::ORA(OraAddr::nX),
			0xDA => Instruction::ORA(OraAddr::nY),

			0x02 => Instruction::ORCC,

			0x10 => Instruction::PSHA,
			0x11 => Instruction::PSHX,
			0x12 => Instruction::PSHY,
			0x13 => Instruction::PSHCC,

			0x14 => Instruction::PULA,
			0x15 => Instruction::PULX,
			0x16 => Instruction::PULY,
			0x17 => Instruction::PULCC,

			0x0D => Instruction::ROLA,

			0x3D => Instruction::ROL(RolAddr::Addr),
			0x4D => Instruction::ROL(RolAddr::nSP),
			0x5D => Instruction::ROL(RolAddr::nX),
			0x6D => Instruction::ROL(RolAddr::AX),
			0x7D => Instruction::ROL(RolAddr::nY),
			0x8D => Instruction::ROL(RolAddr::AY),

			0x0E => Instruction::RORA,

			0x3E => Instruction::ROR(RorAddr::Addr),
			0x4E => Instruction::ROR(RorAddr::nSP),
			0x5E => Instruction::ROR(RorAddr::nX),
			0x6E => Instruction::ROR(RorAddr::AX),
			0x7E => Instruction::ROR(RorAddr::nY),
			0x8E => Instruction::ROR(RorAddr::AY),

			0x43 => Instruction::RTS,
			0x44 => Instruction::RTI,

			0x93 => Instruction::SBCA(SbcaAddr::Data),
			0xA3 => Instruction::SBCA(SbcaAddr::Addr),
			0xB3 => Instruction::SBCA(SbcaAddr::nSP),
			0xC3 => Instruction::SBCA(SbcaAddr::nX),
			0xD3 => Instruction::SBCA(SbcaAddr::nY),

			0xE1 => Instruction::STA(StaAddr::Addr),
			0xE2 => Instruction::STA(StaAddr::nSP),
			0xE3 => Instruction::STA(StaAddr::nX),
			0xE4 => Instruction::STA(StaAddr::AX),
			0xE5 => Instruction::STA(StaAddr::Xplus),
			0xE6 => Instruction::STA(StaAddr::Xminus),
			0xE7 => Instruction::STA(StaAddr::plusX),
			0xE8 => Instruction::STA(StaAddr::minusX),
			0xE9 => Instruction::STA(StaAddr::nY),
			0xEA => Instruction::STA(StaAddr::AY),
			0xEB => Instruction::STA(StaAddr::Yplus),
			0xEC => Instruction::STA(StaAddr::Yminus),
			0xED => Instruction::STA(StaAddr::plusY),
			0xEE => Instruction::STA(StaAddr::minusY),

			0x30 => Instruction::STX(StxAddr::Addr),
			0x40 => Instruction::STX(StxAddr::nSP),
			0x50 => Instruction::STX(StxAddr::nX),
			0x60 => Instruction::STX(StxAddr::AX),
			0x70 => Instruction::STX(StxAddr::nY),
			0x80 => Instruction::STX(StxAddr::AY),

			0x31 => Instruction::STY(StyAddr::Addr),
			0x41 => Instruction::STY(StyAddr::nSP),
			0x51 => Instruction::STY(StyAddr::nX),
			0x61 => Instruction::STY(StyAddr::AX),
			0x71 => Instruction::STY(StyAddr::nY),
			0x81 => Instruction::STY(StyAddr::AY),

			0x32 => Instruction::STSP(StspAddr::Addr),
			0x42 => Instruction::STSP(StspAddr::nSP),
			0x52 => Instruction::STSP(StspAddr::nX),
			0x62 => Instruction::STSP(StspAddr::AX),
			0x72 => Instruction::STSP(StspAddr::nY),
			0x82 => Instruction::STSP(StspAddr::AY),

			0x94 => Instruction::SUBA(SubaAddr::Data),
			0xA4 => Instruction::SUBA(SubaAddr::Addr),
			0xB4 => Instruction::SUBA(SubaAddr::nSP),
			0xC4 => Instruction::SUBA(SubaAddr::nX),
			0xD4 => Instruction::SUBA(SubaAddr::nY),

			0x18 => Instruction::TFR(TfrAddr::ACC),
			0x19 => Instruction::TFR(TfrAddr::CCA),
			0x1A => Instruction::TFR(TfrAddr::XY),
			0x1B => Instruction::TFR(TfrAddr::YX),
			0x1C => Instruction::TFR(TfrAddr::XSP),
			0x1D => Instruction::TFR(TfrAddr::SPX),
			0x1E => Instruction::TFR(TfrAddr::YSP),
			0x1F => Instruction::TFR(TfrAddr::SPY),

			0x09 => Instruction::TSTA,

			0x39 => Instruction::TST(TstAddr::Addr),
			0x49 => Instruction::TST(TstAddr::nSP),
			0x59 => Instruction::TST(TstAddr::nX),
			0x69 => Instruction::TST(TstAddr::AX),
			0x79 => Instruction::TST(TstAddr::nY),
			0x89 => Instruction::TST(TstAddr::AY),

			_ => return Err(FlispError::InvalidOpCode),
		};
		Ok(res)
	}
}
