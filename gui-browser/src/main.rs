#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Read};

use eframe::egui;
use egui::{Context, Id, Ui};
use egui_extras::RetainedImage;

static APP_TITLE: &str = "BeatSaber Browser";

#[derive(Debug)]
struct App {
    current_window: CurrentWindow,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_window: CurrentWindow::Browser(BrowserWindow::default()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum CurrentWindow {
    Browser(BrowserWindow),
    MapDetail,
    Settings,
}

#[derive(Debug, Default)]
struct BrowserWindow {
    side_menu: SideMenu,
}

#[derive(Debug)]
struct SideMenu {
    open: bool,
    open_button_text: String,
    close_button_text: String,
    menu_button_texts: Vec<String>,
}

impl Default for SideMenu {
    fn default() -> Self {
        Self {
            open: false,
            open_button_text: "Menu".to_owned(),
            close_button_text: "Close".to_owned(),
            menu_button_texts: vec!["Maps".to_owned(), "Details".to_owned()],
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(f32::MAX, f32::MAX)),
        ..Default::default()
    };

    eframe::run_native(APP_TITLE, options, Box::new(|_cc| Box::<App>::default()))
}

#[allow(dead_code)]
fn draw_image(ui: &mut Ui, path: String) {
    let mut buffer = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
    let image = RetainedImage::from_svg_bytes("image", &buffer).unwrap();

    image.show(ui);
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match &mut self.current_window {
            CurrentWindow::Browser(window) => {
                egui::SidePanel::left(Id::new("side_menu")).show_animated(
                    ctx,
                    window.side_menu.open,
                    |ui| {
                        draw_settings_bar(ui, &mut window.side_menu);
                    },
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    if !window.side_menu.open
                        && ui.button(&window.side_menu.open_button_text).clicked()
                    {
                        window.side_menu.open = true;
                    }
                });
            }
            CurrentWindow::Settings => {}
            CurrentWindow::MapDetail => {}
        }
    }
}

fn draw_settings_bar(ui: &mut Ui, settings_menu: &mut SideMenu) {
    ui.vertical_centered(|ui| {
        if settings_menu.open && ui.button(&settings_menu.close_button_text).clicked() {
            settings_menu.open = false;
        }

        ui.separator();

        for text in &settings_menu.menu_button_texts {
            ui.label(text);
        }
    });

    // draw_image(ui, "images/settings2-logo.svg".to_string());
}
