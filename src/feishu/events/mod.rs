use std::str::FromStr;

pub mod common;
pub mod im_message;




#[derive(Debug)]
pub enum EventType {
    IMMessageReceive,
}

impl FromStr for EventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "im.message.receive_v1" => Ok(EventType::IMMessageReceive),
            _ => Err(()),
        }
    }
}
