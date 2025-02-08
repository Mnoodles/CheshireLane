use common::logging;
use database::DbContext;
use ecs_persistence::player_info::PlayerInfo;
use tokio::select;
use tokio::sync::{mpsc, oneshot};

enum DbOperation {
    Fetch(u32, oneshot::Sender<Option<PlayerInfo>>),
    Ban(u32),
    Unban(u32),
}

pub struct DbWorkerHandle(mpsc::Sender<DbOperation>);

impl DbWorkerHandle {
    pub async fn fetch(&self, uid: u32) -> Option<PlayerInfo> {
        let (tx, rx) = oneshot::channel();
        let _ = self.0.send(DbOperation::Fetch(uid, tx)).await;

        rx.await.ok().flatten()
    }

    pub async fn ban(&self, uid: u32) {
        let _ = self.0.send(DbOperation::Ban(uid)).await;
    }

    pub async fn unban(&self, uid: u32) {
        let _ = self.0.send(DbOperation::Unban(uid)).await;
    }
}

pub fn start(context: DbContext) -> (DbWorkerHandle, mpsc::Sender<(u32, serde_json::Value)>) {
    let (op_tx, op_rx) = mpsc::channel(32);
    let (save_data_tx, save_data_rx) = mpsc::channel(32);

    tokio::spawn(async move {
        db_work_loop(context, op_rx, save_data_rx).await;
    });

    (DbWorkerHandle(op_tx), save_data_tx)
}

async fn db_work_loop(
    context: DbContext,
    mut op_rx: mpsc::Receiver<DbOperation>,
    mut save_data_rx: mpsc::Receiver<(u32, serde_json::Value)>,
) {
    loop {
        select! {
            op = op_rx.recv() => {
                match op {
                    Some(DbOperation::Fetch(uid, tx)) => {
                        let result = match context
                            .get_player_by_uid(uid.to_string()).await {
                            Ok(Some(player)) => {
                                // Return player data if found player in db
                                match serde_json::from_value(player.data) {
                                    Ok(info) => info,
                                    Err(_) => {
                                        // Update to default if deserialize failed
                                        logging::warn!(
                                            "Failed to deserialize player {}, reset to default",
                                            uid);
                                        update_default_player_info(&context, uid).await
                                    },
                                }
                            },
                            Ok(None) => {
                                // Create a new default player with uid
                                if let Err(_) = context.create_player(uid).await {
                                    logging::error!(
                                        "Failed to create player {}",
                                        uid);
                                    None
                                } else {
                                    update_default_player_info(&context, uid).await
                                }
                            },
                            Err(err) => {
                                logging::error!(
                                    "Failed to fetch player data {} from database: {:?}",
                                    uid, err);
                                None
                            },
                        };

                        let _ = tx.send(result);
                    },
                    Some(DbOperation::Ban(uid)) => {
                        if let Err(err) = context.ban_player_by_uid(uid.to_string()).await {
                            logging::error!("Failed to ban player {}: {}", uid, err);
                        }
                    },
                    Some(DbOperation::Unban(uid)) => {
                        if let Err(err) = context.unban_player_by_uid(uid.to_string()).await {
                            logging::error!("Failed to unban player {}: {}", uid, err);
                        }
                    },
                    None => {
                        logging::error!("DB Worker received a bad DbOperation");
                    },
                }
            },
            save_data = save_data_rx.recv() => {
                if let Some((uid, data)) = save_data {
                    if let Err(err) = context.update_player_data(uid, data).await {
                        logging::error!("Failed to save player data: {}", err);
                    }
                }
            },
        }
    }
}

async fn update_default_player_info(context: &DbContext, uid: u32) -> Option<PlayerInfo> {
    let default_info = PlayerInfo::default();
    if let Err(_) = context
        .update_player_data(uid, serde_json::to_value(default_info.clone()).unwrap())
        .await
    {
        logging::error!("Failed to update default player data for uid {}", uid);
        None
    } else {
        Some(default_info)
    }
}
