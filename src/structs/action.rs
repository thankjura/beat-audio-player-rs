pub enum Action {
    Play,
    Stop,
    Next,
    Prev,
}

impl Action {
    pub fn get_value(&self) -> u8 {
        match self {
            Action::Play => 1,
            Action::Stop => 2,
            Action::Next => 3,
            Action::Prev => 4,
        }
    }

    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Action::Play),
            2 => Some(Action::Stop),
            3 => Some(Action::Next),
            4 => Some(Action::Prev),
            _ => None,
        }
    }
}
