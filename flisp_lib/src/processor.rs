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

	fn lsr(&mut self, data: u8) -> u8 {
		let carry = (data & 1) != 0;
		let res = data >> 1;
		self.set_n(false);
		self.set_z_from(res);
		self.set_v(data & 0b1000_0000 != 0); //Unsure
		self.set_c(carry);
		res
	}

	fn neg(&mut self, data: u8) -> u8 {
		let (res, carry) = data.overflowing_neg();
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(carry);
		self.set_c(data != 0);
		res
	}

	fn or(&mut self, data: u8) -> u8 {
		let res = self.A & data;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(false);
		res
	}

	fn rol(&mut self, data: u8) -> u8 {
		let res = (data << 1) | self.get_c() as u8;
		let carry = (data & 0b1000_0000) != 0;
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(((res ^ data) & 0b1000_0000) != 0);
		self.set_c(carry);
		res
	}

	fn ror(&mut self, data: u8) -> u8 {
		let res = (data >> 1) | ((self.get_c() as u8) << 7);
		let carry = (data & 1) != 0;
		self.set_n(self.get_c());
		self.set_z_from(res);
		self.set_v(self.get_c() && !carry); //Unsure
		self.set_c(carry);
		res
	}

	fn sub(&mut self, data: u8) -> u8 {
		let (res, carry) = self.A.overflowing_sub(data);
		self.set_n_from(res);
		self.set_z_from(res);
		self.set_v(carry);
		self.set_c(((data & !self.A) & 0b1000_0000) != 0); //Unsure
		res
	}

	fn tst(&mut self, data: u8) {
		self.set_n_from(data);
		self.set_z_from(data);
		self.set_v(false);
		self.set_c(false);
	}

	pub fn step(&mut self) {
		let inst = if let Ok(inst) = Instruction::try_from(self.mem[self.PC as usize]) {
			inst
		} else {
			self.PC = self.mem[0xFD];
			return;
		};
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
				self.add(rhs + self.get_c() as u8);
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
			Instruction::LEAX(adr) => {
				let res = match adr {
					LeaxAddr::nX => n + self.X,
					LeaxAddr::nSP => n + self.SP,
				};
				self.X = res;
			}
			Instruction::LEAY(adr) => {
				let res = match adr {
					LeayAddr::nY => n + self.Y,
					LeayAddr::nSP => n + self.SP,
				};
				self.Y = res;
			}
			Instruction::LEASP(adr) => {
				let res = match adr {
					LeaspAddr::nX => n + self.X,
					LeaspAddr::nY => n + self.Y,
					LeaspAddr::nSP => n + self.SP,
				};
				self.SP = res;
			}
			Instruction::LSRA => {
				self.A = self.lsr(self.A);
			}
			Instruction::LSR(adr) => {
				let idx = match adr {
					LsrAddr::Addr => n,
					LsrAddr::nSP => n + self.SP,
					LsrAddr::nX => n + self.X,
					LsrAddr::nY => n + self.Y,
					LsrAddr::AY => self.A + self.Y,
					LsrAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.lsr(self.mem[idx]);
			}
			Instruction::NEGA => {
				self.A = self.neg(self.A);
			}
			Instruction::NEG(adr) => {
				let idx = match adr {
					NegAddr::Addr => n,
					NegAddr::nSP => n + self.SP,
					NegAddr::nX => n + self.X,
					NegAddr::nY => n + self.Y,
					NegAddr::AY => self.A + self.Y,
					NegAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.neg(self.mem[idx]);
			}
			Instruction::NOP => {}
			Instruction::ORA(adr) => {
				let rhs = match adr {
					OraAddr::Data => n,
					OraAddr::Addr => self.mem[n as usize],
					OraAddr::nSP => self.mem[(n + self.SP) as usize],
					OraAddr::nX => self.mem[(n + self.X) as usize],
					OraAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.A = self.or(rhs);
			}
			Instruction::ORCC => {
				let rhs = n;
				self.CC &= rhs;
			}
			Instruction::PSHA => {
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.A;
			}
			Instruction::PSHX => {
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.X;
			}
			Instruction::PSHY => {
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.Y;
			}
			Instruction::PSHCC => {
				self.SP = self.SP.wrapping_sub(1);
				self.mem[self.SP as usize] = self.CC;
			}
			Instruction::PULA => {
				self.A = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::PULX => {
				self.X = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::PULY => {
				self.Y = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::PULCC => {
				self.CC = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::ROLA => {
				self.A = self.rol(self.A);
			}
			Instruction::ROL(adr) => {
				let idx = match adr {
					RolAddr::Addr => n,
					RolAddr::nSP => n + self.SP,
					RolAddr::nX => n + self.X,
					RolAddr::nY => n + self.Y,
					RolAddr::AY => self.A + self.Y,
					RolAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.rol(self.mem[idx]);
			}
			Instruction::RORA => {
				self.A = self.ror(self.A);
			}
			Instruction::ROR(adr) => {
				let idx = match adr {
					RorAddr::Addr => n,
					RorAddr::nSP => n + self.SP,
					RorAddr::nX => n + self.X,
					RorAddr::nY => n + self.Y,
					RorAddr::AY => self.A + self.Y,
					RorAddr::AX => self.A + self.X,
				} as usize;
				self.mem[idx] = self.ror(self.mem[idx]);
			}
			Instruction::RTS => {
				self.PC = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::RTI => {
				self.CC = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
				self.A = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
				self.X = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
				self.Y = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
				self.PC = self.mem[self.SP as usize];
				self.SP = self.SP.wrapping_add(1);
			}
			Instruction::SBCA(adr) => {
				let rhs = match adr {
					SbcaAddr::Data => n,
					SbcaAddr::Addr => self.mem[n as usize],
					SbcaAddr::nSP => self.mem[(n + self.SP) as usize],
					SbcaAddr::nX => self.mem[(n + self.X) as usize],
					SbcaAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.A = self.sub(rhs + self.get_c() as u8);
			}
			Instruction::STA(adr) => {
				let idx = match adr {
					StaAddr::Addr => self.mem[n as usize],
					StaAddr::nSP => self.mem[(n + self.SP) as usize],
					StaAddr::nX => self.mem[(n + self.X) as usize],
					StaAddr::AX => self.mem[(self.A + self.X) as usize],
					StaAddr::Xplus => {
						let x = self.X;
						self.X = self.X.wrapping_add(1);
						self.mem[x as usize]
					}
					StaAddr::Xminus => {
						let x = self.X;
						self.X = self.X.wrapping_sub(1);
						self.mem[x as usize]
					}
					StaAddr::plusX => {
						self.X = self.X.wrapping_add(1);
						self.mem[self.X as usize]
					}
					StaAddr::minusX => {
						self.X = self.X.wrapping_sub(1);
						self.mem[self.X as usize]
					}
					StaAddr::nY => self.mem[(n + self.Y) as usize],
					StaAddr::AY => self.mem[(self.A + self.Y) as usize],
					StaAddr::Yplus => {
						let y = self.Y;
						self.Y = self.Y.wrapping_add(1);
						self.mem[y as usize]
					}
					StaAddr::Yminus => {
						let y = self.Y;
						self.Y = self.Y.wrapping_sub(1);
						self.mem[y as usize]
					}
					StaAddr::plusY => {
						self.Y = self.Y.wrapping_add(1);
						self.mem[self.Y as usize]
					}
					StaAddr::minusY => {
						self.Y = self.Y.wrapping_sub(1);
						self.mem[self.Y as usize]
					}
				} as usize;
				self.mem[idx] = self.A;
			}
			Instruction::STX(adr) => {
				let idx = match adr {
					StxAddr::Addr => n,
					StxAddr::nSP => n + self.SP,
					StxAddr::nX => n + self.X,
					StxAddr::nY => n + self.Y,
					StxAddr::AX => self.A + self.X,
					StxAddr::AY => self.A + self.Y,
				};
				self.mem[idx as usize] = self.X;
			}
			Instruction::STY(adr) => {
				let idx = match adr {
					StyAddr::Addr => n,
					StyAddr::nSP => n + self.SP,
					StyAddr::nX => n + self.X,
					StyAddr::nY => n + self.Y,
					StyAddr::AX => self.A + self.X,
					StyAddr::AY => self.A + self.Y,
				};
				self.mem[idx as usize] = self.Y;
			}
			Instruction::STSP(adr) => {
				let idx = match adr {
					StspAddr::Addr => n,
					StspAddr::nSP => n + self.SP,
					StspAddr::nX => n + self.X,
					StspAddr::nY => n + self.Y,
					StspAddr::AX => self.A + self.X,
					StspAddr::AY => self.A + self.Y,
				};
				self.mem[idx as usize] = self.SP;
			}

			Instruction::SUBA(adr) => {
				let rhs = match adr {
					SubaAddr::Data => n,
					SubaAddr::Addr => self.mem[n as usize],
					SubaAddr::nSP => self.mem[(n + self.SP) as usize],
					SubaAddr::nX => self.mem[(n + self.X) as usize],
					SubaAddr::nY => self.mem[(n + self.Y) as usize],
				};
				self.A = self.sub(rhs);
			}

			Instruction::TFR(adr) => match adr {
				TfrAddr::ACC => self.CC = self.A,
				TfrAddr::CCA => self.A = self.CC,
				TfrAddr::XY => self.Y = self.X,
				TfrAddr::YX => self.X = self.Y,
				TfrAddr::XSP => self.SP = self.X,
				TfrAddr::SPX => self.X = self.SP,
				TfrAddr::YSP => self.SP = self.Y,
				TfrAddr::SPY => self.Y = self.SP,
			},
			Instruction::TSTA => {
				self.tst(self.A);
			}
			Instruction::TST(adr) => {
				let idx = match adr {
					TstAddr::Addr => n,
					TstAddr::nSP => n + self.SP,
					TstAddr::nX => n + self.X,
					TstAddr::nY => n + self.Y,
					TstAddr::AX => self.A + self.X,
					TstAddr::AY => self.A + self.Y,
				} as usize;
				self.tst(self.mem[idx])
			}
		}
		self.PC = self.PC.wrapping_add(1);
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
			flisp.step();
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
