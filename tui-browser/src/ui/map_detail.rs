use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};

use std::{error::Error, io, time::Duration};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};

use crate::{
    api::fetch_beatsaver::fetch_map_details,
    types::{
        leaderboard::{LeaderBoardInfo, Score},
        map_types::MapDifficulty,
    },
    utils::loading::Loading,
};
use crate::{api::fetch_scoresaber::fetch_leaderboard, types::map_types::Map};
use crate::{api::fetch_scoresaber::fetch_leaderboard_info, utils::preview_player::PreviewState};

use crate::utils::preview_player::Preview;

#[derive(PartialEq)]
pub enum MapDetailActiveWindow {
    Difficulties,
    Leaderboard,
}

struct MapDetail {
    description_height: u16,
    description_expanded: bool,
    scoreboard_shown: bool,
    scoreboard_width: u16,
    map: Map,
    active_window: MapDetailActiveWindow,
}

struct DifficultyTable {
    table_state: TableState,
    difficulties: Vec<MapDifficulty>,
}

struct SSLeaderboard {
    table_state: TableState,
    scores: Vec<Score>,
    leaderboard_diffs: Vec<LeaderBoardInfo>,

    current_leaderboard_index: usize,
}

impl MapDetail {
    async fn new(bsr: &String) -> Result<MapDetail, Box<dyn Error>> {
        let response = fetch_map_details(bsr).await;

        match response {
            Ok(map) => Ok(MapDetail {
                description_height: 50,
                description_expanded: false,
                scoreboard_shown: true,
                scoreboard_width: 50,
                map,
                active_window: MapDetailActiveWindow::Difficulties,
            }),
            Err(_) => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to fetch map",
            ))),
        }
    }

    fn toggle_description(&mut self) {
        if self.description_expanded {
            self.description_height = 50;
        } else {
            self.description_height = 100;
        }
        self.description_expanded = !self.description_expanded;
    }

    fn toggle_scoreboard(&mut self) {
        if self.scoreboard_shown {
            self.scoreboard_width = 50;
        } else {
            self.scoreboard_width = 0;
        }
        self.scoreboard_shown = !self.scoreboard_shown;
    }
}

impl DifficultyTable {
    fn new(diffs: Vec<MapDifficulty>) -> Self {
        Self {
            table_state: TableState::default(),
            difficulties: diffs,
        }
    }

    fn next_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.difficulties.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.difficulties.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

impl SSLeaderboard {
    async fn new(map: &Map) -> Result<Self, Box<dyn Error>> {
        let mut leaderboard_diffs = Vec::new();

        for diff in &map.versions[0].diffs {
            if diff.characteristic == "Lightshow" {
                leaderboard_diffs.push(LeaderBoardInfo::new());
                continue;
            }

            let diff_id = get_diff_id(&diff.difficulty);

            let response = fetch_leaderboard_info(
                &map.versions[0].hash,
                diff_id,
                &("Solo".to_owned() + &diff.characteristic),
            )
            .await;

            match response {
                Ok(info) => {
                    leaderboard_diffs.push(info);
                }
                Err(_) => {
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to fetch leaderboard {:?}", &diff.characteristic),
                    )))
                }
            }
        }

        let response = fetch_leaderboard(leaderboard_diffs[0].id as u32, 0).await;

        match response {
            Ok(scores) => Ok(Self {
                table_state: TableState::default(),
                scores,
                leaderboard_diffs,
                current_leaderboard_index: 0,
            }),
            Err(_) => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to fetch scores"),
                )))
            }
        }
    }

    fn next_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.scores.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_item(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.scores.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub async fn get_scores_for_difficulty(&mut self, index: usize) {
        let diff_id = self.leaderboard_diffs[index].id as u32;
        let response = fetch_leaderboard(diff_id, 0).await;

        match response {
            Ok(scores) => {
                self.scores = scores;
            }
            Err(_) => {
                self.scores = Vec::new();
            }
        }
    }

    pub async fn append_scores_for_difficulty(&mut self, index: usize, page: u32) {
        let diff_id = self.leaderboard_diffs[index].id as u32;
        let response = fetch_leaderboard(diff_id, page).await;

        match response {
            Ok(mut scores) => {
                self.scores.append(&mut scores);
            }
            Err(_) => {}
        }
    }
}

