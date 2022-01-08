use std::sync::RwLock;

type Core = String;

pub struct Gateway {
    prefix: String,
    data: RwLock<Option<Core>>,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            prefix: "Hello ".to_string(),
            data: RwLock::new(None),
        }
    }

    pub fn mutate(&self, some_data: Core) {
        let mut state = self.data.write().expect("Could not lock");
        *state = Some(some_data)
    }

    pub fn read(&self) -> Core {
        match self.data.read().unwrap().as_ref() {
            None => "First!".to_string(),
            Some(name) => self.prefix.to_string() + name,
        }
    }
}
