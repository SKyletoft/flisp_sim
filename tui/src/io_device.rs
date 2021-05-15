#[derive(Debug, Copy, Clone, PartialEq, variantly::Variantly)]
pub(crate) enum IoDevice {
	Nothing,
	Bargraph,
	HexDisplay,
	SevenSeg,
	StepperMotor,
	DILSwitch,
	KeyPad,
	IRQFlipFlop,
}
