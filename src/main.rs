extern crate iced;
extern crate walkdir;
extern crate zip;


mod core;
mod ui;

use iced::{Application, executor, Settings, Text, Column, container::Style, Container, Color, Background, button};

fn main() {

	let window_settings = iced::window::Settings {
		size: (800, 400),
		..Default::default()
	};

	let settings = Settings {
		window: window_settings,
		..Default::default()
	};

	BackUp::run(settings).unwrap()
}


#[derive(Debug, Clone, Copy)]
pub enum AppActions {
	BackupProgress
}

struct BackUp {
	button_state: button::State,
	is_clicked: Option<String>
}


impl Application for BackUp {
	type Executor = executor::Default;

	type Message = AppActions;

	type Flags = ();

	fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		let me = Self {
			button_state: button::State::new(),
			is_clicked: None
		};
		(me, iced::Command::none())
	}

	fn title(&self) -> String {
		String::from("Backup")
	}

	fn update(
		&mut self,
		message: Self::Message,
		clipboard: &mut iced::Clipboard,
	) -> iced::Command<Self::Message> {

		match message {
			AppActions::BackupProgress => {
				self.is_clicked = Some("This is my first button click".to_owned());
			}
		}

		iced::Command::none()
	}

	fn view(&mut self) -> iced::Element<'_, Self::Message> {
		let column = Column::new()
				.push(Text::new("Hello, world!"))
				.push(Text::new("This is a new app"))
				.push(
					button::Button::new(
						&mut self.button_state, 
						Text::new("Backup")
					).on_press(AppActions::BackupProgress)
				);

		let updated_column = if self.is_clicked.is_some() {
			column.push(Text::new(self.is_clicked.as_ref().unwrap().clone()))
		} else {
			column
		};

		Container::new(updated_column)
			.style(HomeContainerStyle)
			.into()
	}
}

struct HomeContainerStyle;

impl iced::container::StyleSheet for HomeContainerStyle {
	fn style(&self) -> Style {
		Style {
			background: Some(Background::Color(Color::WHITE)),
			border_radius: 10.0,
			border_width: 1.0,
			border_color: Color::new(1.0, 0.0, 0.0, 0.0),
			text_color: Some(Color::new(0.2, 0.1, 0.1, 1.0))
		}
	}
}