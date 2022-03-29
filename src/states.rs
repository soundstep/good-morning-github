struct Command {
    key: String,
    label: String,
}

pub struct StateDefinition {
    title: String,
    commands: Vec<Command>,
}

// impl StateDefinition {
//     fn new(title: String, commands: Vec<Command>) -> Self {
//         StateDefinition { title, commands }
//     }
// }

pub struct WelcomeState {
    definition: StateDefinition,
}

impl WelcomeState {
    pub fn new() -> Self {
        WelcomeState {
            definition: StateDefinition {
                title: String::from("welcome title"),
                commands: Vec::new(),
            },
        }
    }
}

pub struct SetupState {
    definition: StateDefinition,
}

impl SetupState {
    pub fn new() -> Self {
        SetupState {
            definition: StateDefinition {
                title: String::from("setup title"),
                commands: Vec::new(),
            },
        }
    }
}

pub trait State {}

impl State for WelcomeState {}
impl State for SetupState {}
