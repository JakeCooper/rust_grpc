use std::sync::RwLock;

type Core = String;

pub struct Greatway {
    data: RwLock<Core>,
}

impl Greatway {
    pub fn new() -> Self {
        Self {
            data: RwLock::new("Hi".to_string()),
        }
    }

    pub fn mutate(&self, some_data: Core) {
        let mut state = self.data.write().expect("Could not lock");
        *state = some_data
    }

    pub fn read(&self) -> Core {
        self.data.read().unwrap().to_string()
    }
}
