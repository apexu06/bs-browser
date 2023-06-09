#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::HashMap, fmt::Display, format, fs::File, io::Read, todo};

use eframe::{egui, CreationContext};
use egui::{
    Button, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Id, ImageButton,
    RichText, TextStyle, Ui,
};
use egui_extras::{image::FitTo, RetainedImage};
use lazy_static::lazy_static;
use utils::ui::{build_widget::BuildWidget, button::AppButton};

mod utils;

// add to App struct if we want to be able to change the font at runtime
static FONT: &str = "rubik_regular";

static APP_TITLE: &str = "BeatSaber Browser";

lazy_static! {
    static ref SIDE_MENU_ITEMS: Vec<CurrentWindow> = {
        vec![
            (CurrentWindow::Browser),
            (CurrentWindow::MapDetail),
            (CurrentWindow::Settings),
        ]
    };
    static ref IMG_CACHE: HashMap<AppImage, Result<RetainedImage, String>> = {
        let mut map = HashMap::new();
        map.insert(
            AppImage::SideMenu,
            load_svg(&AppImage::SideMenu.dbg_id(), "settings-logo.svg", (30, 30)),
        );

        map
    };
}

struct App {
    current_window: CurrentWindow,
    side_menu: SideMenu,
}

impl App {
    fn new(cctx: &CreationContext<'_>) -> Self {
        setup_custom_font(&cctx.egui_ctx);

        Self {
            current_window: CurrentWindow::Browser,
            side_menu: SideMenu::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match &mut self.current_window {
            CurrentWindow::Browser => {
                egui::SidePanel::left(Id::new("side_menu")).show_animated(
                    ctx,
                    self.side_menu.open,
                    |ui| {
                        draw_settings_bar(ui, self);
                    },
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            RichText::new(APP_TITLE)
                                .color(Color32::from_rgb(255, 255, 255))
                                .text_style(TextStyle::Heading),
                        );
                    });

                    let clicked = !self.side_menu.open
                        && match get_svg(&AppImage::SideMenu) {
                            Ok(img) => ui
                                .add(ImageButton::new(img.texture_id(ctx), img.size_vec2()))
                                .clicked(),
                            Err(fallback_text) => ui.add(Button::new(fallback_text)).clicked(),
                        };

                    if !self.side_menu.open && clicked {
                        self.side_menu.open = true;
                    }
                });
            }
            CurrentWindow::Settings => todo!("add settings screen"),
            CurrentWindow::MapDetail => todo!("add map details screen"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum AppImage {
    SideMenu,
}

impl AppImage {
    fn dbg_id(&self) -> String {
        match self {
            AppImage::SideMenu => "side_menu_image".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum CurrentWindow {
    Browser,
    MapDetail,
    Settings,
}

impl Display for CurrentWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CurrentWindow::Browser => "Browser",
            CurrentWindow::MapDetail => "Map Details",
            CurrentWindow::Settings => "Settings",
        };

        write!(f, "{str}")
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct SideMenu {
    open: bool,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(f32::MAX, f32::MAX)),
        ..Default::default()
    };

    eframe::run_native(APP_TITLE, options, Box::new(|cc| Box::new(App::new(cc))))
}

fn draw_settings_bar(ui: &mut Ui, app: &mut App) {
    let side_menu = &mut app.side_menu;
    let close_button = AppButton::Close.build();

    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        if side_menu.open && ui.add(close_button).clicked() {
            side_menu.open = false;
        }

        ui.separator();

        for window in SIDE_MENU_ITEMS.iter() {
            if ui
                .add(AppButton::Subtle(format!("{window}")).build())
                .clicked()
            {
                app.current_window = window.clone();
            }
            ui.add_space(10.0);
        }
    });
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
        (Button, FontId::new(20.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);

    font.font_data.insert(
        FONT.to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Rubik-Regular.ttf")),
    );

    font.families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, FONT.to_owned());

    ctx.set_fonts(font);
}

fn get_svg(img: &AppImage) -> &Result<RetainedImage, String> {
    IMG_CACHE.get(img).unwrap()
}

fn load_svg(dbg_id: &str, file_name: &str, size: (u32, u32)) -> Result<RetainedImage, String> {
    let path = format!("assets/images/{file_name}");

    let mut buffer = Vec::new();
    let mut file = File::open(path).map_err(|_e| dbg_id.to_owned())?;
    file.read_to_end(&mut buffer)
        .map_err(|_e| dbg_id.to_owned())?;

    RetainedImage::from_svg_bytes_with_size(dbg_id.to_owned(), &buffer, FitTo::Size(size.0, size.1))
}
