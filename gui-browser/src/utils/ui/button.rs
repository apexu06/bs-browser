use super::build_widget::BuildWidget;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AppButton {
    Close,
}

#[allow(dead_code)]
impl BuildWidget for AppButton {
    type Widget = egui::Button;

    fn build(self) -> Self::Widget {
        match self {
            AppButton::Close => egui::Button::new("Close")
                .fill(egui::Color32::from_rgb(9, 93, 249))
                .stroke(egui::Stroke::new(1.2, egui::Color32::WHITE))
                .min_size((100.0, 30.0).into())
                .rounding(5.0),
        }
    }
}
