use std::sync::Mutex;

pub struct Gateway {
    data: Mutex<String>,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            data: Mutex::new("Fuck".to_string()),
        }
    }
    pub fn set_data(&mut self, data: String) {
        let mut state = self.data.lock().expect("Could not lock mutex");
        *state = data;
    }

    pub fn get_data(&self) -> String {
        self.data.lock().unwrap().to_string()
    }
}
