use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Greeting {
    Hello(Option<String>),
    Shake,
}
