pub enum Action {
    PLAY,
    STOP,
    NEXT,
    PREV,
}

impl Action {
    pub fn get_value(&self) -> u8 {
        match self {
            Action::PLAY => { 1 }
            Action::STOP => { 2 }
            Action::NEXT => { 3 }
            Action::PREV => { 4 }
        }
    }

    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => { Some(Action::PLAY) },
            2 => { Some(Action::STOP) },
            3 => { Some(Action::NEXT) },
            4 => { Some(Action::PREV) },
            _ => { None }
        }
    }
}