use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::DbConn;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde::Serialize;
use crate::CONFIG;
use crate::types::nostr::event::Event;
use crate::types::nostr::tag::Tag;
use entity::nostr::event;
use entity::nostr::tag;

//-////////////////////////////////////////////////////////////////////////////
//  Filter
//-////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "ids")]
    pub event_ids: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub kinds: Option<Vec<u64>>,
    pub e: Option<Vec<String>>,
    pub p: Option<Vec<String>>,
    pub since: Option<u64>,
    pub until: Option<u64>,
    pub limit: Option<usize>,
}

impl Filter {
    pub async fn find_event(self, db: &DbConn) -> Vec<Event> {
        let mut query = event::Entity::find()
            .order_by_desc(event::Column::CreatedAt)
            .find_with_related(tag::Entity);

        // -- Author --
        if let Some(authors) = self.authors {
            let is_match = authors.iter()
                .map(|author| CONFIG.pubkey.starts_with(author))
                .fold(false, |acc, value| if acc {acc} else {value});
            if !is_match {
                return vec![];
            }
        }

        // -- Event Id --
        if let Some(ids) = self.event_ids {
            let mut condition = Condition::any();
            for id in ids {
                condition = condition.add(event::Column::Id.eq(id));
            }
            query = query.filter(condition);
        }

        // todo: add e and p filters

        // -- Date --
        if let Some(since) = self.since {query = query.filter(event::Column::CreatedAt.gt(since));}
        if let Some(until) = self.until {query = query.filter(event::Column::CreatedAt.lt(until));}

        // -- Output --
        let res = query.all(db).await.unwrap()
            .into_iter()
            .map(|(raw_event, raw_tags)| Event::from_model(
                raw_event,
                raw_tags.into_iter().map(|raw_tag| Tag::from_model(raw_tag)).collect()
            ));

        match self.limit {
            Some(limit) => res.take(limit).collect(),
            None => res.collect(),
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
