use std::{convert::TryFrom, mem, result, str::FromStr};

use crate::*;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Flisp {
	A: u8,
	Y: u8,
	X: u8,
	/// INZVC
	CC: u8,
	SP: u8,
	PC: u8,
	mem: [u8; 256],
}

impl Flisp {
	fn set_n(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 3)) | ((to as u8) << 3);
	}
	fn get_n(&self) -> bool {
		self.CC & (1 << 3) != 0
	}
	fn set_n_from(&mut self, data: u8) {
		self.set_n((data & 0b1000_0000) != 0);
	}
	fn set_z(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 2)) | ((to as u8) << 2);
	}
	fn get_z(&self) -> bool {
		self.CC & (1 << 2) != 0
	}
	fn set_z_from(&mut self, data: u8) {
		self.set_z(data == 0);
	}
	fn set_v(&mut self, to: bool) {
		self.CC = (self.CC & !(1 << 1)) | ((to as u8) << 1);
	}
	fn get_v(&self) -> bool {
		self.CC & (1 << 2) != 0
	}
	fn set_c(&mut self, to: bool) {
		self.CC = (self.CC & !1) | (to as u8);
	}
	fn get_c(&self) -> bool {
		self.CC & 1 != 0
	}

	fn add(&mut self, data: u8) {
		let a_before = self.A as i8;
		let (res, carry) = self.A.overflowing_add(data);
		self.A = res;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(a_before > self.A as i8);
		self.set_c(carry);
	}

	fn and(&mut self, data: u8) -> u8 {
		let res = self.A & data;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(false);
		res
	}

	fn asl(&mut self, data: u8) -> u8 {
		let res = data << 1;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(((data ^ res) & 0b1000_0000) != 0);
		self.set_c((data & 0b1000_0000) != 0);
		data << 1
	}

	fn asr(&mut self, data: u8) -> u8 {
		let res = (data >> 1) | (data & 0b1000_0000);
		self.set_n_from(data);
		self.set_z_from(res);
		self.set_v(false);
		self.set_c((data & 1) != 0);
		res
	}

	fn clr(&mut self) {
		self.set_n(false);
		self.set_z(true);
		self.set_v(false);
		self.set_c(false);
	}

	fn cmp(&mut self, lhs: u8, rhs: u8) {
		let (diff, carry) = lhs.overflowing_sub(rhs);
		self.set_n_from(diff);
		self.set_z_from(diff);
		self.set_v(carry);
		self.set_c(diff as i8 > lhs as i8); //Unsure
	}

	fn com(&mut self, data: u8) -> u8 {
		let res = !data;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(false);
		res
	}

	fn dec(&mut self, data: u8) -> u8 {
		let (res, carry) = data.overflowing_sub(1);
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(carry);
		res
	}

	fn eora(&mut self, data: u8) {
		let res = self.A ^ data;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(false);
		self.A = res;
	}

	fn inc(&mut self, data: u8) -> u8 {
		let (res, carry) = data.overflowing_add(1);
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(carry);
		res
	}

	pub fn step(&mut self) -> crate::error::Result<()> {
		let inst: Instruction = Instruction::try_from(self.mem[self.PC as usize])?;
		let n = self.mem[self.PC.wrapping_add(1) as usize];
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
				self.A = self.and(rhs);
			}
			Instruction::ANDCC => {
				let rhs = n;
				self.CC &= rhs;
			}
			Instruction::ASLA => {
				self.A = self.asl(self.A);
			}
			Instruction::ASL(adr) => {
				let idx = match adr {
					AslAddr::Addr => n,
					AslAddr::nSP => n + self.SP,
					AslAddr::nX => n + self.X,
					AslAddr::nY => n + self.Y,
					AslAddr::AY => self.A + self.Y,
					AslAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.asl(self.mem[idx]);
			}
			Instruction::ASRA => {
				self.A = self.asr(self.A);
			}
			Instruction::ASR(adr) => {
				let idx = match adr {
					AsrAddr::Addr => n,
					AsrAddr::nSP => n + self.SP,
					AsrAddr::nX => n + self.X,
					AsrAddr::nY => n + self.Y,
					AsrAddr::AY => self.A + self.Y,
					AsrAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.asr(self.mem[idx]);
			}
			Instruction::BITA(adr) => {
				let word = match adr {
					BitaAddr::Data => n,
					BitaAddr::Addr => self.mem[n as usize],
					BitaAddr::nSP => self.mem[(n + self.SP) as usize],
					BitaAddr::nX => self.mem[(n + self.X) as usize],
					BitaAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.and(word);
			}
			Instruction::BLE => {
				if (self.get_n() ^ self.get_v()) || self.get_z() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BLS => {
				if self.get_c() || self.get_z() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BLT => {
				if self.get_n() ^ self.get_v() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BMI => {
				if self.get_n() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BNE => {
				if !self.get_z() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BPL => {
				if self.get_n() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BRA => {
				self.PC = self.PC.wrapping_add(n);
			}
			Instruction::BSR => {
				//Ordering?
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.PC;
				self.PC = self.PC.wrapping_add(n);
			}
			Instruction::BVC => {
				if !self.get_v() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BVS => {
				if self.get_v() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BCC => {
				if !self.get_c() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BCS => {
				if self.get_c() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BEQ => {
				if self.get_z() {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BGE => {
				if !(self.get_n() ^ self.get_v()) {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BGT => {
				if !((self.get_n() ^ self.get_v()) || self.get_z()) {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::BHI => {
				if !(self.get_c() || self.get_z()) {
					self.PC = self.PC.wrapping_add(n);
				}
			}
			Instruction::CLRA => {
				self.clr();
				self.A = 0;
			}
			Instruction::CLR(adr) => {
				let idx = match adr {
					ClrAddr::Addr => n,
					ClrAddr::nSP => n + self.SP,
					ClrAddr::nX => n + self.X,
					ClrAddr::nY => n + self.Y,
					ClrAddr::AY => self.A + self.Y,
					ClrAddr::AX => self.A + self.X,
				} as usize;
				self.clr();
				self.mem[idx] = 0;
			}
			Instruction::CMPA(adr) => {
				let rhs = match adr {
					CmpaAddr::Addr => self.mem[n as usize],
					CmpaAddr::Data => n,
					CmpaAddr::nSP => n.wrapping_add(self.SP),
					CmpaAddr::nX => n.wrapping_add(self.X),
					CmpaAddr::nY => n.wrapping_add(self.Y),
				};
				self.cmp(self.A, rhs);
			}
			Instruction::CMPX(adr) => {
				let rhs = match adr {
					CmpxAddr::Addr => self.mem[n as usize],
					CmpxAddr::Data => n,
					CmpxAddr::nSP => n.wrapping_add(self.SP),
				};
				self.cmp(self.X, rhs);
			}
			Instruction::CMPY(adr) => {
				let rhs = match adr {
					CmpyAddr::Addr => self.mem[n as usize],
					CmpyAddr::Data => n,
					CmpyAddr::nSP => n.wrapping_add(self.SP),
				};
				self.cmp(self.Y, rhs);
			}
			Instruction::CMPSP(adr) => {
				let rhs = match adr {
					CmpspAddr::Data => n,
					CmpspAddr::Addr => self.mem[n as usize],
				};
				self.cmp(self.SP, rhs);
			}
			Instruction::COMA => {
				self.A = self.com(self.A);
			}
			Instruction::COM(adr) => {
				let idx = match adr {
					ComAddr::Addr => n,
					ComAddr::nSP => n + self.SP,
					ComAddr::nX => n + self.X,
					ComAddr::nY => n + self.Y,
					ComAddr::AY => self.A + self.Y,
					ComAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.com(self.mem[idx]);
			}
			Instruction::DECA => {
				self.A = self.dec(self.A);
			}
			Instruction::DEC(adr) => {
				let idx = match adr {
					DecAddr::Addr => n,
					DecAddr::nSP => n + self.SP,
					DecAddr::nX => n + self.X,
					DecAddr::nY => n + self.Y,
					DecAddr::AY => self.A + self.Y,
					DecAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.dec(self.mem[idx]);
			}
			Instruction::EORA(adr) => {
				let rhs = match adr {
					EoraAddr::Data => n,
					EoraAddr::Addr => self.mem[n as usize],
					EoraAddr::nSP => self.mem[(n + self.SP) as usize],
					EoraAddr::nX => self.mem[(n + self.X) as usize],
					EoraAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.eora(rhs);
			}
			Instruction::EXG(adr) => match adr {
				ExgAddr::XY => mem::swap(&mut self.X, &mut self.Y),
				ExgAddr::ACC => mem::swap(&mut self.A, &mut self.CC),
				ExgAddr::XSP => mem::swap(&mut self.X, &mut self.SP),
				ExgAddr::YSP => mem::swap(&mut self.Y, &mut self.SP),
			},
			Instruction::INCA => {
				self.A = self.inc(self.A);
			}
			Instruction::INC(adr) => {
				let idx = match adr {
					IncAddr::Addr => n,
					IncAddr::nSP => n + self.SP,
					IncAddr::nX => n + self.X,
					IncAddr::nY => n + self.Y,
					IncAddr::AY => self.A + self.Y,
					IncAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.inc(self.mem[idx]);
			}
			Instruction::JMP(adr) => {
				let target = match adr {
					JmpAddr::Addr => n,
					JmpAddr::nX => n + self.X,
					JmpAddr::nY => n + self.Y,
					JmpAddr::AY => self.A + self.Y,
					JmpAddr::AX => self.A + self.X,
				};
				self.PC = target;
			}
			Instruction::JSR(adr) => {
				let target = match adr {
					JsrAddr::Addr => n,
					JsrAddr::nX => n + self.X,
					JsrAddr::nY => n + self.Y,
					JsrAddr::AY => self.A + self.Y,
					JsrAddr::AX => self.A + self.X,
				};
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.PC;
				self.PC = target;
			}
			Instruction::LDA(adr) => {
				let data = match adr {
					LdaAddr::Data => n,
					LdaAddr::Addr => self.mem[n as usize],
					LdaAddr::nSP => self.mem[(n + self.SP) as usize],
					LdaAddr::nX => self.mem[(n + self.X) as usize],
					LdaAddr::AX => self.mem[(self.A + self.X) as usize],
					LdaAddr::Xplus => {
						let x = self.X;
						self.X = self.X.wrapping_add(1);
						self.mem[x as usize]
					}
					LdaAddr::Xminus => {
						let x = self.X;
						self.X = self.X.wrapping_sub(1);
						self.mem[x as usize]
					}
					LdaAddr::plusX => {
						self.X = self.X.wrapping_add(1);
						self.mem[self.X as usize]
					}
					LdaAddr::minusX => {
						self.X = self.X.wrapping_sub(1);
						self.mem[self.X as usize]
					}
					LdaAddr::nY => self.mem[(n + self.Y) as usize],
					LdaAddr::AY => self.mem[(self.A + self.Y) as usize],
					LdaAddr::Yplus => {
						let y = self.Y;
						self.Y = self.Y.wrapping_add(1);
						self.mem[y as usize]
					}
					LdaAddr::Yminus => {
						let y = self.Y;
						self.Y = self.Y.wrapping_sub(1);
						self.mem[y as usize]
					}
					LdaAddr::plusY => {
						self.Y = self.Y.wrapping_add(1);
						self.mem[self.Y as usize]
					}
					LdaAddr::minusY => {
						self.Y = self.Y.wrapping_sub(1);
						self.mem[self.Y as usize]
					}
				};
				self.set_n_from(data);
				self.set_z_from(data);
				self.set_v(false);
				self.A = data;
			}
			Instruction::LDX(adr) => {
				let data = match adr {
					LdxAddr::Data => n,
					LdxAddr::Addr => self.mem[n as usize],
					LdxAddr::nSP => self.mem[(n + self.SP) as usize],
					LdxAddr::nX => self.mem[(n + self.X) as usize],
					LdxAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.set_n_from(data);
				self.set_z_from(data);
				self.set_v(false);
				self.A = data;
			}
			Instruction::LDY(adr) => {
				let data = match adr {
					LdyAddr::Data => n,
					LdyAddr::Addr => self.mem[n as usize],
					LdyAddr::nSP => self.mem[(n + self.SP) as usize],
					LdyAddr::nX => self.mem[(n + self.X) as usize],
					LdyAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.set_n_from(data);
				self.set_z_from(data);
				self.set_v(false);
				self.A = data;
			}
			Instruction::LDSP(adr) => {
				let data = match adr {
					LdspAddr::Data => n,
					LdspAddr::Addr => self.mem[n as usize],
					LdspAddr::nSP => self.mem[(n + self.SP) as usize],
					LdspAddr::nX => self.mem[(n + self.X) as usize],
					LdspAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.set_n_from(data);
				self.set_z_from(data);
				self.set_v(false);
				self.A = data;
			}
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
		self.PC = self.PC.wrapping_add(1);
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
			SP: 0,
			PC: 0xFF,
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

		flisp.PC = flisp.mem[255];

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
