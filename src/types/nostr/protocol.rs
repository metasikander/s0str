use serde::Deserialize;
use serde::Serialize;
use crate::types::nostr::filter::Filter;
use crate::types::nostr::event::Event;

#[derive(PartialEq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MsgType {
    EVENT,
    REQ,
    CLOSE,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProtocolData {
    MsgType(MsgType),
    SubscriptionID(String),
    Event(Event),
    Filter(Filter),
}

pub enum Protocol {
    Event(Event),
    Request{sub_id: String, filters: Vec<Filter>},
    Close{sub_id: String},
}

impl Protocol {
    pub fn from_data(data: Vec<ProtocolData>) -> Result<Protocol, &'static str> {
        if let ProtocolData::MsgType(msg) = data[0].clone() {
            println!("----------------------------------------------");
            println!("type: {:?}, nr2: {:?}", &data[0], &data[1]);
            return match msg {
                MsgType::EVENT => Protocol::from_event(data),
                MsgType::REQ => Protocol::from_request(data),
                MsgType::CLOSE => Protocol::from_close(data),
            }
        } else {
            Err("Could not decode protocol")
        }
    }

    fn from_event(data: Vec<ProtocolData>) -> Result<Protocol, &'static str> {
        if let ProtocolData::Event(event) = data[1].clone() {
            Ok(Protocol::Event(event))
        } else {
            Err("Could not decode event")
        }
    }

    fn from_request(data: Vec<ProtocolData>) -> Result<Protocol, &'static str> {
        let id: String = if let ProtocolData::SubscriptionID(id) = data[1].clone() {
            id
        } else {
            return Err("Could not decode request");
        };

        let mut filters: Vec<Filter> = vec![];
        for entry in &data[2..] {
            if let ProtocolData::Filter(filter) = entry {
                filters.push(filter.clone());
            } else {
                return Err("Could not decode request");
            }
        }

        Ok(Protocol::Request{ sub_id: id, filters: filters })
    }

    fn from_close(data: Vec<ProtocolData>) -> Result<Protocol, &'static str> {
        if let ProtocolData::SubscriptionID(id) = data[1].clone() {
            Ok(Protocol::Close { sub_id: id })
        } else {
            return Err("Could not decode close");
        }
    }
}
