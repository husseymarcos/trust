use crate::context::RunContext;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io::{self, Stdout};
use std::path::PathBuf;

pub fn run(ctx: &RunContext, initial_query: Option<String>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    std::fs::create_dir_all(&ctx.root)?;
    let entries = list_entries(&ctx.root)?;
    let mut app = App {
        _root: ctx.root.clone(),
        entries,
        selected: 0,
        query: initial_query.unwrap_or_default(),
        scroll: 0,
    };
    app.clamp_selection();
    let result = event_loop(&mut terminal, &mut app, ctx);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    Ok(ratatui::Terminal::new(backend)?)
}

fn restore_terminal(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>,
) -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn list_entries(root: &std::path::Path) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(root)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .collect();
    entries.sort_by(|a, b| b.cmp(a));
    Ok(entries)
}

struct App {
    _root: PathBuf,
    entries: Vec<PathBuf>,
    selected: usize,
    query: String,
    scroll: usize,
}

impl App {
    fn clamp_selection(&mut self) {
        let len = self.entries.len().max(1);
        self.selected = self.selected.min(len.saturating_sub(1));
    }

    fn ensure_selection_visible(&mut self, list_height: usize) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + list_height {
            self.scroll = self.selected - list_height + 1;
        }
    }
}

fn event_loop(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>,
    app: &mut App,
    ctx: &RunContext,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break Ok(()),
                    (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        app.selected = app.selected.saturating_sub(1);
                        app.clamp_selection();
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        if app.selected + 1 < app.entries.len() {
                            app.selected += 1;
                        }
                        app.clamp_selection();
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(path) = app.entries.get(app.selected) {
                            ctx.print_cd(path);
                        }
                        break Ok(());
                    }
                    (KeyCode::Backspace, _) => {
                        app.query.pop();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE) if !c.is_control() => {
                        app.query.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(area);

    let list_area = chunks[1];
    app.ensure_selection_visible(list_area.height as usize);

    let header_block = Block::default().borders(Borders::ALL).title(" Try Selector ");
    let header_inner = header_block.inner(chunks[0]);
    let header_lines = vec![
        Line::from("📁 Try Selector"),
        Line::from("─".repeat(header_inner.width as usize)),
        Line::from(format!("> {}", app.query)),
    ];
    f.render_widget(
        Paragraph::new(header_lines).block(header_block),
        chunks[0],
    );

    let list_height = list_area.height as usize;
    let start = app.scroll;
    let end = (app.scroll + list_height).min(app.entries.len());
    let lines: Vec<Line> = app.entries[start..end]
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name: String = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let idx = start + i;
            let (prefix, style) = if idx == app.selected {
                ("→ ", Style::default().add_modifier(Modifier::BOLD))
            } else {
                ("  ", Style::default())
            };
            Line::from(vec![
                Span::styled(prefix, style),
                Span::styled("📁 ", style),
                Span::styled(name, style),
            ])
        })
        .collect();
    let list = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    f.render_widget(list, list_area);

    let footer = Paragraph::new("↑↓: Navigate  Enter: Select  Ctrl-D: Delete  Esc: Cancel")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
