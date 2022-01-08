use super::gateways::Gateway;

pub struct Controller {
    pub gwty: Gateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            gwty: Gateway::new(),
        }
    }
    pub fn say_hello(&self, name: String) {
        println!("Henlo {}", name);
    }

    pub fn get_stateway(&self) -> String {
        self.gwty.get_data()
    }

    pub fn set_stateway(&mut self, data: String) {
        self.gwty.set_data(data)
    }
}
