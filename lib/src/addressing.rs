use crate::error::Result;
use crate::processor::Flisp;

use std::fmt;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LdaAddr {
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
pub enum StaAddr {
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
pub enum ExgAddr {
	XY,
	ACC,
	XSP,
	YSP,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LeaxAddr {
	nX,
	nSP,
}
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LeayAddr {
	nY,
	nSP,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LeaspAddr {
	nX,
	nY,
	nSP,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddrTypeOne {
	Addr,
	nSP,
	nX,
	nY,
	AX,
	AY,
}

impl AddrTypeOne {
	pub fn get_index(&self, flisp: &Flisp, n: u8) -> usize {
		match self {
			AddrTypeOne::Addr => n as usize,
			AddrTypeOne::nSP => n.wrapping_add(flisp.SP) as usize,
			AddrTypeOne::nX => n.wrapping_add(flisp.X) as usize,
			AddrTypeOne::nY => n.wrapping_add(flisp.Y) as usize,
			AddrTypeOne::AX => flisp.A.wrapping_add(flisp.X) as usize,
			AddrTypeOne::AY => flisp.A.wrapping_add(flisp.Y) as usize,
		}
	}

	pub fn write_with_next<T: fmt::Write>(&self, buf: &mut T, next: u8) -> Result<u8> {
		let (_, ret) = match self {
			AddrTypeOne::Addr => (write!(buf, "${:02X}", next)?, 2),
			AddrTypeOne::nSP => (write!(buf, "${:02X},SP", next)?, 2),
			AddrTypeOne::nX => (write!(buf, "${:02X},X", next)?, 2),
			AddrTypeOne::nY => (write!(buf, "${:02X},Y", next)?, 2),
			AddrTypeOne::AX => (write!(buf, "A,X")?, 1),
			AddrTypeOne::AY => (write!(buf, "A,Y")?, 1),
		};
		Ok(ret)
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddrTypeTwo {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

impl AddrTypeTwo {
	pub fn get_value(&self, flisp: &Flisp, n: u8) -> u8 {
		match self {
			AddrTypeTwo::Data => n,
			AddrTypeTwo::Addr => flisp.mem[n as usize],
			AddrTypeTwo::nSP => flisp.mem[n.wrapping_add(flisp.SP) as usize],
			AddrTypeTwo::nX => flisp.mem[n.wrapping_add(flisp.X) as usize],
			AddrTypeTwo::nY => flisp.mem[n.wrapping_add(flisp.Y) as usize],
		}
	}

	pub fn write_with_next<T: fmt::Write>(&self, buf: &mut T, next: u8) -> Result<()> {
		match self {
			AddrTypeTwo::Data => write!(buf, "#${:02X}", next)?,
			AddrTypeTwo::Addr => write!(buf, "${:02X}", next)?,
			AddrTypeTwo::nSP => write!(buf, "${:02X},SP", next)?,
			AddrTypeTwo::nX => write!(buf, "${:02X},X", next)?,
			AddrTypeTwo::nY => write!(buf, "${:02X},Y", next)?,
		}
		Ok(())
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddrTypeThree {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

impl AddrTypeThree {
	pub fn get_index(&self, flisp: &Flisp, n: u8) -> usize {
		match self {
			AddrTypeThree::Addr => n as usize,
			AddrTypeThree::nSP => n.wrapping_add(flisp.SP) as usize,
			AddrTypeThree::nX => n.wrapping_add(flisp.X) as usize,
			AddrTypeThree::nY => n.wrapping_add(flisp.Y) as usize,
			AddrTypeThree::AY => flisp.A.wrapping_add(flisp.Y) as usize,
			AddrTypeThree::AX => flisp.A.wrapping_add(flisp.X) as usize,
		}
	}

	pub fn write_with_next<T: fmt::Write>(&self, buf: &mut T, next: u8) -> Result<u8> {
		let (_, ret) = match self {
			AddrTypeThree::Addr => (write!(buf, "{:02X}", next)?, 2),
			AddrTypeThree::nSP => (write!(buf, "{:02X},SP", next)?, 2),
			AddrTypeThree::nX => (write!(buf, "{:02X},X", next)?, 2),
			AddrTypeThree::nY => (write!(buf, "{:02X},Y", next)?, 2),
			AddrTypeThree::AY => (write!(buf, "A,Y")?, 1),
			AddrTypeThree::AX => (write!(buf, "A,X")?, 1),
		};
		Ok(ret)
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddrTypeFour {
	Data,
	Addr,
	nSP,
}

impl AddrTypeFour {
	pub fn get_value(&self, flisp: &Flisp, n: u8) -> u8 {
		match self {
			AddrTypeFour::Addr => flisp.mem[n as usize],
			AddrTypeFour::Data => n,
			AddrTypeFour::nSP => n.wrapping_add(flisp.SP),
		}
	}

	pub fn write_with_next<T: fmt::Write>(&self, buf: &mut T, next: u8) -> Result<()> {
		match self {
			AddrTypeFour::Data => write!(buf, "#${:02X}", next)?,
			AddrTypeFour::Addr => write!(buf, "${:02X}", next)?,
			AddrTypeFour::nSP => write!(buf, "${:02X},SP", next)?,
		}
		Ok(())
	}
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CmpspAddr {
	Data,
	Addr,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddrTypeFive {
	Addr,
	nX,
	nY,
	AY,
	AX,
}

impl AddrTypeFive {
	pub fn get_target(&self, flisp: &Flisp, n: u8) -> u8 {
		match self {
			AddrTypeFive::Addr => n,
			AddrTypeFive::nX => n.wrapping_add(flisp.X),
			AddrTypeFive::nY => n.wrapping_add(flisp.Y),
			AddrTypeFive::AY => flisp.A.wrapping_add(flisp.Y),
			AddrTypeFive::AX => flisp.A.wrapping_add(flisp.X),
		}
	}

	pub fn write_with_next<T: fmt::Write>(&self, buf: &mut T, next: u8) -> Result<u8> {
		let (_, ret) = match self {
			AddrTypeFive::Addr => (write!(buf, "{:02X}", next)?, 2),
			AddrTypeFive::nX => (write!(buf, "{:02X},X", next)?, 2),
			AddrTypeFive::nY => (write!(buf, "{:02X},Y", next)?, 2),
			AddrTypeFive::AY => (write!(buf, "A,Y")?, 1),
			AddrTypeFive::AX => (write!(buf, "A,X")?, 1),
		};
		Ok(ret)
	}
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TfrAddr {
	ACC,
	CCA,
	XY,
	YX,
	XSP,
	SPX,
	YSP,
	SPY,
}
