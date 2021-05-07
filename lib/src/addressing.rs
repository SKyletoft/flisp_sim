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
pub enum StxAddr {
	Addr,
	nSP,
	nX,
	nY,
	AX,
	AY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StyAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StspAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AdcaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IncAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EoraAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SubaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TstAddr {
	Addr,
	nSP,
	nX,
	nY,
	AX,
	AY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AndaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AslAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LsrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AsrAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CmpaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CmpxAddr {
	Data,
	Addr,
	nSP,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CmpyAddr {
	Data,
	Addr,
	nSP,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CmpspAddr {
	Data,
	Addr,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JmpAddr {
	Addr,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JsrAddr {
	Addr,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LdxAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LdyAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LdspAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NegAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OraAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RolAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RorAddr {
	Addr,
	nSP,
	nX,
	nY,
	AY,
	AX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SbcaAddr {
	Data,
	Addr,
	nSP,
	nX,
	nY,
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
