use egui::{Color32, Stroke};

pub struct AppButton {
    text: String,
    size: egui::Vec2,
    color: Color32,
    stroke: egui::Stroke,
}

impl AppButton {
    pub fn new(text: &str, size: egui::Vec2, color: Color32) -> Self {
        Self {
            text: text.to_owned(),
            size,
            color,
            stroke: Stroke::new(1.2, Color32::WHITE),
        }
    }

    pub fn close_button() -> Self {
        Self {
            text: "Close".to_owned(),
            size: (100.0, 30.0).into(),
            color: Color32::from_rgb(9, 93, 249),
            stroke: Stroke::new(1.2, Color32::WHITE),
        }
    }
}

impl Into<egui::Button> for AppButton {
    fn into(self) -> egui::Button {
        egui::Button::new(self.text)
            .fill(self.color)
            .min_size(self.size)
            .rounding(5.0)
            .stroke(self.stroke)
    }
}
