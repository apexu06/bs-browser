use iced::{
    executor,
    widget::{Button, Column, Text},
    Application, Command, Element, Renderer, Theme,
};

struct App {
    strings: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Actions {
    Test,
}

fn main() {
    App::run(iced::Settings::default()).unwrap();
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Actions;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (
            App {
                strings: vec![String::from("fuck")],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("BeatSaber Browser")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Actions::Test => {
                self.strings.push(String::from("Test"));
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut col =
            Column::new().push(Button::new(Text::new("Do something")).on_press(Actions::Test));

        for s in &self.strings {
            col = col.push(Text::new(s));
        }

        col.into()
    }
}
