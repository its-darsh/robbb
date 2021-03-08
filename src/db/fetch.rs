use std::collections::HashMap;

use anyhow::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use serenity::model::id::{GuildId, UserId};

use super::Db;

#[derive(Debug)]
pub struct Fetch {
    pub user: UserId,
    pub info: HashMap<String, String>,
}

impl Db {
    pub async fn set_fetch(&self, user: UserId, info: HashMap<String, String>) -> Result<Fetch> {
        let mut conn = self.pool.acquire().await?;
        {
            let user = user.0 as i64;
            let info = serde_json::to_string(&info)?;

            // TODO this should be handled on sql layer
            let result = sqlx::query!("insert into fetch (usr, info) values (?, ?)", user, info)
                .execute(&mut conn)
                .await;
            if result.is_err() {
                sqlx::query!("update fetch set usr=?, info=?", user, info)
                    .execute(&mut conn)
                    .await?;
            }
        }

        Ok(Fetch { user, info })
    }

    pub async fn get_fetch(&self, user: UserId) -> Result<Option<Fetch>> {
        let mut conn = self.pool.acquire().await?;
        let user = user.0 as i64;
        let value = sqlx::query!("select * from fetch where usr=?", user)
            .fetch_optional(&mut conn)
            .await?;
        if let Some(x) = value {
            Ok(Some(Fetch {
                user: UserId(x.usr as u64),
                info: serde_json::from_str(&x.info)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_fetch(
        &self,
        user: UserId,
        new_values: HashMap<String, String>,
    ) -> Result<Fetch> {
        let mut fetch = self
            .get_fetch(user)
            .await?
            .map(|x| x.info)
            .unwrap_or_default();

        for (key, value) in new_values {
            fetch.insert(key, value);
        }

        Ok(self.set_fetch(user, fetch).await?)
    }
}