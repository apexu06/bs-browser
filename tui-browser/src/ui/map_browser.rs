use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    ExecutableCommand,
};
use indicatif::{ProgressBar, ProgressStyle};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::{api::fetch_beatsaver::fetch_maps, types::map_types::Map, utils::loading::Loading};

use super::map_detail;

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
    Sorting(SortMode),
}

#[derive(PartialEq)]
enum SortMode {
    Normal,
    Filtering,
}

pub struct Browser {
    query: String,
    input: String,
    results: Vec<Map>,
    filtered_results: Vec<Map>,
    input_mode: InputMode,
    table_state: TableState,
    page_index: i32,
}

impl Browser {
    fn default() -> Browser {
        Browser {
            query: String::new(),
            input: String::new(),
            results: Vec::new(),
            filtered_results: Vec::new(),
            input_mode: InputMode::Normal,
            table_state: TableState::default(),
            page_index: 1,
        }
    }

    fn set_results(&mut self, new_results: Vec<Map>) {
        self.results = new_results.clone();
        self.filtered_results = new_results;
    }

    fn append_results(&mut self, new_results: &mut Vec<Map>) {
        let mut data = new_results.clone();
        self.results.append(new_results);
        self.filtered_results.append(&mut data);
    }

    fn next_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.filtered_results.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_results.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

fn error_component<B: Backend>(frame: &mut Frame<B>) -> Rect {
    let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let columns = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(rows[1]);

    (Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(50)].as_ref())
        .split(columns[1]))[1]
}

pub async fn start_browser<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let mut browser = Browser::default();
    loop {
        terminal.draw(|frame| draw_browser(frame, &mut browser))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match browser.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('s') => {
                            browser.input_mode = InputMode::Editing;
                            browser.input.clear();
                        }
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('S') => {
                            browser.input_mode = InputMode::Sorting(SortMode::Normal);
                            browser.input.clear();
                        }
                        KeyCode::Char('F') => {
                            if browser.filtered_results.len() == 0 {
                                continue;
                            }

                            let mut spinner = Loading::new(terminal);
                            spinner.start();

                            let request = fetch_maps(&browser.query, browser.page_index).await;

                            match request {
                                Ok(mut data) => {
                                    browser.append_results(&mut data);
                                    browser.page_index += 1;
                                }
                                Err(e) => show_error(terminal, e.to_string()),
                            }
                        }
                        KeyCode::Char('c') => browser.set_results(Vec::new()),
                        KeyCode::Down => browser.next_item(),
                        KeyCode::Up => browser.previous_item(),
                        KeyCode::Enter => {
                            if browser.filtered_results.len() == 0 {
                                continue;
                            }
                            let selected = browser.table_state.selected().unwrap_or(0);
                            match map_detail::start_details(
                                terminal,
                                &browser.filtered_results[selected].id,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => show_error(terminal, e.to_string()),
                            }
                        }
                        _ => {}
                    },

                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            browser.query = browser.input.clone();
                            browser.input_mode = InputMode::Normal;

                            let mut spinner = Loading::new(terminal);
                            spinner.start();

                            let request = fetch_maps(&browser.query, 0).await;

                            match request {
                                Ok(results) => {
                                    browser.set_results(results);
                                    browser.page_index = 1;
                                }
                                Err(e) => show_error(terminal, e.to_string()),
                            };
                        }
                        KeyCode::Char(c) => {
                            browser.input.push(c);
                        }
                        KeyCode::Backspace => {
                            browser.input.pop();
                        }
                        KeyCode::Esc => browser.input_mode = InputMode::Normal,
                        _ => {}
                    },

                    InputMode::Sorting(SortMode::Normal) => match key.code {
                        KeyCode::Esc => {
                            browser.input_mode = InputMode::Normal;
                            browser.filtered_results = browser.results.clone();
                        }
                        KeyCode::Char('I') => sort_results(&mut browser, "id"),
                        KeyCode::Char('N') => sort_results(&mut browser, "song_name"),
                        KeyCode::Char('A') => sort_results(&mut browser, "author"),
                        KeyCode::Char('D') => sort_results(&mut browser, "date"),
                        KeyCode::Char('f') => {
                            browser.input_mode = InputMode::Sorting(SortMode::Filtering);
                            browser.input.clear();
                        }
                        KeyCode::Down => browser.next_item(),
                        KeyCode::Up => browser.previous_item(),
                        KeyCode::Enter => {
                            if browser.filtered_results.len() == 0 {
                                continue;
                            }
                            let selected = browser.table_state.selected().unwrap_or(0);
                            match map_detail::start_details(
                                terminal,
                                &browser.filtered_results[selected].id,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => show_error(terminal, e.to_string()),
                            }
                        }
                        _ => {}
                    },
                    InputMode::Sorting(SortMode::Filtering) => match key.code {
                        KeyCode::Esc => {
                            browser.input_mode = InputMode::Sorting(SortMode::Normal);
                            browser.filtered_results = browser.results.clone();
                        }
                        KeyCode::Char(c) => {
                            browser.input.push(c);
                            let input = browser.input.clone();
                            filter_results(&mut browser, &input)
                        }
                        KeyCode::Backspace => {
                            browser.input.pop();
                            let input = browser.input.clone();
                            filter_results(&mut browser, &input)
                        }
                        KeyCode::Enter => {
                            browser.input_mode = InputMode::Sorting(SortMode::Normal);
                        }

                        _ => {}
                    },
                }
            }
        }
    }
}

