#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Read};

use eframe::{egui, CreationContext};
use egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, Id, ImageButton, RichText, TextStyle,
    Ui,
};
use egui_extras::{image::FitTo, RetainedImage};

static APP_TITLE: &str = "BeatSaber Browser";

#[derive(Debug)]
struct App {
    current_window: CurrentWindow,
}

impl App {
    fn new(cctx: &CreationContext<'_>) -> Self {
        setup_custom_font(&cctx.egui_ctx);
        Self {
            current_window: CurrentWindow::Browser(BrowserWindow::default()),
        }
    }
}

fn setup_custom_font(ctx: &egui::Context) {
    use egui::TextStyle::*;
    use FontFamily::Proportional;

    let mut font = FontDefinitions::default();
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (Heading, FontId::new(40.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Small, FontId::new(12.0, Proportional)),
        (Button, FontId::new(18.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);

    font.font_data.insert(
        "rubik_regular".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Rubik-Regular.ttf")),
    );

    font.families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "rubik_regular".to_owned());

    ctx.set_fonts(font);
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
    close_button_text: String,
    menu_button_texts: Vec<String>,
}

impl Default for SideMenu {
    fn default() -> Self {
        Self {
            open: false,
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

    eframe::run_native(APP_TITLE, options, Box::new(|cc| Box::new(App::new(cc))))
}

#[allow(dead_code)]
fn create_svg(file_name: &str, size: (u32, u32)) -> Result<RetainedImage, String> {
    let path = "assets/images/".to_owned() + &file_name;
    let id = "image";

    let mut buffer = Vec::new();
    let mut file = File::open(path).map_err(|_e| id)?;
    file.read_to_end(&mut buffer).map_err(|_e| id)?;

    return RetainedImage::from_svg_bytes_with_size(id, &buffer, FitTo::Size(size.0, size.1));
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
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            RichText::new("BeatSaber Browser").text_style(TextStyle::Heading),
                        );
                    });
                    let settings_img = create_svg("settings-logo.svg", (20, 20)).unwrap();
                    let button = ImageButton::new(settings_img.texture_id(ctx), (20.0, 20.0));

                    if !window.side_menu.open && ui.add(button).clicked() {
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
}
