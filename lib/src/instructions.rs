use std::convert::TryFrom;

use crate::*;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
	ADCA(AddrTypeTwo),
	ADDA(AddrTypeTwo),
	ANDA(AddrTypeTwo),
	ANDCC,
	ASLA,
	ASL(AddrTypeThree),
	ASRA,
	ASR(AddrTypeThree),
	BITA(AddrTypeTwo),
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
	CLR(AddrTypeThree),
	CMPA(AddrTypeTwo),
	CMPX(AddrTypeFour),
	CMPY(AddrTypeFour),
	CMPSP(CmpspAddr),
	COMA,
	COM(AddrTypeThree),
	DECA,
	DEC(AddrTypeThree),
	EORA(AddrTypeTwo),
	EXG(ExgAddr),
	INCA,
	INC(AddrTypeThree),
	JMP(AddrTypeFive),
	JSR(AddrTypeFive),
	LDA(LdaAddr),
	LDX(AddrTypeTwo),
	LDY(AddrTypeTwo),
	LDSP(AddrTypeTwo),
	LEAX(LeaxAddr),
	LEAY(LeayAddr),
	LEASP(LeaspAddr),
	LSRA,
	LSR(AddrTypeThree),
	NEGA,
	NEG(AddrTypeThree),
	NOP,
	ORA(AddrTypeTwo),
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
	ROL(AddrTypeThree),
	RORA,
	ROR(AddrTypeThree),
	RTS,
	RTI,
	SBCA(AddrTypeTwo),
	STA(StaAddr),
	STX(AddrTypeOne),
	STY(AddrTypeOne),
	STSP(AddrTypeOne),
	SUBA(AddrTypeTwo),
	TFR(TfrAddr),
	TSTA,
	TST(AddrTypeOne),
}

impl TryFrom<u8> for Instruction {
	type Error = FlispError;

	fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
		let res = match value {
			0x95 => Instruction::ADCA(AddrTypeTwo::Data),
			0xA5 => Instruction::ADCA(AddrTypeTwo::Addr),
			0xB5 => Instruction::ADCA(AddrTypeTwo::nSP),
			0xC5 => Instruction::ADCA(AddrTypeTwo::nX),
			0xD5 => Instruction::ADCA(AddrTypeTwo::nY),

			0x96 => Instruction::ADDA(AddrTypeTwo::Data),
			0xA6 => Instruction::ADDA(AddrTypeTwo::Addr),
			0xB6 => Instruction::ADDA(AddrTypeTwo::nSP),
			0xC6 => Instruction::ADDA(AddrTypeTwo::nX),
			0xD6 => Instruction::ADDA(AddrTypeTwo::nY),

			0x99 => Instruction::ANDA(AddrTypeTwo::Data),
			0xA9 => Instruction::ANDA(AddrTypeTwo::Addr),
			0xB9 => Instruction::ANDA(AddrTypeTwo::nSP),
			0xC9 => Instruction::ANDA(AddrTypeTwo::nX),
			0xD9 => Instruction::ANDA(AddrTypeTwo::nY),

			0x01 => Instruction::ANDCC,

			0x0B => Instruction::ASLA,

			0x3B => Instruction::ASL(AddrTypeThree::Addr),
			0x4B => Instruction::ASL(AddrTypeThree::nSP),
			0x5B => Instruction::ASL(AddrTypeThree::nX),
			0x6B => Instruction::ASL(AddrTypeThree::AX),
			0x7B => Instruction::ASL(AddrTypeThree::nY),
			0x8B => Instruction::ASL(AddrTypeThree::AY),

			0x0F => Instruction::ASRA,

			0x3F => Instruction::ASR(AddrTypeThree::Addr),
			0x4F => Instruction::ASR(AddrTypeThree::nSP),
			0x5F => Instruction::ASR(AddrTypeThree::nX),
			0x6F => Instruction::ASR(AddrTypeThree::AX),
			0x7F => Instruction::ASR(AddrTypeThree::nY),
			0x8F => Instruction::ASR(AddrTypeThree::AY),

			0x29 => Instruction::BCC,
			0x28 => Instruction::BCS,
			0x24 => Instruction::BEQ,
			0x2D => Instruction::BGE,
			0x2C => Instruction::BGT,
			0x2A => Instruction::BHI,

			0x98 => Instruction::BITA(AddrTypeTwo::Data),
			0xA8 => Instruction::BITA(AddrTypeTwo::Addr),
			0xB8 => Instruction::BITA(AddrTypeTwo::nSP),
			0xC8 => Instruction::BITA(AddrTypeTwo::nX),
			0xD8 => Instruction::BITA(AddrTypeTwo::nY),

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

			0x35 => Instruction::CLR(AddrTypeThree::Addr),
			0x45 => Instruction::CLR(AddrTypeThree::nSP),
			0x55 => Instruction::CLR(AddrTypeThree::nX),
			0x65 => Instruction::CLR(AddrTypeThree::AX),
			0x75 => Instruction::CLR(AddrTypeThree::nY),
			0x85 => Instruction::CLR(AddrTypeThree::AY),

			0x97 => Instruction::CMPA(AddrTypeTwo::Data),
			0xA7 => Instruction::CMPA(AddrTypeTwo::Addr),
			0xB7 => Instruction::CMPA(AddrTypeTwo::nSP),
			0xC7 => Instruction::CMPA(AddrTypeTwo::nX),
			0xD7 => Instruction::CMPA(AddrTypeTwo::nY),

			0x9C => Instruction::CMPX(AddrTypeFour::Data),
			0xAC => Instruction::CMPX(AddrTypeFour::Addr),
			0xBC => Instruction::CMPX(AddrTypeFour::nSP),

			0x9D => Instruction::CMPY(AddrTypeFour::Data),
			0xAD => Instruction::CMPY(AddrTypeFour::Addr),
			0xBD => Instruction::CMPY(AddrTypeFour::nSP),

			0x9E => Instruction::CMPSP(CmpspAddr::Data),
			0xAE => Instruction::CMPSP(CmpspAddr::Addr),

			0x0A => Instruction::COMA,

			0x3A => Instruction::COM(AddrTypeThree::Addr),
			0x4A => Instruction::COM(AddrTypeThree::nSP),
			0x5A => Instruction::COM(AddrTypeThree::nX),
			0x6A => Instruction::COM(AddrTypeThree::AX),
			0x7A => Instruction::COM(AddrTypeThree::nY),
			0x8A => Instruction::COM(AddrTypeThree::AY),

			0x08 => Instruction::DECA,

			0x38 => Instruction::DEC(AddrTypeThree::Addr),
			0x48 => Instruction::DEC(AddrTypeThree::nSP),
			0x58 => Instruction::DEC(AddrTypeThree::nX),
			0x68 => Instruction::DEC(AddrTypeThree::AX),
			0x78 => Instruction::DEC(AddrTypeThree::nY),
			0x88 => Instruction::DEC(AddrTypeThree::AY),

			0x9B => Instruction::EORA(AddrTypeTwo::Data),
			0xAB => Instruction::EORA(AddrTypeTwo::Addr),
			0xBB => Instruction::EORA(AddrTypeTwo::nSP),
			0xCB => Instruction::EORA(AddrTypeTwo::nX),
			0xDB => Instruction::EORA(AddrTypeTwo::nY),

			0x9F => Instruction::EXG(ExgAddr::ACC),
			0xAF => Instruction::EXG(ExgAddr::XY),
			0xBF => Instruction::EXG(ExgAddr::XSP),
			0xCF => Instruction::EXG(ExgAddr::YSP),

			0x07 => Instruction::INCA,

			0x37 => Instruction::INC(AddrTypeThree::Addr),
			0x47 => Instruction::INC(AddrTypeThree::nSP),
			0x57 => Instruction::INC(AddrTypeThree::nX),
			0x67 => Instruction::INC(AddrTypeThree::AX),
			0x77 => Instruction::INC(AddrTypeThree::nY),
			0x87 => Instruction::INC(AddrTypeThree::AY),

			0x33 => Instruction::JMP(AddrTypeFive::Addr),
			0x53 => Instruction::JMP(AddrTypeFive::nX),
			0x63 => Instruction::JMP(AddrTypeFive::AX),
			0x73 => Instruction::JMP(AddrTypeFive::nY),
			0x83 => Instruction::JMP(AddrTypeFive::AY),

			0x34 => Instruction::JSR(AddrTypeFive::Addr),
			0x54 => Instruction::JSR(AddrTypeFive::nX),
			0x64 => Instruction::JSR(AddrTypeFive::AX),
			0x74 => Instruction::JSR(AddrTypeFive::nY),
			0x84 => Instruction::JSR(AddrTypeFive::AY),

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

			0x90 => Instruction::LDX(AddrTypeTwo::Data),
			0xA0 => Instruction::LDX(AddrTypeTwo::Addr),
			0xB0 => Instruction::LDX(AddrTypeTwo::nSP),
			0xC0 => Instruction::LDX(AddrTypeTwo::nX),
			0xD0 => Instruction::LDX(AddrTypeTwo::nY),

			0x91 => Instruction::LDY(AddrTypeTwo::Data),
			0xA1 => Instruction::LDY(AddrTypeTwo::Addr),
			0xB1 => Instruction::LDY(AddrTypeTwo::nSP),
			0xC1 => Instruction::LDY(AddrTypeTwo::nX),
			0xD1 => Instruction::LDY(AddrTypeTwo::nY),

			0x92 => Instruction::LDSP(AddrTypeTwo::Data),
			0xA2 => Instruction::LDSP(AddrTypeTwo::Addr),
			0xB2 => Instruction::LDSP(AddrTypeTwo::nSP),
			0xC2 => Instruction::LDSP(AddrTypeTwo::nX),
			0xD2 => Instruction::LDSP(AddrTypeTwo::nY),

			0xCC => Instruction::LEAX(LeaxAddr::nX),
			0xDC => Instruction::LEAX(LeaxAddr::nSP),

			0xCD => Instruction::LEAY(LeayAddr::nY),
			0xDD => Instruction::LEAY(LeayAddr::nSP),

			0xBE => Instruction::LEASP(LeaspAddr::nSP),
			0xCE => Instruction::LEASP(LeaspAddr::nX),
			0xDE => Instruction::LEASP(LeaspAddr::nY),

			0x0C => Instruction::LSRA,

			0x3C => Instruction::LSR(AddrTypeThree::Addr),
			0x4C => Instruction::LSR(AddrTypeThree::nSP),
			0x5C => Instruction::LSR(AddrTypeThree::nX),
			0x6C => Instruction::LSR(AddrTypeThree::AX),
			0x7C => Instruction::LSR(AddrTypeThree::nY),
			0x8C => Instruction::LSR(AddrTypeThree::AY),

			0x06 => Instruction::NEGA,

			0x36 => Instruction::NEG(AddrTypeThree::Addr),
			0x46 => Instruction::NEG(AddrTypeThree::nSP),
			0x56 => Instruction::NEG(AddrTypeThree::nX),
			0x66 => Instruction::NEG(AddrTypeThree::AX),
			0x76 => Instruction::NEG(AddrTypeThree::nY),
			0x86 => Instruction::NEG(AddrTypeThree::AY),

			0x00 => Instruction::NOP,

			0x9A => Instruction::ORA(AddrTypeTwo::Data),
			0xAA => Instruction::ORA(AddrTypeTwo::Addr),
			0xBA => Instruction::ORA(AddrTypeTwo::nSP),
			0xCA => Instruction::ORA(AddrTypeTwo::nX),
			0xDA => Instruction::ORA(AddrTypeTwo::nY),

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

			0x3D => Instruction::ROL(AddrTypeThree::Addr),
			0x4D => Instruction::ROL(AddrTypeThree::nSP),
			0x5D => Instruction::ROL(AddrTypeThree::nX),
			0x6D => Instruction::ROL(AddrTypeThree::AX),
			0x7D => Instruction::ROL(AddrTypeThree::nY),
			0x8D => Instruction::ROL(AddrTypeThree::AY),

			0x0E => Instruction::RORA,

			0x3E => Instruction::ROR(AddrTypeThree::Addr),
			0x4E => Instruction::ROR(AddrTypeThree::nSP),
			0x5E => Instruction::ROR(AddrTypeThree::nX),
			0x6E => Instruction::ROR(AddrTypeThree::AX),
			0x7E => Instruction::ROR(AddrTypeThree::nY),
			0x8E => Instruction::ROR(AddrTypeThree::AY),

			0x43 => Instruction::RTS,
			0x44 => Instruction::RTI,

			0x93 => Instruction::SBCA(AddrTypeTwo::Data),
			0xA3 => Instruction::SBCA(AddrTypeTwo::Addr),
			0xB3 => Instruction::SBCA(AddrTypeTwo::nSP),
			0xC3 => Instruction::SBCA(AddrTypeTwo::nX),
			0xD3 => Instruction::SBCA(AddrTypeTwo::nY),

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

			0x30 => Instruction::STX(AddrTypeOne::Addr),
			0x40 => Instruction::STX(AddrTypeOne::nSP),
			0x50 => Instruction::STX(AddrTypeOne::nX),
			0x60 => Instruction::STX(AddrTypeOne::AX),
			0x70 => Instruction::STX(AddrTypeOne::nY),
			0x80 => Instruction::STX(AddrTypeOne::AY),

			0x31 => Instruction::STY(AddrTypeOne::Addr),
			0x41 => Instruction::STY(AddrTypeOne::nSP),
			0x51 => Instruction::STY(AddrTypeOne::nX),
			0x61 => Instruction::STY(AddrTypeOne::AX),
			0x71 => Instruction::STY(AddrTypeOne::nY),
			0x81 => Instruction::STY(AddrTypeOne::AY),

			0x32 => Instruction::STSP(AddrTypeOne::Addr),
			0x42 => Instruction::STSP(AddrTypeOne::nSP),
			0x52 => Instruction::STSP(AddrTypeOne::nX),
			0x62 => Instruction::STSP(AddrTypeOne::AX),
			0x72 => Instruction::STSP(AddrTypeOne::nY),
			0x82 => Instruction::STSP(AddrTypeOne::AY),

			0x94 => Instruction::SUBA(AddrTypeTwo::Data),
			0xA4 => Instruction::SUBA(AddrTypeTwo::Addr),
			0xB4 => Instruction::SUBA(AddrTypeTwo::nSP),
			0xC4 => Instruction::SUBA(AddrTypeTwo::nX),
			0xD4 => Instruction::SUBA(AddrTypeTwo::nY),

			0x18 => Instruction::TFR(TfrAddr::ACC),
			0x19 => Instruction::TFR(TfrAddr::CCA),
			0x1A => Instruction::TFR(TfrAddr::XY),
			0x1B => Instruction::TFR(TfrAddr::YX),
			0x1C => Instruction::TFR(TfrAddr::XSP),
			0x1D => Instruction::TFR(TfrAddr::SPX),
			0x1E => Instruction::TFR(TfrAddr::YSP),
			0x1F => Instruction::TFR(TfrAddr::SPY),

			0x09 => Instruction::TSTA,

			0x39 => Instruction::TST(AddrTypeOne::Addr),
			0x49 => Instruction::TST(AddrTypeOne::nSP),
			0x59 => Instruction::TST(AddrTypeOne::nX),
			0x69 => Instruction::TST(AddrTypeOne::AX),
			0x79 => Instruction::TST(AddrTypeOne::nY),
			0x89 => Instruction::TST(AddrTypeOne::AY),

			_ => {
				return Err(FlispError::InvalidOpCode(line!()));
			}
		};
		Ok(res)
	}
}