fn draw_browser<B: Backend>(frame: &mut Frame<B>, browser: &mut Browser) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let top_text_bar = match browser.input_mode {
        InputMode::Normal => {
            vec![
                Span::raw("Exit(q) "),
                Span::raw("Search(s) "),
                Span::raw("Sort(S) "),
                Span::raw(if browser.results.len() != 0 {
                    "Fetch more(F) "
                } else {
                    ""
                }),
                Span::raw("Clear(c)"),
            ]
        }
        InputMode::Editing => vec![Span::raw("Go Back(Esc) "), Span::raw("Search(Enter)")],
        InputMode::Sorting(SortMode::Normal) => {
            vec![
                Span::raw("Go Back(Esc) Filter(frame) | "),
                Span::styled("Sort by: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("ID(I) Song name(N) Artist(A) Date(D)"),
            ]
        }
        InputMode::Sorting(SortMode::Filtering) => {
            vec![Span::raw("Go Back(Esc) "), Span::raw("Confirm(Enter)")]
        }
    };

    frame.render_widget(
        Paragraph::new(Text::from(Spans::from(top_text_bar))),
        chunks[0],
    );

    let input = Paragraph::new(browser.input.as_ref())
        .style(match browser.input_mode {
            InputMode::Sorting(SortMode::Filtering) => Style::default().fg(Color::Magenta),
            InputMode::Editing => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title(
            if browser.input_mode == InputMode::Sorting(SortMode::Normal) {
                "Filter"
            } else {
                "Search"
            },
        ));

    frame.render_widget(input, chunks[1]);

    match browser.input_mode {
        InputMode::Sorting(SortMode::Filtering) => frame.set_cursor(
            chunks[1].x + browser.input.width() as u16 + 1,
            chunks[1].y + 1,
        ),
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + browser.input.width() as u16 + 1,
            chunks[1].y + 1,
        ),
        _ => {}
    }

    frame.render_stateful_widget(
        display_maps(&browser.filtered_results),
        chunks[2],
        &mut browser.table_state,
    );
}

fn show_error<B: Backend>(terminal: &mut Terminal<B>, error: String) {
    terminal.clear().unwrap();
    let text = vec![
        Spans::from(Span::raw(error)),
        Spans::from(Span::styled(
            "press any key to continue",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(Text::from(text))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    terminal
        .draw(|frame| {
            let error_rect = error_component(frame);
            frame.render_widget(paragraph, error_rect);
        })
        .unwrap();
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                break;
            }
        }
    }
}

fn sort_results(browser: &mut Browser, criteria: &str) {
    match criteria {
        "id" => browser.filtered_results.sort_by(|a, b| a.id.cmp(&b.id)),
        "song_name" => browser.filtered_results.sort_by(|a, b| {
            a.metadata
                .song_name
                .to_ascii_lowercase()
                .cmp(&b.metadata.song_name.to_ascii_lowercase())
        }),
        "author" => browser.filtered_results.sort_by(|a, b| {
            a.metadata
                .song_author_name
                .to_ascii_lowercase()
                .cmp(&b.metadata.song_author_name.to_ascii_lowercase())
        }),
        "date" => browser
            .filtered_results
            .sort_by(|a, b| a.last_published_at.cmp(&b.last_published_at)),
        _ => {}
    }
}

fn filter_results(browser: &mut Browser, filter: &str) {
    let mut filtered_results = Vec::new();

    let lines: Vec<(String, Map)> = browser
        .results
        .clone()
        .into_iter()
        .map(|m| {
            (
                format!(
                    "{} {} {} {} {}",
                    m.id,
                    m.metadata.song_name,
                    m.metadata.song_author_name,
                    m.metadata.level_author_name,
                    m.last_published_at
                ),
                m,
            )
        })
        .collect();

    for (line, map) in lines {
        if line.to_lowercase().contains(&filter.to_lowercase()) {
            filtered_results.push(map);
        }
    }
    browser.filtered_results = filtered_results;
}

fn display_maps(maps: &Vec<Map>) -> Table<'static> {
    let header = Row::new(vec![
        Cell::from("ID"),
        Cell::from("SONG NAME"),
        Cell::from("SONG AUTHOR"),
        Cell::from("LEVEL AUTHOR"),
        Cell::from("DATE"),
    ])
    .style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    );

    let rows: Vec<Row> = maps
        .iter()
        .map(|m| {
            Row::new(vec![
                Cell::from(format!("{}", m.id)),
                Cell::from(format!("{}", m.metadata.song_name)),
                Cell::from(format!("{}", m.metadata.song_author_name)),
                Cell::from(format!("{}", m.metadata.level_author_name)),
                Cell::from(format!(
                    "{}",
                    m.last_published_at.split("T").next().unwrap_or("")
                )),
            ])
        })
        .collect();

    Table::new(rows)
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .header(header)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(Block::default().borders(Borders::ALL).title("Maps"))
}
