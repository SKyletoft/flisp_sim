use std::{convert::TryFrom, result, str::FromStr};

use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Flisp {
	A: u8,
	Y: u8,
	X: u8,
	CC: u8, //	INZVC
	SP: u8,
	mem: [u8; 256],
}

impl Flisp {
	fn set_n(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 3)) | ((to as u8) << 3);
	}
	fn set_z(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 2)) | ((to as u8) << 2);
	}
	fn set_v(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 1)) | ((to as u8) << 1);
	}
	fn set_c(&mut self, to: bool) {
		self.CC = (self.CC & !1) | (to as u8);
	}

	fn add(&mut self, data: u8) {
		let a_before = self.A as i8;
		let (res, carry) = self.A.overflowing_add(data);
		self.A = res;
		self.set_n((res & (1 << 7)) != 0);
		self.set_z(res == 0);
		self.set_v(a_before > self.A as i8);
		self.set_c(carry);
	}

	fn anda(&mut self, data: u8) {
		self.A &= data;
		self.set_n(self.A & (1 << 7) != 0);
		self.set_z(self.A == 0);
		self.set_v(false);
	}

	pub fn step(&mut self) -> crate::error::Result<()> {
		let inst: Instruction = Instruction::try_from(self.mem[self.SP as usize])?;
		let n = self.mem[self.SP.wrapping_add(1) as usize];
		match inst {
			Instruction::ADCA(adr) => {
				let rhs = match adr {
					AdcaAddr::Addr => self.mem[n as usize],
					AdcaAddr::Data => n,
					AdcaAddr::nSP => n.wrapping_add(self.SP),
					AdcaAddr::nX => n.wrapping_add(self.X),
					AdcaAddr::nY => n.wrapping_add(self.Y),
				};
				self.add(rhs + (self.CC & 1));
			}
			Instruction::ADDA(adr) => {
				let rhs = match adr {
					AddaAddr::Addr => self.mem[n as usize],
					AddaAddr::Data => n,
					AddaAddr::nSP => n.wrapping_add(self.SP),
					AddaAddr::nX => n.wrapping_add(self.X),
					AddaAddr::nY => n.wrapping_add(self.Y),
				};
				self.add(rhs);
			}
			Instruction::ANDA(adr) => {
				let rhs = match adr {
					AndaAddr::Addr => self.mem[n as usize],
					AndaAddr::Data => n,
					AndaAddr::nSP => n.wrapping_add(self.SP),
					AndaAddr::nX => n.wrapping_add(self.X),
					AndaAddr::nY => n.wrapping_add(self.Y),
				};
				self.anda(rhs);
			}
			Instruction::ANDCC => {
				let rhs = n;
				self.CC &= rhs;
			}
			Instruction::ASLA => {}
			Instruction::ASL(adr) => {
				let idx = match adr {
					AslAddr::Addr => n,
					AslAddr::nSP => n + self.SP,
					AslAddr::nX => n + self.X,
					AslAddr::nY => n + self.Y,
					AslAddr::AY => self.A + self.Y,
					AslAddr::AX => self.A + self.X,
				};
			}
			Instruction::ASRA => {}
			Instruction::ASR(_) => {}
			Instruction::BITA(_) => {}
			Instruction::BLE => {}
			Instruction::BLS => {}
			Instruction::BLT => {}
			Instruction::BMI => {}
			Instruction::BNE => {}
			Instruction::BPL => {}
			Instruction::BRA => {}
			Instruction::BSR => {}
			Instruction::BVC => {}
			Instruction::BVS => {}
			Instruction::BCC => {}
			Instruction::BCS => {}
			Instruction::BEQ => {}
			Instruction::BGE => {}
			Instruction::BGT => {}
			Instruction::BHI => {}
			Instruction::CLRA => {}
			Instruction::CLR(_) => {}
			Instruction::CMPA(_) => {}
			Instruction::CMPX(_) => {}
			Instruction::CMPY(_) => {}
			Instruction::CMPSP(_) => {}
			Instruction::COMA => {}
			Instruction::COM(_) => {}
			Instruction::DECA => {}
			Instruction::DEC(_) => {}
			Instruction::EORA(_) => {}
			Instruction::EXG(_) => {}
			Instruction::INCA => {}
			Instruction::INC(_) => {}
			Instruction::JMP(_) => {}
			Instruction::JSR(_) => {}
			Instruction::LDA(_) => {}
			Instruction::LDX(_) => {}
			Instruction::LDY(_) => {}
			Instruction::LDSP(_) => {}
			Instruction::LEAX(_) => {}
			Instruction::LEAY(_) => {}
			Instruction::LEASP(_) => {}
			Instruction::LSRA => {}
			Instruction::LSR(_) => {}
			Instruction::NEGA => {}
			Instruction::NEG(_) => {}
			Instruction::NOP => {}
			Instruction::ORA(_) => {}
			Instruction::ORCC => {}
			Instruction::PSHA => {}
			Instruction::PSHX => {}
			Instruction::PSHY => {}
			Instruction::PSHCC => {}
			Instruction::PULA => {}
			Instruction::PULX => {}
			Instruction::PULY => {}
			Instruction::PULCC => {}
			Instruction::ROLA => {}
			Instruction::ROL(_) => {}
			Instruction::RORA => {}
			Instruction::ROR(_) => {}
			Instruction::RTS => {}
			Instruction::RTI => {}
			Instruction::SBCA(_) => {}
			Instruction::STA(_) => {}
			Instruction::STX(_) => {}
			Instruction::STY(_) => {}
			Instruction::STSP(_) => {}
			Instruction::SUBA(_) => {}
			Instruction::TFR(_) => {}
			Instruction::TSTA => {}
			Instruction::TST(_) => {}
		}
		self.SP += 1;
		Ok(())
	}
}