fn get_difficulty_color(difficulty: &String) -> Color {
    match difficulty.as_str() {
        "Easy" => Color::Green,
        "Normal" => Color::Blue,
        "Hard" => Color::Rgb(255, 99, 71),
        "Expert" => Color::Red,
        "ExpertPlus" => Color::Magenta,
        _ => Color::White,
    }
}

fn get_diff_id(diff: &str) -> u8 {
    return match diff {
        "Easy" => 1,
        "Normal" => 3,
        "Hard" => 5,
        "Expert" => 7,
        "ExpertPlus" => 9,
        _ => 0,
    };
}

fn get_diff_name(diff_id: u8) -> String {
    match diff_id {
        1 => "Easy",
        3 => "Normal",
        5 => "Hard",
        7 => "Expert",
        9 => "Expert+",
        _ => "Unknown",
    }
    .to_owned()
}

pub fn truncate(s: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return s;
    }
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub async fn start_details<B: Backend>(
    terminal: &mut Terminal<B>,
    id: &String,
) -> Result<(), io::Error> {
    let mut spinner = Loading::new(terminal);

    spinner.start();

    let mut map_detail = match MapDetail::new(id).await {
        Ok(map_detail) => map_detail,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };

    let mut difficulty_table = DifficultyTable::new(map_detail.map.versions[0].diffs.clone());

    let mut preview = match Preview::new(&map_detail.map.versions[0].preview_url).await {
        Ok(preview) => preview,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };

    let mut leaderboard = match SSLeaderboard::new(&map_detail.map).await {
        Ok(leaderboard) => leaderboard,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };

    spinner.stop();

    let mut selected = 0;
    let mut current_leaderboard_page = 1;

    loop {
        terminal.draw(|rect| {
            draw_details(
                rect,
                &mut preview,
                &mut map_detail,
                &mut leaderboard,
                &mut difficulty_table,
            )
        })?;

        if poll(Duration::from_millis(256)).unwrap_or(false) {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match preview.state {
                        PreviewState::Paused => match key.code {
                            KeyCode::Char('r') => preview.resume(),
                            _ => {}
                        },

                        PreviewState::Playing => match key.code {
                            KeyCode::Char('s') => preview.stop(),
                            KeyCode::Char('P') => preview.pause(),
                            KeyCode::Char('i') => preview.inc_vol(),
                            KeyCode::Char('d') => preview.dec_vol(),
                            _ => {}
                        },
                        PreviewState::Stopped => match key.code {
                            KeyCode::Char('p') => preview.play(),
                            KeyCode::Char('e') => map_detail.toggle_description(),
                            KeyCode::Char('S') => map_detail.toggle_scoreboard(),
                            KeyCode::Esc => return Ok(()),

                            KeyCode::Enter => {
                                if map_detail.active_window == MapDetailActiveWindow::Leaderboard {
                                    continue;
                                }

                                let mut spinner = Loading::new(terminal);
                                spinner.start();

                                selected = difficulty_table.table_state.selected().unwrap_or(0);
                                leaderboard.current_leaderboard_index = selected;
                                leaderboard.get_scores_for_difficulty(selected).await;
                                current_leaderboard_page = 1;

                                spinner.stop();
                            }
                            KeyCode::Char('F') => {
                                current_leaderboard_page += 1;

                                let mut spinner = Loading::new(terminal);
                                spinner.start();
                                leaderboard
                                    .append_scores_for_difficulty(
                                        selected,
                                        current_leaderboard_page,
                                    )
                                    .await;
                                spinner.stop();
                            }

                            _ => match map_detail.active_window {
                                MapDetailActiveWindow::Difficulties => match key.code {
                                    KeyCode::Up => difficulty_table.previous_item(),
                                    KeyCode::Down => difficulty_table.next_item(),
                                    KeyCode::Right => {
                                        map_detail.active_window =
                                            MapDetailActiveWindow::Leaderboard;
                                        difficulty_table.table_state.select(None);
                                        leaderboard.table_state.select(Some(0));
                                    }
                                    _ => {}
                                },
                                MapDetailActiveWindow::Leaderboard => match key.code {
                                    KeyCode::Up => leaderboard.previous_item(),
                                    KeyCode::Down => leaderboard.next_item(),
                                    KeyCode::Left => {
                                        map_detail.active_window =
                                            MapDetailActiveWindow::Difficulties;
                                        leaderboard.table_state.select(None);
                                        difficulty_table.table_state.select(Some(0));
                                    }

                                    _ => {}
                                },
                            },
                        },
                    }
                }
            }
        }
        if preview.sink.empty() {
            preview.stop();
        }
    }
}

