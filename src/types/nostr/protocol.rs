use std::collections::VecDeque;
use serde::Deserialize;
use serde::Serialize;
use crate::types::nostr::filter::Filter;
use crate::types::nostr::event::Event;

//-////////////////////////////////////////////////////////////////////////////
//  Raw Protocol
//-////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ProtocolCategory {
    EVENT,
    REQ,
    CLOSE,
    NOTICE,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProtocolData {
    MsgType(ProtocolCategory),
    SubscriptionID(String),
    Event(Event),
    Filter(Filter),
}

//-////////////////////////////////////////////////////////////////////////////
//  Protocol
//-////////////////////////////////////////////////////////////////////////////
pub enum Protocol {
    Event(Event),
    Request{sub_id: String, filters: Vec<Filter>},
    Close{sub_id: String},
}

impl Protocol {
    pub fn from_data(mut data: VecDeque<ProtocolData>) -> Result<Protocol, &'static str> {
        match data.pop_front().unwrap() {
            ProtocolData::MsgType(msg) => match match msg {
                ProtocolCategory::NOTICE => None,
                ProtocolCategory::EVENT  => Protocol::from_event(data),
                ProtocolCategory::REQ    => Protocol::from_request(data),
                ProtocolCategory::CLOSE  => Protocol::from_close(data),
            } {
                None => Err("Could not decode event"),
                Some(protocol) => Ok(protocol)
            },
            _ => Err("Could not decode protocol"),
        }
    }

    fn from_event(mut data: VecDeque<ProtocolData>) -> Option<Protocol> {
        if let ProtocolData::Event(event) = data.pop_front().unwrap() {
            Some(Protocol::Event(event))
        } else {
            None
        }
    }

    fn from_request(mut data: VecDeque<ProtocolData>) -> Option<Protocol> {
        let sub_id: String = if let ProtocolData::SubscriptionID(sub_id) = data.pop_front().unwrap() {
            Some(sub_id)
        } else {
            None
        }?;

        let filters: Vec<Filter> = data.into_iter()
            .fold(Some(vec![]), |acc, entry| match acc {
                None => None,
                Some(mut acc) => match entry {
                    ProtocolData::Filter(filter) => {
                        acc.push(filter);
                        Some(acc)
                    },
                    _ => None,
                }
            })?;

        Some(Protocol::Request{ sub_id, filters })
    }

    fn from_close(mut data: VecDeque<ProtocolData>) -> Option<Protocol> {
        if let ProtocolData::SubscriptionID(sub_id) = data.pop_front().unwrap() {
            Some(Protocol::Close { sub_id })
        } else {
            None
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
