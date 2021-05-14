use anyhow::Result;
use crossterm::{
	event, execute,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use flisp_lib::processor::Flisp;
use std::{fmt::Write as fmtWrite, io, time::Duration};
use tui::{
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout},
	text::Span,
	widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table},
	Terminal,
};

const MEM_SLICE: [u8; 256] = [
	0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
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
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9B,
];

fn write_mem(mem: &[u8; 256], out: &mut String) -> Result<()> {
	out.clear();
	out.reserve(256 * 3);

	for line in mem.chunks(16) {
		for val in line.iter() {
			write!(out, "{:02X} ", val)?;
		}
		out.push('\n');
	}

	Ok(())
}

fn main() -> Result<()> {
	execute!(io::stdout(), EnterAlternateScreen)?;
	let mut flisp = Flisp {
		A: 0,
		X: 0,
		Y: 0,
		SP: 0,
		CC: 0,
		PC: 0xFF,
		mem: MEM_SLICE,
	};
	flisp.PC = flisp.mem[flisp.PC as usize];

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

	let mut dis_asm_buffer = String::new();

	'drawing_loop: loop {
		register_a_buffer.clear();
		register_x_buffer.clear();
		register_y_buffer.clear();
		register_pc_buffer.clear();
		register_sp_buffer.clear();
		register_cc_buffer.clear();
		dis_asm_buffer.clear();

		write_mem(&flisp.mem, &mut memory_text_buffer)?;
		write!(&mut register_a_buffer, "0x{:02X}", flisp.A)?;
		write!(&mut register_x_buffer, "0x{:02X}", flisp.X)?;
		write!(&mut register_y_buffer, "0x{:02X}", flisp.Y)?;
		write!(&mut register_pc_buffer, "0x{:02X}", flisp.PC)?;
		write!(&mut register_sp_buffer, "0x{:02X}", flisp.SP)?;
		write!(&mut register_cc_buffer, "0b {:05b}", flisp.CC)?;

		let mut idx = flisp.PC;
		loop {
			let next = flisp.print_disassembly(&mut dis_asm_buffer, idx)?;
			dis_asm_buffer.push('\n');
			if next < idx {
				break;
			}
			idx = next;
		}
		let items = dis_asm_buffer
			.lines()
			.map(ListItem::new)
			.collect::<Vec<_>>();

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

			let widths = vec![Constraint::Min(2); 16];
			let memory_table = Table::new(
				memory_text_buffer
					.lines()
					.map(|line| Row::new(line.trim().split_ascii_whitespace())),
			)
			.block(
				Block::default()
					.title("Memory")
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded),
			)
			.widths(&widths);
			f.render_widget(memory_table, ui_split[0]);

			let title_text: [(&str, &str); 6] = [
				("A", &register_a_buffer),
				("X", &register_x_buffer),
				("Y", &register_y_buffer),
				("CC─INZVC", &register_cc_buffer),
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

			let dis_asm_list = List::new(items).block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title("Disassembly"),
			);
			f.render_widget(dis_asm_list, ui_split[2]);

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

		if event::poll(Duration::from_millis(50))? {
			if let event::Event::Key(key) = event::read()? {
				match key.code {
					event::KeyCode::Char(c) => match c {
						'h' => {
							flisp.step();
						}
						'j' | 'k' | 'l' => {}
						':' => break,
						_ => {}
					},
					event::KeyCode::Esc => {
						break 'drawing_loop;
					}
					_ => {}
				}
			}
		}
	}

	println!("Command started?");

	crossterm::terminal::disable_raw_mode()?;
	execute!(io::stdout(), LeaveAlternateScreen)?;
	Ok(())
}
