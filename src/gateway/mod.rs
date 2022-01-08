use super::greatway::Greatway;

pub struct Gateway {
    pub gtwy: Greatway,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            gtwy: Greatway::new(),
        }
    }

    pub fn mutate(&self, data: String) {
        self.gtwy.mutate(data);
        println!("UMM FARTIN: {}", 4);
    }

    pub fn read(&self) -> String {
        self.gtwy.read()
    }
}
