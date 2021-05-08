use anyhow::Result;
use crossterm::{cursor, event, execute, ExecutableCommand};
use flisp_lib::processor::Flisp;
use std::{fmt::Write as fmtWrite, io, time::Duration};
use tui::{
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout},
	text::Span,
	widgets::{Block, BorderType, Borders, Paragraph, Wrap},
	Terminal,
};

const MEM_SLICE: [u8; 256] = [
	0x02, 0x03, 0x05, 0x07, 0x0B, 0x0D, 0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25, 0x29, 0x2B, 0x2F, 0x35,
	0x3B, 0x3D, 0x43, 0x47, 0x49, 0x4F, 0x53, 0x59, 0x61, 0x65, 0x67, 0x6B, 0x6D, 0x71, 0x7F, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 0x0E, 0x1E,
	0x35, 0x3F, 0xF1, 0x3F, 0xA7, 0x3D, 0x2D, 0x17, 0x91, 0x00, 0xFA, 0x10, 0xF2, 0x02, 0x34, 0x8A,
	0xBE, 0x01, 0xE1, 0x3E, 0x05, 0xA7, 0x3E, 0x25, 0x02, 0x05, 0x43, 0x37, 0x3F, 0x33, 0x42, 0xF0,
	0xFF, 0x43, 0xF0, 0x05, 0x10, 0xF2, 0x00, 0x97, 0x05, 0x2F, 0x1C, 0x10, 0x34, 0x40, 0xBE, 0x01,
	0x09, 0x24, 0x0C, 0xF0, 0x00, 0xA6, 0x3D, 0x10, 0x15, 0xF2, 0x00, 0xE3, 0x00, 0x37, 0x3D, 0xF0,
	0x02, 0xB6, 0x00, 0xE2, 0x00, 0x33, 0x65, 0xBE, 0x01, 0x43, 0x10, 0xF0, 0x00, 0xE2, 0xFF, 0x14,
	0xB7, 0x01, 0x2D, 0x01, 0x43, 0xB4, 0x01, 0x47, 0xFE, 0x33, 0x90, 0x92, 0xFB, 0x34, 0x62, 0x33,
	0x9F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x01, 0x7F, 0x50, 0x71, 0x6E, 0x1E, 0x81, 0x9F, 0x00, 0x00, 0x00, 0x00, 0x9B,
];

fn write_mem(mem: &[u8; 256], out: &mut String) -> Result<()> {
	out.clear();
	out.reserve(256 * 3);

	for val in mem.iter() {
		write!(out, "{:02X} ", val)?;
	}

	Ok(())
}

fn main() -> Result<()> {
	let flisp = Flisp {
		A: 0,
		X: 0,
		Y: 0,
		SP: 0,
		CC: 0,
		PC: 0xFF,
		mem: MEM_SLICE,
	};

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	crossterm::terminal::enable_raw_mode()?;

	let mut memory_text_buffer = String::new();
	let mut register_a_buffer = String::new();
	let mut register_x_buffer = String::new();
	let mut register_y_buffer = String::new();
	let mut register_pc_buffer = String::new();
	let mut register_sp_buffer = String::new();
	let mut register_cc_buffer = String::new();

	loop {
		register_a_buffer.clear();
		register_x_buffer.clear();
		register_y_buffer.clear();
		register_pc_buffer.clear();
		register_sp_buffer.clear();
		register_cc_buffer.clear();

		write_mem(&MEM_SLICE, &mut memory_text_buffer)?;
		write!(&mut register_a_buffer, "0x{:02X}", flisp.A)?;
		write!(&mut register_x_buffer, "0x{:02X}", flisp.X)?;
		write!(&mut register_y_buffer, "0x{:02X}", flisp.Y)?;
		write!(&mut register_pc_buffer, "0x{:02X}", flisp.PC)?;
		write!(&mut register_sp_buffer, "0x{:02X}", flisp.SP)?;
		write!(&mut register_cc_buffer, "0x{:02X}", flisp.CC)?;

		terminal.draw(|f| {
			let control_split = Layout::default()
				.direction(Direction::Vertical)
				.margin(1)
				.constraints([
					Constraint::Min(18),
					Constraint::Min(f.size().height.saturating_sub(23)),
					Constraint::Min(3),
				])
				.split(f.size());
			let ui_split = Layout::default()
				.direction(Direction::Horizontal)
				.constraints(
					[
						Constraint::Min(3 * 16 + 1),
						Constraint::Min(10),
						Constraint::Min(20),
						Constraint::Min(f.size().width.saturating_sub(3 * 16 + 31)),
					]
					.as_ref(),
				)
				.split(control_split[0]);
			let register_split = Layout::default()
				.direction(Direction::Vertical)
				.constraints(
					[
						Constraint::Min(3),
						Constraint::Min(3),
						Constraint::Min(3),
						Constraint::Min(3),
						Constraint::Min(3),
						Constraint::Min(3),
					]
					.as_ref(),
				)
				.split(ui_split[1]);

			let memory_paragraph = Paragraph::new(Span::raw(&memory_text_buffer))
				.block(
					Block::default()
						.title("Memory")
						.borders(Borders::ALL)
						.border_type(BorderType::Rounded),
				)
				.wrap(Wrap { trim: true });
			f.render_widget(memory_paragraph, ui_split[0]);

			let title_text: [(&str, &str); 6] = [
				("A", &register_a_buffer),
				("X", &register_x_buffer),
				("Y", &register_y_buffer),
				("CC", &register_cc_buffer),
				("SP", &register_sp_buffer),
				("PC", &register_pc_buffer),
			];
			for (idx, (title, text)) in title_text.iter().cloned().enumerate() {
				f.render_widget(
					Paragraph::new(Span::raw(text)).block(
						Block::default()
							.borders(Borders::ALL)
							.border_type(BorderType::Rounded)
							.title(title),
					),
					register_split[idx],
				);
			}

			let dis_asm_paragraph = Paragraph::new(Span::raw("")).block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title("Disassembly"),
			);
			f.render_widget(dis_asm_paragraph, ui_split[2]);

			let controls_paragraph = Paragraph::new(Span::raw(
				"Step: [H]    Run: [J]    Faster: [K]    Slower: [L]    Command: [:]",
			))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title("Controls"),
			);
			f.render_widget(controls_paragraph, control_split[2]);
		})?;

		if event::poll(Duration::from_millis(50000))? {
			if let event::Event::Key(key) = event::read()? {
				match key.code {
					event::KeyCode::Char(c) => match c {
						'H' | 'J' | 'K' | 'L' => {}
						':' => break,
						_ => {}
					},
					event::KeyCode::Esc => {
						crossterm::terminal::disable_raw_mode()?;
						return Ok(());
					}
					_ => {}
				}
			}
		}
	}

	println!("Command started?");

	crossterm::terminal::disable_raw_mode()?;
	Ok(())
}
