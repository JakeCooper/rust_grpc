use std::sync::Mutex;

pub struct Controller {
    pub data: Mutex<String>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            data: Mutex::new("Hi".to_string()),
        }
    }
    pub fn say_hello(&self, name: String) -> String {
        return format!("Hello {}", name);
    }

    pub fn set_data(&self, data: String) {
        let mut state = self.data.lock().expect("Could not lock mutex");
        *state = data;
    }

    pub fn get_data(&self) -> String {
        self.data.lock().unwrap().to_string()
    }
}