impl FromStr for Flisp {
	type Err = FlispError;

	fn from_str(s: &str) -> result::Result<Self, Self::Err> {
		let mut flisp = Flisp {
			A: 0,
			Y: 0,
			X: 0,
			CC: 0,
			SP: 0xFF,
			mem: [0; 256],
		};

		for line in s.lines().filter(|l| !l.is_empty()) {
			let line = line
				.strip_prefix(" #setMemory  ")
				.ok_or(FlispError::InvalidLineConversion(line!()))?;
			let adr = u8::from_str_radix(
				line.get(0..2)
					.ok_or(FlispError::InvalidLineConversion(line!()))?,
				16,
			)
			.map_err(|_| FlispError::InvalidLineConversion(line!()))?;
			let val = u8::from_str_radix(
				line.get(3..5)
					.ok_or(FlispError::InvalidLineConversion(line!()))?,
				16,
			)
			.map_err(|_| FlispError::InvalidLineConversion(line!()))?;
			flisp.mem[adr as usize] = val;
		}

		flisp.SP = flisp.mem[255];

		Ok(flisp)
	}
}

#[cfg(test)]
mod test {
	use std::str::FromStr;

	use crate::*;
	#[test]
	fn primes() {
		let primes_source = include_str!("deps/primes_source.fmem");
		let mut flisp = Flisp::from_str(primes_source).unwrap();
		let starting_mem = [
			0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x35, 0x3F, 0xF1, 0x3F, 0xA7, 0x3D,
			0x2D, 0x17, 0x91, 0x00, 0xFA, 0x10, 0xF2, 0x02, 0x34, 0x8A, 0xBE, 0x01, 0xE1, 0x3E,
			0x05, 0xA7, 0x3E, 0x25, 0x02, 0x05, 0x43, 0x37, 0x3F, 0x33, 0x42, 0xF0, 0xFF, 0x43,
			0xF0, 0x05, 0x10, 0xF2, 0x00, 0x97, 0x05, 0x2F, 0x1C, 0x10, 0x34, 0x40, 0xBE, 0x01,
			0x09, 0x24, 0x0C, 0xF0, 0x00, 0xA6, 0x3D, 0x10, 0x15, 0xF2, 0x00, 0xE3, 0x00, 0x37,
			0x3D, 0xF0, 0x02, 0xB6, 0x00, 0xE2, 0x00, 0x33, 0x65, 0xBE, 0x01, 0x43, 0x10, 0xF0,
			0x00, 0xE2, 0xFF, 0x14, 0xB7, 0x01, 0x2D, 0x01, 0x43, 0xB4, 0x01, 0x47, 0xFE, 0x33,
			0x90, 0x92, 0xFB, 0x34, 0x62, 0x33, 0x9F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x9B,
		];
		assert_eq!(flisp.mem, starting_mem);

		while flisp.SP != 0x9F {
			flisp.step().unwrap();
		}

		let ending_mem: [u8; 256] = [
			0x02, 0x03, 0x05, 0x07, 0x0B, 0x0D, 0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25, 0x29, 0x2B,
			0x2F, 0x35, 0x3B, 0x3D, 0x43, 0x47, 0x49, 0x4F, 0x53, 0x59, 0x61, 0x65, 0x67, 0x6B,
			0x6D, 0x71, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 0x0E, 0x1E, 0x35, 0x3F, 0xF1, 0x3F, 0xA7, 0x3D,
			0x2D, 0x17, 0x91, 0x00, 0xFA, 0x10, 0xF2, 0x02, 0x34, 0x8A, 0xBE, 0x01, 0xE1, 0x3E,
			0x05, 0xA7, 0x3E, 0x25, 0x02, 0x05, 0x43, 0x37, 0x3F, 0x33, 0x42, 0xF0, 0xFF, 0x43,
			0xF0, 0x05, 0x10, 0xF2, 0x00, 0x97, 0x05, 0x2F, 0x1C, 0x10, 0x34, 0x40, 0xBE, 0x01,
			0x09, 0x24, 0x0C, 0xF0, 0x00, 0xA6, 0x3D, 0x10, 0x15, 0xF2, 0x00, 0xE3, 0x00, 0x37,
			0x3D, 0xF0, 0x02, 0xB6, 0x00, 0xE2, 0x00, 0x33, 0x65, 0xBE, 0x01, 0x43, 0x10, 0xF0,
			0x00, 0xE2, 0xFF, 0x14, 0xB7, 0x01, 0x2D, 0x01, 0x43, 0xB4, 0x01, 0x47, 0xFE, 0x33,
			0x90, 0x92, 0xFB, 0x34, 0x62, 0x33, 0x9F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7F, 0x50, 0x71, 0x6E, 0x1E, 0x81, 0x9F, 0x00,
			0x00, 0x00, 0x00, 0x9B,
		];
		assert_eq!(flisp.mem, ending_mem);
	}
}