fn draw_details<B: Backend>(
    frame: &mut Frame<B>,
    preview: &mut Preview,
    map_detail: &mut MapDetail,
    leaderboard: &mut SSLeaderboard,
    difficulty_table: &mut DifficultyTable,
) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Percentage(map_detail.scoreboard_width),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let left_boxes = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Percentage(map_detail.description_height),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(columns[0]);

    let top_text = Spans::from(Span::raw(match preview.state {
        PreviewState::Playing => "Pause(P) Stop(s) Increase Volume(i) Decrease Volume(d)",
        PreviewState::Paused => "Resume(r)",
        PreviewState::Stopped => {
            "Close(Esc) Toggle Scoreboard(S) Play Preview(p) Fetch more scores(F)"
        }
    }));

    frame.render_widget(Paragraph::new(Text::from(top_text)), left_boxes[0]);
    draw_leaderboard(frame, leaderboard, columns[1]);

    draw_top_left_box(frame, &map_detail.map, left_boxes[1]);
    draw_bottom_left_box(
        frame,
        map_detail,
        difficulty_table,
        &leaderboard,
        left_boxes[2],
    );
}

fn draw_leaderboard<B: Backend>(
    frame: &mut Frame<B>,
    ssleaderboard: &mut SSLeaderboard,
    right_column: Rect,
) {
    let scores = &ssleaderboard.scores;
    let leaderboard = &ssleaderboard.leaderboard_diffs[ssleaderboard.current_leaderboard_index];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(right_column);

    let header = Row::new(vec![
        Cell::from("RANK"),
        Cell::from("NAME"),
        Cell::from("ACC"),
        Cell::from("PP"),
        Cell::from("SCORE"),
        Cell::from("MISSES"),
    ])
    .style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    );

    let rows: Vec<Row> = scores
        .iter()
        .map(|score| {
            Row::new(vec![
                Cell::from(format!("{}", score.rank)),
                Cell::from(format!("{}", score.leaderboard_player_info.name)),
                Cell::from(format!(
                    "{:.2}%",
                    (score.base_score as f32 / leaderboard.max_score as f32) * 100.0
                )),
                Cell::from(format!("{:.2}", score.pp)),
                Cell::from(format!("{}", score.base_score)),
                Cell::from(format!("{}", score.bad_cuts + score.missed_notes)),
            ])
        })
        .collect();

    let diff_name = get_diff_name(leaderboard.difficulty.difficulty);
    let title = &format!(
        "Leaderboard - {} - {}",
        diff_name, leaderboard.difficulty.game_mode
    );

    let table = Table::new(rows)
        .header(header)
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(40),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(20),
        ])
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if scores.is_empty() {
                    "No scores to display"
                } else {
                    title
                }),
        );

    frame.render_stateful_widget(table, chunks[0], &mut ssleaderboard.table_state);
}

