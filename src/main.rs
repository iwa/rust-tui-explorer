use std::fs::{self};
use std::path::Path;
use std::{io, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    // let path = ".";

    // println!("--- Calculating file sizes in directory: {} ---", path);

    // let mut size: i64 = 0;

    // match calculate_directory_size(Path::new(path), &mut size) {
    //     Ok(()) => println!("\nTotal size of files: {}", format_size(size.clone())),
    //     Err(e) => eprintln!("\nError reading directory: {}", e),
    // };
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

fn calculate_directory_size(dir: &Path, size: &mut i64) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                calculate_directory_size(&path, size)?;
            } else if path.is_file() {
                let metadata = fs::metadata(&path)?;
                let file_size = metadata.len() as i64;
                *size += file_size;
                println!("File: {:?}, Size: {} bytes", path, file_size);
            }
        }
    }

    Ok(())
}

fn format_size(size: i64) -> String {
    if size < 1024 {
        format!("{} bytes", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[derive(Debug, Default)]
pub struct App {
    path: String,
    tree_dirs: Vec<String>,
    tree_files: Vec<String>,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.path = ".".to_owned();
        self.tree_dirs = vec![];
        self.tree_files = vec![];

        let binding = self.path.clone();
        let value = binding.as_str();
        let dir = Path::new(&value);

        self.generate_filetree(dir)?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.exit(),
            KeyCode::Down => self.exit(),
            KeyCode::Enter => self.exit(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn generate_filetree(&mut self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            let mut directories: Vec<String> = vec![];
            let mut files: Vec<String> = vec![];

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let filename = entry.file_name();

                if entry.path().is_dir() {
                    directories.push(filename.to_string_lossy().to_string());
                    self.tree_dirs.push(filename.to_string_lossy().to_string());
                } else {
                    files.push(filename.to_string_lossy().to_string());
                    self.tree_files.push(filename.to_string_lossy().to_string());
                }
            }

            self.tree_dirs.sort();
            self.tree_files.sort();
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Up ".into(),
            "<Up>".blue().bold(),
            " Down ".into(),
            "<Down>".blue().bold(),
            " Calculate Size ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines: Vec<Line> = vec![];

        self.tree_dirs.iter().for_each(|s| {
            lines.push(Line::from(s.as_str()).yellow().bold());
        });

        self.tree_files.iter().for_each(|s| {
            lines.push(Line::from(s.as_str()).green());
        });

        let text = Text::from(lines);

        Paragraph::new(text).block(block).render(area, buf);
    }
}
