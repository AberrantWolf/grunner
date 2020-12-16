use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::cell::Cell;
use std::io::Read;
use std::{collections::HashMap, rc::Rc};
use std::{fs::File, sync::Arc};

thread_local!(static OPTION_ID: Cell<usize> = Cell::new(0));
fn next_option_id() -> usize {
    OPTION_ID.with(|id| {
        let result = id.get();
        id.set(result + 1);
        result
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrunnerChoiceType {
    #[serde(skip, default = "next_option_id")]
    pub id: usize,
    pub label: String,
    pub arg: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GrunnerOption {
    Choices {
        choices: Vec<GrunnerChoiceType>,

        #[serde(skip)]
        selected: Option<usize>,
    },
    Flag {
        name: String,
        value: bool,
        arg: String,
    },
}

impl GrunnerOption {
    pub fn try_set_option(&mut self, option_id: &usize) -> bool {
        if let GrunnerOption::Choices { choices, selected } = self {
            for opt in choices {
                if opt.id == *option_id {
                    *selected = Some(*option_id);
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrunnerSection {
    pub label: String,
    pub options: HashMap<String, GrunnerOption>,
    pub actions: HashMap<String, GrunnerAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrunnerAction {
    pub execute: String,

    #[serde(default)]
    pub args: Vec<String>,

    #[serde(skip)]
    pub gui_state: iced::button::State,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrunnerConfig {
    pub sections: Vec<GrunnerSection>,
}

pub fn load_grunner_config<S>(path: S) -> Result<GrunnerConfig>
where
    S: Into<String>,
{
    let mut file = File::open(path.into()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: GrunnerConfig = serde_json::from_str(&data).unwrap();

    Ok(config)
}
