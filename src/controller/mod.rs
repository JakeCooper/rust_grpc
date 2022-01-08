use super::gateway::Gateway;

pub struct Controller {
    pub gtwy: Gateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            gtwy: Gateway::new(),
        }
    }

    pub fn mutate(&self, data: String) {
        self.gtwy.mutate(data)
    }

    pub fn read(&self) -> String {
        self.gtwy.read()
    }
}
