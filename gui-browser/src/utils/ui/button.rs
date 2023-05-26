use super::build_widget::BuildWidget;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[allow(dead_code)]
pub enum AppButton {
    Default(String),
    Subtle(String),
    Close,
}

#[allow(dead_code)]
impl BuildWidget for AppButton {
    type Widget = egui::Button;

    fn build(self) -> Self::Widget {
        match self {
            AppButton::Default(text) => egui::Button::new(text)
                .min_size((100.0, 30.0).into())
                .rounding(5.0),
            AppButton::Subtle(text) => egui::Button::new(text)
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT))
                .min_size((100.0, 30.0).into())
                .rounding(0.0),
            AppButton::Close => egui::Button::new("Close")
                // it's a bit strange to make a close button blue
                .fill(egui::Color32::from_rgb(9, 93, 249))
                .stroke(egui::Stroke::new(1.2, egui::Color32::WHITE))
                .min_size((100.0, 30.0).into())
                .rounding(5.0),
        }
    }
}
