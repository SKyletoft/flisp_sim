use std::process::Command;

fn main() {
	let command = Command::new("node")
		.arg("src/reference/converter.mjs")
		.output()
		.unwrap()
		.stdout;
	std::fs::write("target/match", command).unwrap();
}