fn draw_bottom_left_box<B: Backend>(
    frame: &mut Frame<B>,
    map_detail: &mut MapDetail,
    difficulties_table: &mut DifficultyTable,
    leaderboard: &SSLeaderboard,
    bottom_box: Rect,
) {
    let map = &map_detail.map;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1)].as_ref())
        .split(bottom_box);

    let links = vec![
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Download"),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::raw(&map.versions[0].download_url),
        ]),
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Cover"),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::raw(&map.versions[0].cover_url),
        ]),
    ];

    let links = Paragraph::new(links)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Links"));

    frame.render_widget(links, chunks[0]);

    let header = Row::new(vec![
        "DIFF", "MODE", "NJS", "NPS", "NOTES", "BOMBS", "STARS",
    ])
    .height(1)
    .style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    );

    let rows: Vec<Row> = map_detail.map.versions[0]
        .diffs
        .iter()
        .enumerate()
        .map(|(i, diff)| {
            Row::new(vec![
                Cell::from(Span::raw(diff.difficulty.to_string())),
                Cell::from(Span::raw(diff.characteristic.to_string())),
                Cell::from(Span::raw(diff.njs.to_string())),
                Cell::from(Span::raw(format!("{:.2}", diff.nps))),
                Cell::from(Span::raw(diff.notes.to_string())),
                Cell::from(Span::raw(diff.bombs.to_string())),
                Cell::from(Span::raw(if leaderboard.leaderboard_diffs[i].id == 0 {
                    "N/A".to_owned()
                } else {
                    leaderboard.leaderboard_diffs[i].stars.to_string()
                })),
            ])
            .style(Style::default().fg(get_difficulty_color(&diff.difficulty)))
        })
        .collect();

    let table = Table::new(rows)
        .widths(&[Constraint::Percentage(100 / 7); 7])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Difficulties"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(table, chunks[1], &mut difficulties_table.table_state);
}

fn draw_top_left_box<B: Backend>(frame: &mut Frame<B>, map: &Map, top_box: Rect) {
    let song_name_text = Text::from(Span::styled(
        format!(
            "  {} {}",
            &map.metadata.song_name, &map.metadata.song_sub_name
        ),
        Style::default()
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD),
    ));

    let top_left_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(top_box);

    let song_name = Paragraph::new(song_name_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Song Name"));

    frame.render_widget(song_name, top_left_box[0]);

    let top_left_middle_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(top_left_box[1]);

    let box_width = if top_left_middle_box[0].width <= 25 {
        0
    } else {
        (top_left_middle_box[0].width as usize / 2 - 3)
            + ((top_left_middle_box[0].width as usize / 2 - 3) - 10)
    };

    let song_info = vec![
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Artist"),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!(
                "{: >box_width$}",
                truncate(&map.metadata.song_author_name, box_width),
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Mapper"),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!(
                "{: >box_width$}",
                truncate(&map.metadata.level_author_name, box_width),
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "BPM"),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!("{: >box_width$}", &map.metadata.bpm,)),
        ]),
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Published"),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!(
                "{: >box_width$}",
                truncate(
                    &map.last_published_at.split("T").collect::<Vec<&str>>()[0],
                    box_width
                ),
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                format!("{: <10} -> ", "Duration"),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!(
                "{: >box_width$}",
                (&map.metadata.duration / 60).to_string()
                    + ":"
                    + &(&map.metadata.duration % 60).to_string()
            )),
        ]),
    ];

    let song_info = Paragraph::new(song_info)
        .block(Block::default().borders(Borders::ALL).title("Song Info"))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(song_info, top_left_middle_box[0]);

    let stats_width = if top_left_middle_box[1].width <= 10 {
        0
    } else {
        (top_left_middle_box[1].width as usize / 2) - 3
    };

    let stats = vec![
        Spans::from(Span::raw(format!(
            "{: <stats_width$} -> {: >stats_width$}",
            "ranked", &map.ranked
        ))),
        Spans::from(Span::raw(format!(
            "{: <stats_width$} -> {: >stats_width$}",
            "qualified", &map.qualified
        ))),
        Spans::from(Span::raw(format!(
            "{: <stats_width$} -> {: >stats_width$}",
            "Automapper", &map.automapper
        ))),
        Spans::from(Span::raw(format!(
            "{: <stats_width$} -> {: >stats_width$}",
            "Upvotes", &map.stats.upvotes
        ))),
        Spans::from(Span::raw(format!(
            "{: <stats_width$} -> {: >stats_width$}",
            "Downvotes", &map.stats.downvotes
        ))),
    ];

    let stats = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .wrap(Wrap { trim: true });

    frame.render_widget(stats, top_left_middle_box[1]);

    let desc = Paragraph::new(map.description.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description (e to expand)"),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(desc, top_left_box[2]);
}
