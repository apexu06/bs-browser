pub trait BuildWidget {
    type Widget: egui::Widget;
    fn build(self) -> Self::Widget;
}