impl From<Instruction> for u8 {
	fn from(i: Instruction) -> Self {
		match i {
			Instruction::ADCA(AddrTypeTwo::Data) => 0x95,
			Instruction::ADCA(AddrTypeTwo::Addr) => 0xA5,
			Instruction::ADCA(AddrTypeTwo::nSP) => 0xB5,
			Instruction::ADCA(AddrTypeTwo::nX) => 0xC5,
			Instruction::ADCA(AddrTypeTwo::nY) => 0xD5,

			Instruction::ADDA(AddrTypeTwo::Data) => 0x96,
			Instruction::ADDA(AddrTypeTwo::Addr) => 0xA6,
			Instruction::ADDA(AddrTypeTwo::nSP) => 0xB6,
			Instruction::ADDA(AddrTypeTwo::nX) => 0xC6,
			Instruction::ADDA(AddrTypeTwo::nY) => 0xE6,

			Instruction::ANDA(AddrTypeTwo::Data) => 0x99,
			Instruction::ANDA(AddrTypeTwo::Addr) => 0xA9,
			Instruction::ANDA(AddrTypeTwo::nSP) => 0xB9,
			Instruction::ANDA(AddrTypeTwo::nX) => 0xC9,
			Instruction::ANDA(AddrTypeTwo::nY) => 0xD9,

			Instruction::ANDCC => 0x01,

			Instruction::ASLA => 0x0B,

			Instruction::ASL(AddrTypeThree::Addr) => 0x3B,
			Instruction::ASL(AddrTypeThree::nSP) => 0x4B,
			Instruction::ASL(AddrTypeThree::nX) => 0x5B,
			Instruction::ASL(AddrTypeThree::AX) => 0x6B,
			Instruction::ASL(AddrTypeThree::nY) => 0x7B,
			Instruction::ASL(AddrTypeThree::AY) => 0x8B,

			Instruction::ASRA => 0x0F,

			Instruction::ASR(AddrTypeThree::Addr) => 0x3F,
			Instruction::ASR(AddrTypeThree::nSP) => 0x4F,
			Instruction::ASR(AddrTypeThree::nX) => 0x5F,
			Instruction::ASR(AddrTypeThree::AX) => 0x6F,
			Instruction::ASR(AddrTypeThree::nY) => 0x7F,
			Instruction::ASR(AddrTypeThree::AY) => 0x8F,

			Instruction::BCC => 0x29,
			Instruction::BCS => 0x28,
			Instruction::BEQ => 0x24,
			Instruction::BGE => 0x2D,
			Instruction::BGT => 0x2C,
			Instruction::BHI => 0x2A,

			Instruction::BITA(AddrTypeTwo::Data) => 0x98,
			Instruction::BITA(AddrTypeTwo::Addr) => 0xA8,
			Instruction::BITA(AddrTypeTwo::nSP) => 0xB8,
			Instruction::BITA(AddrTypeTwo::nX) => 0xC8,
			Instruction::BITA(AddrTypeTwo::nY) => 0xD8,

			Instruction::BLE => 0x2E,
			Instruction::BLS => 0x2B,
			Instruction::BLT => 0x2F,
			Instruction::BMI => 0x22,
			Instruction::BNE => 0x25,
			Instruction::BPL => 0x23,
			Instruction::BRA => 0x21,
			Instruction::BSR => 0x20,
			Instruction::BVC => 0x27,
			Instruction::BVS => 0x26,

			Instruction::CLRA => 0x05,

			Instruction::CLR(AddrTypeThree::Addr) => 0x35,
			Instruction::CLR(AddrTypeThree::nSP) => 0x45,
			Instruction::CLR(AddrTypeThree::nX) => 0x55,
			Instruction::CLR(AddrTypeThree::AX) => 0x65,
			Instruction::CLR(AddrTypeThree::nY) => 0x75,
			Instruction::CLR(AddrTypeThree::AY) => 0x85,

			Instruction::CMPA(AddrTypeTwo::Data) => 0x97,
			Instruction::CMPA(AddrTypeTwo::Addr) => 0xA7,
			Instruction::CMPA(AddrTypeTwo::nSP) => 0xB7,
			Instruction::CMPA(AddrTypeTwo::nX) => 0xC7,
			Instruction::CMPA(AddrTypeTwo::nY) => 0xD7,

			Instruction::CMPX(AddrTypeFour::Data) => 0x9C,
			Instruction::CMPX(AddrTypeFour::Addr) => 0xAC,
			Instruction::CMPX(AddrTypeFour::nSP) => 0xBC,

			Instruction::CMPY(AddrTypeFour::Data) => 0x9D,
			Instruction::CMPY(AddrTypeFour::Addr) => 0xAD,
			Instruction::CMPY(AddrTypeFour::nSP) => 0xBD,

			Instruction::CMPSP(CmpspAddr::Data) => 0x9E,
			Instruction::CMPSP(CmpspAddr::Addr) => 0xAE,

			Instruction::COMA => 0x0A,

			Instruction::COM(AddrTypeThree::Addr) => 0x3A,
			Instruction::COM(AddrTypeThree::nSP) => 0x4A,
			Instruction::COM(AddrTypeThree::nX) => 0x5A,
			Instruction::COM(AddrTypeThree::AX) => 0x6A,
			Instruction::COM(AddrTypeThree::nY) => 0x7A,
			Instruction::COM(AddrTypeThree::AY) => 0x8A,

			Instruction::DECA => 0x08,

			Instruction::DEC(AddrTypeThree::Addr) => 0x38,
			Instruction::DEC(AddrTypeThree::nSP) => 0x48,
			Instruction::DEC(AddrTypeThree::nX) => 0x58,
			Instruction::DEC(AddrTypeThree::AX) => 0x68,
			Instruction::DEC(AddrTypeThree::nY) => 0x78,
			Instruction::DEC(AddrTypeThree::AY) => 0x88,

			Instruction::EORA(AddrTypeTwo::Data) => 0x9B,
			Instruction::EORA(AddrTypeTwo::Addr) => 0xAB,
			Instruction::EORA(AddrTypeTwo::nSP) => 0xBB,
			Instruction::EORA(AddrTypeTwo::nX) => 0xCB,
			Instruction::EORA(AddrTypeTwo::nY) => 0xDB,

			Instruction::EXG(ExgAddr::ACC) => 0x9F,
			Instruction::EXG(ExgAddr::XY) => 0xAF,
			Instruction::EXG(ExgAddr::XSP) => 0xBF,
			Instruction::EXG(ExgAddr::YSP) => 0xCF,

			Instruction::INCA => 0x07,

			Instruction::INC(AddrTypeThree::Addr) => 0x37,
			Instruction::INC(AddrTypeThree::nSP) => 0x47,
			Instruction::INC(AddrTypeThree::nX) => 0x57,
			Instruction::INC(AddrTypeThree::AX) => 0x67,
			Instruction::INC(AddrTypeThree::nY) => 0x77,
			Instruction::INC(AddrTypeThree::AY) => 0x87,

			Instruction::JMP(AddrTypeFive::Addr) => 0x33,
			Instruction::JMP(AddrTypeFive::nX) => 0x53,
			Instruction::JMP(AddrTypeFive::AX) => 0x63,
			Instruction::JMP(AddrTypeFive::nY) => 0x73,
			Instruction::JMP(AddrTypeFive::AY) => 0x83,

			Instruction::JSR(AddrTypeFive::Addr) => 0x34,
			Instruction::JSR(AddrTypeFive::nX) => 0x54,
			Instruction::JSR(AddrTypeFive::AX) => 0x64,
			Instruction::JSR(AddrTypeFive::nY) => 0x74,
			Instruction::JSR(AddrTypeFive::AY) => 0x84,

			Instruction::LDA(LdaAddr::Data) => 0xF0,
			Instruction::LDA(LdaAddr::Addr) => 0xF1,
			Instruction::LDA(LdaAddr::nSP) => 0xF2,
			Instruction::LDA(LdaAddr::nX) => 0xF3,
			Instruction::LDA(LdaAddr::AX) => 0xF4,
			Instruction::LDA(LdaAddr::Xplus) => 0xF5,
			Instruction::LDA(LdaAddr::Xminus) => 0xF6,
			Instruction::LDA(LdaAddr::plusX) => 0xF7,
			Instruction::LDA(LdaAddr::minusX) => 0xF8,
			Instruction::LDA(LdaAddr::nY) => 0xF9,
			Instruction::LDA(LdaAddr::AY) => 0xFA,
			Instruction::LDA(LdaAddr::Yplus) => 0xFB,
			Instruction::LDA(LdaAddr::Yminus) => 0xFC,
			Instruction::LDA(LdaAddr::plusY) => 0xFD,
			Instruction::LDA(LdaAddr::minusY) => 0xFE,

			Instruction::LDX(AddrTypeTwo::Data) => 0x90,
			Instruction::LDX(AddrTypeTwo::Addr) => 0xA0,
			Instruction::LDX(AddrTypeTwo::nSP) => 0xB0,
			Instruction::LDX(AddrTypeTwo::nX) => 0xC0,
			Instruction::LDX(AddrTypeTwo::nY) => 0xD0,

			Instruction::LDY(AddrTypeTwo::Data) => 0x91,
			Instruction::LDY(AddrTypeTwo::Addr) => 0xA1,
			Instruction::LDY(AddrTypeTwo::nSP) => 0xB1,
			Instruction::LDY(AddrTypeTwo::nX) => 0xC1,
			Instruction::LDY(AddrTypeTwo::nY) => 0xD1,

			Instruction::LDSP(AddrTypeTwo::Data) => 0x92,
			Instruction::LDSP(AddrTypeTwo::Addr) => 0xA2,
			Instruction::LDSP(AddrTypeTwo::nSP) => 0xB2,
			Instruction::LDSP(AddrTypeTwo::nX) => 0xC2,
			Instruction::LDSP(AddrTypeTwo::nY) => 0xD2,

			Instruction::LEAX(LeaxAddr::nX) => 0xCC,
			Instruction::LEAX(LeaxAddr::nSP) => 0xDC,

			Instruction::LEAY(LeayAddr::nY) => 0xCD,
			Instruction::LEAY(LeayAddr::nSP) => 0xDD,

			Instruction::LEASP(LeaspAddr::nSP) => 0xBE,
			Instruction::LEASP(LeaspAddr::nX) => 0xCE,
			Instruction::LEASP(LeaspAddr::nY) => 0xDE,

			Instruction::LSRA => 0x0C,

			Instruction::LSR(AddrTypeThree::Addr) => 0x3C,
			Instruction::LSR(AddrTypeThree::nSP) => 0x4C,
			Instruction::LSR(AddrTypeThree::nX) => 0x5C,
			Instruction::LSR(AddrTypeThree::AX) => 0x6C,
			Instruction::LSR(AddrTypeThree::nY) => 0x7C,
			Instruction::LSR(AddrTypeThree::AY) => 0x8C,

			Instruction::NEGA => 0x06,

			Instruction::NEG(AddrTypeThree::Addr) => 0x36,
			Instruction::NEG(AddrTypeThree::nSP) => 0x46,
			Instruction::NEG(AddrTypeThree::nX) => 0x56,
			Instruction::NEG(AddrTypeThree::AX) => 0x66,
			Instruction::NEG(AddrTypeThree::nY) => 0x76,
			Instruction::NEG(AddrTypeThree::AY) => 0x86,

			Instruction::NOP => 0x00,

			Instruction::ORA(AddrTypeTwo::Data) => 0x9A,
			Instruction::ORA(AddrTypeTwo::Addr) => 0xAA,
			Instruction::ORA(AddrTypeTwo::nSP) => 0xBA,
			Instruction::ORA(AddrTypeTwo::nX) => 0xCA,
			Instruction::ORA(AddrTypeTwo::nY) => 0xDA,

			Instruction::ORCC => 0x02,

			Instruction::PSHA => 0x10,
			Instruction::PSHX => 0x11,
			Instruction::PSHY => 0x12,
			Instruction::PSHCC => 0x13,

			Instruction::PULA => 0x14,
			Instruction::PULX => 0x15,
			Instruction::PULY => 0x16,
			Instruction::PULCC => 0x17,

			Instruction::ROLA => 0x0D,

			Instruction::ROL(AddrTypeThree::Addr) => 0x3D,
			Instruction::ROL(AddrTypeThree::nSP) => 0x4D,
			Instruction::ROL(AddrTypeThree::nX) => 0x5D,
			Instruction::ROL(AddrTypeThree::AX) => 0x6D,
			Instruction::ROL(AddrTypeThree::nY) => 0x7D,
			Instruction::ROL(AddrTypeThree::AY) => 0x8D,

			Instruction::RORA => 0x0E,

			Instruction::ROR(AddrTypeThree::Addr) => 0x3E,
			Instruction::ROR(AddrTypeThree::nSP) => 0x4E,
			Instruction::ROR(AddrTypeThree::nX) => 0x5E,
			Instruction::ROR(AddrTypeThree::AX) => 0x6E,
			Instruction::ROR(AddrTypeThree::nY) => 0x7E,
			Instruction::ROR(AddrTypeThree::AY) => 0x8E,

			Instruction::RTS => 0x43,
			Instruction::RTI => 0x44,

			Instruction::SBCA(AddrTypeTwo::Data) => 0x93,
			Instruction::SBCA(AddrTypeTwo::Addr) => 0xA3,
			Instruction::SBCA(AddrTypeTwo::nSP) => 0xB3,
			Instruction::SBCA(AddrTypeTwo::nX) => 0xC3,
			Instruction::SBCA(AddrTypeTwo::nY) => 0xD3,

			Instruction::STA(StaAddr::Addr) => 0xE1,
			Instruction::STA(StaAddr::nSP) => 0xE2,
			Instruction::STA(StaAddr::nX) => 0xE3,
			Instruction::STA(StaAddr::AX) => 0xE4,
			Instruction::STA(StaAddr::Xplus) => 0xE5,
			Instruction::STA(StaAddr::Xminus) => 0xE6,
			Instruction::STA(StaAddr::plusX) => 0xE7,
			Instruction::STA(StaAddr::minusX) => 0xE8,
			Instruction::STA(StaAddr::nY) => 0xE9,
			Instruction::STA(StaAddr::AY) => 0xEA,
			Instruction::STA(StaAddr::Yplus) => 0xEB,
			Instruction::STA(StaAddr::Yminus) => 0xEC,
			Instruction::STA(StaAddr::plusY) => 0xED,
			Instruction::STA(StaAddr::minusY) => 0xEE,

			Instruction::STX(AddrTypeOne::Addr) => 0x30,
			Instruction::STX(AddrTypeOne::nSP) => 0x40,
			Instruction::STX(AddrTypeOne::nX) => 0x50,
			Instruction::STX(AddrTypeOne::AX) => 0x60,
			Instruction::STX(AddrTypeOne::nY) => 0x70,
			Instruction::STX(AddrTypeOne::AY) => 0x80,

			Instruction::STY(AddrTypeOne::Addr) => 0x31,
			Instruction::STY(AddrTypeOne::nSP) => 0x41,
			Instruction::STY(AddrTypeOne::nX) => 0x51,
			Instruction::STY(AddrTypeOne::AX) => 0x61,
			Instruction::STY(AddrTypeOne::nY) => 0x71,
			Instruction::STY(AddrTypeOne::AY) => 0x81,

			Instruction::STSP(AddrTypeOne::Addr) => 0x32,
			Instruction::STSP(AddrTypeOne::nSP) => 0x42,
			Instruction::STSP(AddrTypeOne::nX) => 0x52,
			Instruction::STSP(AddrTypeOne::AX) => 0x62,
			Instruction::STSP(AddrTypeOne::nY) => 0x72,
			Instruction::STSP(AddrTypeOne::AY) => 0x82,

			Instruction::SUBA(AddrTypeTwo::Data) => 0x94,
			Instruction::SUBA(AddrTypeTwo::Addr) => 0xA4,
			Instruction::SUBA(AddrTypeTwo::nSP) => 0xB4,
			Instruction::SUBA(AddrTypeTwo::nX) => 0xC4,
			Instruction::SUBA(AddrTypeTwo::nY) => 0xD4,

			Instruction::TFR(TfrAddr::ACC) => 0x18,
			Instruction::TFR(TfrAddr::CCA) => 0x19,
			Instruction::TFR(TfrAddr::XY) => 0x1A,
			Instruction::TFR(TfrAddr::YX) => 0x1B,
			Instruction::TFR(TfrAddr::XSP) => 0x1C,
			Instruction::TFR(TfrAddr::SPX) => 0x1D,
			Instruction::TFR(TfrAddr::YSP) => 0x1E,
			Instruction::TFR(TfrAddr::SPY) => 0x1F,

			Instruction::TSTA => 0x09,

			Instruction::TST(AddrTypeOne::Addr) => 0x39,
			Instruction::TST(AddrTypeOne::nSP) => 0x49,
			Instruction::TST(AddrTypeOne::nX) => 0x59,
			Instruction::TST(AddrTypeOne::AX) => 0x69,
			Instruction::TST(AddrTypeOne::nY) => 0x79,
			Instruction::TST(AddrTypeOne::AY) => 0x89,
		}
	}
}

