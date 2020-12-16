use std::{collections::HashMap, rc::Rc, sync::Arc};

use iced::{
    executor, Application, Button, Column, Command, Container, Element, Length, Settings,
    Subscription, Text,
};
use GruiMessage::ChoiceChanged;

use crate::command_actions::{GrunnerConfig, GrunnerOption};
use crate::{command_actions::GrunnerAction, task_subscription};

pub fn run_grui(config: GrunnerConfig) {
    Grui::run(Settings::with_flags(config)).expect("Error running Grunner UI");
}

#[derive(Debug)]
pub struct Grui {
    config: GrunnerConfig,
    state: GState,
    button_states: HashMap<String, iced::button::State>,
}

#[derive(Debug)]
enum GState {
    Idle,
    Working(GrunnerAction),
}

#[derive(Debug, Clone)]
pub enum GruiMessage {
    Start,
    StartAction(GrunnerAction),
    ActionUpdate(task_subscription::ActionProgress),
    ChoiceChanged(usize),
}

impl Grui {
    fn new(config: GrunnerConfig) -> Self {
        Grui {
            config,
            state: GState::Idle,
            button_states: HashMap::new(),
        }
    }
}

impl Application for Grui {
    type Executor = executor::Default;
    type Message = GruiMessage;
    type Flags = GrunnerConfig;

    fn new(flags: GrunnerConfig) -> (Grui, Command<GruiMessage>) {
        (Grui::new(flags), Command::none())
    }

    fn title(&self) -> String {
        String::from("Grunner UI")
    }

    fn update(&mut self, message: GruiMessage) -> Command<GruiMessage> {
        match message {
            GruiMessage::Start => {}
            GruiMessage::StartAction(act) => self.state = GState::Working(act),
            GruiMessage::ActionUpdate(update) => match update {
                task_subscription::ActionProgress::Starting => {}
                task_subscription::ActionProgress::Continuing => {}
                task_subscription::ActionProgress::Completed => self.state = GState::Idle,
                task_subscription::ActionProgress::Error => self.state = GState::Idle,
            },
            ChoiceChanged(opt_id) => {
                for sect in &mut self.config.sections {
                    for (_, opt) in &mut sect.options {
                        if opt.try_set_option(&opt_id) {}
                    }
                }
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<GruiMessage> {
        match &self.state {
            GState::Working(act) => {
                task_subscription::build_subscription(&act).map(GruiMessage::ActionUpdate)
            }
            _ => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<GruiMessage> {
        let mut content = Column::new();

        match self.state {
            GState::Idle => {
                for sect in self.config.sections.iter_mut() {
                    // TODO: Draw a label and separator for this section

                    for (_section_label, opt) in sect.options.iter_mut() {
                        match opt {
                            GrunnerOption::Choices { choices, selected } => {
                                for choice in choices.iter() {
                                    if let None = selected {
                                        *selected = Some(choice.id);
                                    }
                                    content = content.push(iced::radio::Radio::new(
                                        choice.id,
                                        &choice.label,
                                        selected.to_owned(),
                                        GruiMessage::ChoiceChanged,
                                    ));
                                }
                            }
                            GrunnerOption::Flag {
                                name: _,
                                value: _,
                                arg: _,
                            } => {}
                        }
                    }

                    for (text, act) in sect.actions.iter_mut() {
                        let act_clone = act.clone();
                        content = content.push(
                            Button::new(&mut act.gui_state, Text::new(text))
                                .on_press(GruiMessage::StartAction(act_clone)),
                        );
                    }
                }
            }
            // let column: Column<_> = self
            //     .config
            //     .actions
            //     .iter()
            //     .enumerate()
            //     .fold(Column::new(), |column, (i, (text, act))| {
            //         column.push(
            //             Button::new(btn_state, Text::new(text))
            //                 .on_press(GruiMessage::StartAction(act.clone())),
            //         )
            //     })
            //     .into();
            GState::Working(_) => {
                content = content.push(Text::new("Working..."));
            }
        }
        // let column: Column<_> = self
        //     .config
        //     .actions
        //     .iter()
        //     .enumerate()
        //     .fold(Column::new(), |column, (i, (text, _act))| {
        //         column.push(Text::new(text))
        //     })
        //     .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
