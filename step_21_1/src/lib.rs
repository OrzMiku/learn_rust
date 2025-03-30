use step_21::Log;

pub trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);   
}

#[derive(Log)]
pub struct Database {
    url: String,
    connections: u32
}

impl Database {
    pub fn new(url: String) -> Self {
        Database {
            url,
            connections: 0
        }
    }

    pub fn connect(&mut self) {
        self.connections += 1;
        if self.connections > 20 {
            self.error("Too many connections");
        } else if self.connections > 15 {
            self.info(format!("Connected to {}", self.url).as_str());
            self.warn("Too many connections");
        } else {
            self.info(format!("Connected to {}", self.url).as_str());
        }
    }
}