impl Instruction {
	pub fn size(&self) -> u8 {
		match self {
			Instruction::BLE
			| Instruction::BLS
			| Instruction::BLT
			| Instruction::BMI
			| Instruction::BNE
			| Instruction::BPL
			| Instruction::BRA
			| Instruction::BSR
			| Instruction::BVC
			| Instruction::BVS
			| Instruction::BCC
			| Instruction::BCS
			| Instruction::BEQ
			| Instruction::BGE
			| Instruction::BGT
			| Instruction::BHI
			| Instruction::JMP(_)
			| Instruction::JSR(_) => 2,

			Instruction::RTS | Instruction::RTI => 1,

			Instruction::ADCA(_)
			| Instruction::ADDA(_)
			| Instruction::ANDA(_)
			| Instruction::ANDCC
			| Instruction::BITA(_)
			| Instruction::CMPA(_)
			| Instruction::CMPX(_)
			| Instruction::CMPY(_)
			| Instruction::CMPSP(_)
			| Instruction::EORA(_)
			| Instruction::LEAX(_)
			| Instruction::LEAY(_)
			| Instruction::LEASP(_)
			| Instruction::ORA(_)
			| Instruction::ORCC
			| Instruction::SBCA(_)
			| Instruction::SUBA(_)
			| Instruction::LDX(_)
			| Instruction::LDY(_)
			| Instruction::LDSP(_) => 2,

			Instruction::ASLA
			| Instruction::ASRA
			| Instruction::CLRA
			| Instruction::COMA
			| Instruction::DECA
			| Instruction::EXG(_)
			| Instruction::INCA
			| Instruction::LSRA
			| Instruction::NEGA
			| Instruction::NOP
			| Instruction::PSHA
			| Instruction::PSHX
			| Instruction::PSHY
			| Instruction::PSHCC
			| Instruction::PULA
			| Instruction::PULX
			| Instruction::PULY
			| Instruction::PULCC
			| Instruction::ROLA
			| Instruction::RORA
			| Instruction::TFR(_)
			| Instruction::TSTA => 1,

			Instruction::ASL(adr)
			| Instruction::ASR(adr)
			| Instruction::CLR(adr)
			| Instruction::COM(adr)
			| Instruction::DEC(adr)
			| Instruction::INC(adr)
			| Instruction::LSR(adr)
			| Instruction::NEG(adr)
			| Instruction::ROL(adr)
			| Instruction::ROR(adr) => match adr {
				AddrTypeThree::Addr
				| AddrTypeThree::nSP
				| AddrTypeThree::nX
				| AddrTypeThree::nY => 2,
				AddrTypeThree::AY | AddrTypeThree::AX => 1,
			},

			Instruction::STX(adr)
			| Instruction::STY(adr)
			| Instruction::STSP(adr)
			| Instruction::TST(adr) => match adr {
				AddrTypeOne::Addr | AddrTypeOne::nSP | AddrTypeOne::nX | AddrTypeOne::nY => 2,
				AddrTypeOne::AX | AddrTypeOne::AY => 1,
			},

			Instruction::STA(adr) => match adr {
				StaAddr::Addr | StaAddr::nSP | StaAddr::nX | StaAddr::nY => 2,
				StaAddr::AX
				| StaAddr::Xplus
				| StaAddr::Xminus
				| StaAddr::plusX
				| StaAddr::minusX
				| StaAddr::AY
				| StaAddr::Yplus
				| StaAddr::Yminus
				| StaAddr::plusY
				| StaAddr::minusY => 1,
			},
			Instruction::LDA(adr) => match adr {
				LdaAddr::Data | LdaAddr::Addr | LdaAddr::nSP | LdaAddr::nX | LdaAddr::nY => 2,
				LdaAddr::AX
				| LdaAddr::Xplus
				| LdaAddr::Xminus
				| LdaAddr::plusX
				| LdaAddr::minusX
				| LdaAddr::AY
				| LdaAddr::Yplus
				| LdaAddr::Yminus
				| LdaAddr::plusY
				| LdaAddr::minusY => 1,
			},
		}
	}
}
