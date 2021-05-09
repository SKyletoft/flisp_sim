use std::{
	convert::{TryFrom, TryInto},
	fmt::Write,
	mem, result,
	str::FromStr,
};

use crate::*;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Flisp {
	pub A: u8,
	pub Y: u8,
	pub X: u8,
	/// INZVC
	pub CC: u8,
	pub SP: u8,
	pub PC: u8,
	pub mem: [u8; 256],
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
				let rhs = adr.get_value(&self, n);
				self.add(rhs + self.get_c() as u8);
			}
			Instruction::ADDA(adr) => {
				let rhs = adr.get_value(&self, n);
				self.add(rhs);
			}
			Instruction::ANDA(adr) => {
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.asl(self.mem[idx]);
			}
			Instruction::ASRA => {
				self.A = self.asr(self.A);
			}
			Instruction::ASR(adr) => {
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.asr(self.mem[idx]);
			}
			Instruction::BITA(adr) => {
				let rhs = adr.get_value(&self, n);
				self.and(rhs);
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
				let idx = adr.get_index(&self, n);
				self.clr();
				self.mem[idx] = 0;
			}
			Instruction::CMPA(adr) => {
				let rhs = adr.get_value(&self, n);
				self.cmp(self.A, rhs);
			}
			Instruction::CMPX(adr) => {
				let rhs = adr.get_value(&self, n);
				self.cmp(self.X, rhs);
			}
			Instruction::CMPY(adr) => {
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.com(self.mem[idx]);
			}
			Instruction::DECA => {
				self.A = self.dec(self.A);
			}
			Instruction::DEC(adr) => {
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.dec(self.mem[idx]);
			}
			Instruction::EORA(adr) => {
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.inc(self.mem[idx]);
			}
			Instruction::JMP(adr) => {
				let target = adr.get_target(&self, n);
				self.PC = target;
			}
			Instruction::JSR(adr) => {
				let target = adr.get_target(&self, n);
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
				let data = adr.get_value(self, n);
				self.set_n_from(data);
				self.set_z_from(data);
				self.set_v(false);
				self.A = data;
			}
			Instruction::LDY(adr) => {
				let rhs = adr.get_value(&self, n);
				self.set_n_from(rhs);
				self.set_z_from(rhs);
				self.set_v(false);
				self.A = rhs;
			}
			Instruction::LDSP(adr) => {
				let rhs = adr.get_value(&self, n);
				self.set_n_from(rhs);
				self.set_z_from(rhs);
				self.set_v(false);
				self.A = rhs;
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
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.lsr(self.mem[idx]);
			}
			Instruction::NEGA => {
				self.A = self.neg(self.A);
			}
			Instruction::NEG(adr) => {
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.neg(self.mem[idx]);
			}
			Instruction::NOP => {}
			Instruction::ORA(adr) => {
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.mem[idx] = self.rol(self.mem[idx]);
			}
			Instruction::RORA => {
				self.A = self.ror(self.A);
			}
			Instruction::ROR(adr) => {
				let idx = adr.get_index(&self, n);
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
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.mem[idx as usize] = self.X;
			}
			Instruction::STY(adr) => {
				let idx = adr.get_index(&self, n);
				self.mem[idx as usize] = self.Y;
			}
			Instruction::STSP(adr) => {
				let idx = adr.get_index(&self, n);
				self.mem[idx as usize] = self.SP;
			}

			Instruction::SUBA(adr) => {
				let rhs = adr.get_value(&self, n);
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
				let idx = adr.get_index(&self, n);
				self.tst(self.mem[idx])
			}
		}
		if self.PC == 0xFF {
			self.PC = self.mem[0xFF]
		} else {
			self.PC = self.PC.wrapping_add(1);
		}
	}

	//Prints line of disassembly to the out string and returns the index of the next instruction,
	// taking instruction parametres into account
	pub fn print_disassembly<T: Write>(&self, out: &mut T, idx: u8) -> Result<u8> {
		let index = idx as usize;
		let read: Result<Instruction> = self.mem[index].try_into();
		let next = self.mem[idx.wrapping_add(1) as usize];
		match read {
			Ok(inst) => match inst {
				Instruction::ADCA(adr) => {
					write!(out, "ADCA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::ADDA(adr) => {
					write!(out, "ADDA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::ANDA(adr) => {
					write!(out, "ANDA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::ANDCC => {
					write!(out, "ANDCC\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::ASLA => {
					write!(out, "ASLA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ASL(adr) => {
					write!(out, "ASL\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::ASRA => {
					write!(out, "ASRA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ASR(adr) => {
					write!(out, "ASR\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::BITA(adr) => {
					write!(out, "BITA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BLE => {
					write!(out, "BLE\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BLS => {
					write!(out, "BLS\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BLT => {
					write!(out, "BLT\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BMI => {
					write!(out, "BMI\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BNE => {
					write!(out, "BNE\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BPL => {
					write!(out, "BPL\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BRA => {
					write!(out, "BRA\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BSR => {
					write!(out, "BSR\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BVC => {
					write!(out, "BVC\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BVS => {
					write!(out, "BVS\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BCC => {
					write!(out, "BCC\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BCS => {
					write!(out, "BCS\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BEQ => {
					write!(out, "BEQ\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BGE => {
					write!(out, "BGE\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BGT => {
					write!(out, "BGT\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::BHI => {
					write!(out, "BHI\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::CLRA => {
					write!(out, "CLRA\t{:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::CLR(adr) => {
					write!(out, "CLR\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::CMPA(adr) => {
					write!(out, "CMPA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::CMPX(adr) => {
					write!(out, "CMPA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::CMPY(adr) => {
					write!(out, "CMPY\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::CMPSP(adr) => {
					match adr {
						CmpspAddr::Data => write!(out, "CMPSP\t#${:02X}", next)?,
						CmpspAddr::Addr => write!(out, "CMPSP\t${:02X}", next)?,
					}
					Ok(idx.wrapping_add(2))
				}
				Instruction::COMA => {
					write!(out, "COMA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::COM(adr) => {
					write!(out, "COM\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::DECA => {
					write!(out, "DECA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::DEC(adr) => {
					write!(out, "DEC\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::EORA(adr) => {
					write!(out, "EORA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::EXG(adr) => {
					match adr {
						ExgAddr::XY => write!(out, "EXG\tX,Y")?,
						ExgAddr::ACC => write!(out, "EXG\tA,CC")?,
						ExgAddr::XSP => write!(out, "EXG\tX,SP")?,
						ExgAddr::YSP => write!(out, "EXG\tY,SP")?,
					}
					Ok(idx.wrapping_add(1))
				}
				Instruction::INCA => {
					write!(out, "INCA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::INC(adr) => {
					write!(out, "INC\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::JMP(adr) => {
					write!(out, "JMP\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::JSR(adr) => {
					write!(out, "JSR\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::LDA(adr) => {
					let (_, ret) = match adr {
						LdaAddr::Data => (write!(out, "LDA\t#${:02X}", next)?, 2),
						LdaAddr::Addr => (write!(out, "LDA\t${:02X}", next)?, 2),
						LdaAddr::nSP => (write!(out, "LDA\t${:02X},SP", next)?, 2),
						LdaAddr::nX => (write!(out, "LDA\t${:02X},X", next)?, 2),
						LdaAddr::AX => (write!(out, "LDA\tA,Y")?, 1),
						LdaAddr::Xplus => (write!(out, "LDA\tX+")?, 1),
						LdaAddr::Xminus => (write!(out, "LDA\tX-")?, 1),
						LdaAddr::plusX => (write!(out, "LDA\t+X")?, 1),
						LdaAddr::minusX => (write!(out, "LDA\t-X")?, 1),
						LdaAddr::nY => (write!(out, "LDA\t${:02X},Y", next)?, 2),
						LdaAddr::AY => (write!(out, "LDA\tA,Y")?, 1),
						LdaAddr::Yplus => (write!(out, "LDA\tY+")?, 1),
						LdaAddr::Yminus => (write!(out, "LDA\tY-")?, 1),
						LdaAddr::plusY => (write!(out, "LDA\t+Y")?, 1),
						LdaAddr::minusY => (write!(out, "LDA\t-Y")?, 1),
					};
					Ok(idx.wrapping_add(ret))
				}
				Instruction::LDX(adr) => {
					write!(out, "LDX\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::LDY(adr) => {
					write!(out, "LDY\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::LDSP(adr) => {
					write!(out, "LDSP\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::LEAX(adr) => {
					match adr {
						LeaxAddr::nX => write!(out, "LEAX\t${:02X},X", next)?,
						LeaxAddr::nSP => write!(out, "LEAX\t${:02X},SP", next)?,
					}
					Ok(idx.wrapping_add(2))
				}
				Instruction::LEAY(adr) => {
					match adr {
						LeayAddr::nY => write!(out, "LEAY\t${:02X},Y", next)?,
						LeayAddr::nSP => write!(out, "LEAY\t${:02X},SP", next)?,
					}
					Ok(idx.wrapping_add(2))
				}
				Instruction::LEASP(adr) => {
					match adr {
						LeaspAddr::nX => write!(out, "LEASP\t${:02X},X", next)?,
						LeaspAddr::nY => write!(out, "LEASP\t${:02X},Y", next)?,
						LeaspAddr::nSP => write!(out, "LEASP\t${:02X},SP", next)?,
					}
					Ok(idx.wrapping_add(2))
				}
				Instruction::LSRA => {
					write!(out, "LSRA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::LSR(adr) => {
					write!(out, "LSR\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::NEGA => {
					write!(out, "NEGA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::NEG(adr) => {
					write!(out, "NEG\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::NOP => {
					write!(out, "NOP")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ORA(adr) => {
					write!(out, "ORA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::ORCC => {
					write!(out, "ORCC #${:02X}", next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::PSHA => {
					write!(out, "PSHA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PSHX => {
					write!(out, "PSHX")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PSHY => {
					write!(out, "PSHY")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PSHCC => {
					write!(out, "PSHCC")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PULA => {
					write!(out, "PULA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PULX => {
					write!(out, "PULX")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PULY => {
					write!(out, "PULY")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::PULCC => {
					write!(out, "PULCC")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ROLA => {
					write!(out, "ROLA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ROL(adr) => {
					write!(out, "ROL\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::RORA => {
					write!(out, "RORA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::ROR(adr) => {
					write!(out, "ROR\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::RTS => {
					write!(out, "RTS")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::RTI => {
					write!(out, "RTI")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::SBCA(adr) => {
					write!(out, "SBCA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::STA(adr) => {
					let (_, ret) = match adr {
						StaAddr::Addr => (write!(out, "STA\t${:02X}", next)?, 2),
						StaAddr::nSP => (write!(out, "STA\t${:02X},SP", next)?, 2),
						StaAddr::nX => (write!(out, "STA\t${:02X},X", next)?, 2),
						StaAddr::AX => (write!(out, "STA\tA,Y")?, 1),
						StaAddr::Xplus => (write!(out, "STA\tX+")?, 1),
						StaAddr::Xminus => (write!(out, "STA\tX-")?, 1),
						StaAddr::plusX => (write!(out, "STA\t+X")?, 1),
						StaAddr::minusX => (write!(out, "STA\t-X")?, 1),
						StaAddr::nY => (write!(out, "STA\t${:02X},Y", next)?, 2),
						StaAddr::AY => (write!(out, "STA\tA,Y")?, 1),
						StaAddr::Yplus => (write!(out, "STA\tY+")?, 1),
						StaAddr::Yminus => (write!(out, "STA\tY-")?, 1),
						StaAddr::plusY => (write!(out, "STA\t+Y")?, 1),
						StaAddr::minusY => (write!(out, "STA\t-Y")?, 1),
					};
					Ok(idx.wrapping_add(ret))
				}
				Instruction::STX(adr) => {
					write!(out, "STX\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::STY(adr) => {
					write!(out, "STY\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::STSP(adr) => {
					write!(out, "STSP\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
				Instruction::SUBA(adr) => {
					write!(out, "SUBA\t")?;
					adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(2))
				}
				Instruction::TFR(adr) => {
					match adr {
						TfrAddr::ACC => write!(out, "TFR\tA,CC")?,
						TfrAddr::CCA => write!(out, "TFR\tCC,A")?,
						TfrAddr::XY => write!(out, "TFR\tX,Y")?,
						TfrAddr::YX => write!(out, "TFR\tY,X")?,
						TfrAddr::XSP => write!(out, "TFR\tX,SP")?,
						TfrAddr::SPX => write!(out, "TFR\tSP,X")?,
						TfrAddr::YSP => write!(out, "TFR\tY,SP")?,
						TfrAddr::SPY => write!(out, "TFR\tSP,Y")?,
					}
					Ok(idx.wrapping_add(1))
				}
				Instruction::TSTA => {
					write!(out, "TSTA")?;
					Ok(idx.wrapping_add(1))
				}
				Instruction::TST(adr) => {
					write!(out, "TST\t")?;
					let ret = adr.write_with_next(out, next)?;
					Ok(idx.wrapping_add(ret))
				}
			},
			Err(_) => {
				write!(out, "\tFCB\t${:02X}", self.mem[index])?;
				Ok(idx.wrapping_add(1))
			}
		}
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

impl Default for Flisp {
	fn default() -> Self {
		Flisp {
			A: 0,
			Y: 0,
			X: 0,
			CC: 0,
			SP: 0,
			PC: 0xFF,
			mem: [0; 256],
		}
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
