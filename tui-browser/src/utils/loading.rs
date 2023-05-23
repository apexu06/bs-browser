use std::{io::stdout, time::Duration};

use crossterm::{cursor::MoveTo, ExecutableCommand};
use indicatif::{ProgressBar, ProgressStyle};
use tui::{backend::Backend, Terminal};

pub struct Loading<'a, B: Backend> {
    spinner: ProgressBar,
    terminal: &'a mut Terminal<B>,
}

impl<'a, B: Backend> Loading<'a, B> {
    pub fn new(terminal: &'a mut Terminal<B>) -> Self {
        let spinner = ProgressBar::new_spinner();

        spinner.set_message(" ".repeat((terminal.size().unwrap().width / 2) as usize));
        spinner.enable_steady_tick(Duration::from_millis(100));
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⣷⣯⣟⡿⢿⣻⣽⣾")
                .template("{msg} {spinner}")
                .unwrap(),
        );
        Self { spinner, terminal }
    }

    pub fn start(&mut self) {
        let height = self.terminal.size().unwrap().height / 2 - 3;
        stdout().execute(MoveTo(0, height)).unwrap();

        self.terminal.clear().unwrap();
        self.spinner.enable_steady_tick(Duration::from_millis(100));
    }

    pub fn stop(&mut self) {
        self.spinner.finish_and_clear();
    }
}
