mod utils;
pub mod schema;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
    Result,
};
use common::logging;
use config::CONFIG;
use schema::Account;
use crate::schema::Player;

const ACCOUNT_TABLE: &str = "account";
const PLAYER_TABLE: &str = "player";
const SDK_NAMESPACE: &str = "cheshire";
const SDK_DB_NAME: &str = "user";

#[derive(Clone)]
pub struct DbContext(Surreal<Client>);

impl DbContext {
    pub async fn connect() -> Result<Self> {
        const DEFINE_INCREMENT_FUNC_QUERY: &str = r#"DEFINE FUNCTION fn::increment($name: string) {
    RETURN (UPSERT ONLY type::thing('counter', $name)
    SET value += 1).value;
};"#;
        use surrealdb::opt::auth::Root;

        let database = Surreal::new::<Ws>(&CONFIG.database_config.url)
            .await?;
        database.signin(Root {
            username: &CONFIG.database_config.username,
            password: &CONFIG.database_config.password,
        }).await?;

        database.use_ns(SDK_NAMESPACE).use_db(SDK_DB_NAME).await?;

        // uid auto increment
        database.query(DEFINE_INCREMENT_FUNC_QUERY).await?;

        Ok(Self(database))
    }
}

impl DbContext {
    pub async fn create_account(&self, device_id: String) -> Result<Option<Account>> {
        const ACCOUNT_UID_COUNTER: &str = "account_uid";

        if self.get_account_by_device_id(device_id.clone()).await?.is_some() {
            return Ok(None);
        }

        let account = Account::new(device_id);
        let id = self.get_next_uid(ACCOUNT_UID_COUNTER)
            .await?;

        let account: Account = self.0
            .create((ACCOUNT_TABLE, id.to_string()))
            .content(account)
            .await?
            .unwrap();

        Ok(Some(account))
    }

    pub async fn get_account_by_device_id(&self, device_id: String) -> Result<Option<Account>> {
        const SELECT_WHERE_QUERY: &str = r#"SELECT * FROM account WHERE device_id = $value"#;
        const VALUE_BIND: &str = "value";

        self.0
            .query(SELECT_WHERE_QUERY)
            .bind((VALUE_BIND, device_id))
            .await?
            .take(0)
    }

    pub async fn get_account_by_uid(&self, uid: String) -> Result<Option<Account>> {
        self.0.select((ACCOUNT_TABLE, uid)).await
    }

    pub async fn get_next_uid(&self, counter_name: &'static str) -> Result<u32> {
        const UID_INCREMENT_QUERY: &str = "RETURN fn::increment($counter_name)";
        const UID_COUNTER_NAME_BIND: &str = "counter_name";

        Ok(self.0
            .query(UID_INCREMENT_QUERY)
            .bind((UID_COUNTER_NAME_BIND, counter_name))
            .await?
            .check()?
            .take::<Option<u32>>(0)?
            .unwrap())
    }
}

impl DbContext {
    pub async fn create_player(&self, uid: u32) -> Result<Option<Player>> {
        if self.get_player_by_uid(uid.to_string()).await?.is_some() {
            return Ok(None);
        }

        let player = Player::new();

        let player: Player = self.0
            .create((PLAYER_TABLE, uid.to_string()))
            .content(player)
            .await?
            .unwrap();

        Ok(Some(player))
    }

    pub async fn get_player_by_uid(&self, uid: String) -> Result<Option<Player>> {
        self.0.select((PLAYER_TABLE, uid)).await
    }

    pub async fn update_player_data(
        &self,
        uid: u32,
        data: serde_json::Value,
    ) -> Result<Option<Player>> {
        if let Some(mut player) = self.get_player_by_uid(uid.to_string()).await? {
            player.data = data;

            let updated_player: Player = self.0
                .update((PLAYER_TABLE, uid.to_string()))
                .content(player)
                .await?
                .unwrap();

            Ok(Some(updated_player))
        } else {
            Ok(None)
        }
    }
}

impl DbContext {
    pub async fn ban_player_by_uid(&self, uid: String) -> Result<()> {
        if let Some(mut player) = self.get_player_by_uid(uid.to_string()).await? {
            player.is_banned = true;
            let _: Option<Player> = self.0
                .update((PLAYER_TABLE, uid.to_string()))
                .content(player)
                .await?;

            Ok(())
        } else {
            logging::warn!("No player with uid {} found", uid);
            Ok(())
        }
    }

    pub async fn unban_player_by_uid(&self, uid: String) -> Result<()> {
        if let Some(mut player) = self.get_player_by_uid(uid.to_string()).await? {
            player.is_banned = false;
            let _: Option<Player> = self.0
                .update((PLAYER_TABLE, uid.to_string()))
                .content(player)
                .await?;

            Ok(())
        } else {
            logging::warn!("No player with uid {} found", uid);
            Ok(())
        }
    }
}
