use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub text: String,
    pub author: String
}
