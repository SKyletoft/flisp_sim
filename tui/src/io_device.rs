#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum IoDevice {
	Nothing,
	Bargraph(u8),
	HexDisplay(u8),
	SevenSeg(u8),
	StepperMotor(u8),
	DILSwitch(u8),
	KeyPad(u8),
	IRQFlipFlop(u8),
}

impl IoDevice {
	pub(crate) fn read(&self) -> Option<u8> {
		let res = match self {
			IoDevice::Nothing => return None,
			IoDevice::Bargraph(_) => 0,
			IoDevice::HexDisplay(stored) => *stored,
			IoDevice::SevenSeg(stored) => *stored,
			IoDevice::StepperMotor(stored) => *stored,
			IoDevice::DILSwitch(stored) => *stored,
			IoDevice::KeyPad(stored) => *stored,
			IoDevice::IRQFlipFlop(stored) => *stored,
		};
		Some(res)
	}

	pub(crate) fn to_widget<W: tui::widgets::Widget>(&self) -> Box<W> {
		todo!()
	}
}
