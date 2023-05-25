#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Read};

use eframe::egui;
use egui::{ColorImage, Context, Id, TextureHandle, Ui};
use egui_extras::RetainedImage;

enum CurrentWindow {
    Browser,
    MapDetail,
    Settings,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(f32::MAX, f32::MAX)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

struct App {
    current_window: CurrentWindow,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_window: CurrentWindow::Browser,
        }
    }
}

struct Image {
    texture: TextureHandle,
}

fn draw_image(ui: &mut Ui, path: String) {
    let mut buffer = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
    let image = RetainedImage::from_svg_bytes("image", &buffer).unwrap();

    image.show(ui);
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.current_window {
            CurrentWindow::Browser => {
                egui::SidePanel::left(Id::new("settings")).show(ctx, |ui| {
                    draw_settings_bar(self, ui, ctx);
                });
            }

            CurrentWindow::Settings => {}

            CurrentWindow::MapDetail => {}
        }
    }
}

fn draw_settings_bar(app: &mut App, ui: &mut Ui, ctx: &Context) {
    ui.heading("Settings");
    ui.separator();
    ui.vertical_centered(|ui| {
        ui.label("Map");
        ui.separator();
        ui.label("Detail");
    });

    draw_image(ui, "images/settings2-logo.svg".to_string());
}
