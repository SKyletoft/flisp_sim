use crate::processor::Flisp;

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
