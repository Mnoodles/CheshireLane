use encryption::Md5;
use common::logging;
use config::CONFIG;
use proto::p10::{Cs10800, Cs10020, Sc10801, Sc10021, Cs10018, Sc10019};
use crate::{VERSION, SERVERS};

pub fn get_assets_hash(_req: Cs10800) -> Sc10801 {
    let monday_0oclock = "2020-11-23 07:00:00";
    let monday_0oclock = common::time::get_timestamp_s(monday_0oclock);

    Sc10801 {
        gateway_ip: CONFIG.dispatch_config.ip.clone(),
        gateway_port: CONFIG.dispatch_config.port as u32,
        url: format!("http://{}", CONFIG.dispatch_config.ip),
        version: VERSION.get().unwrap().0.clone(),
        proxy_ip: Some(CONFIG.dispatch_config.ip.clone()),
        proxy_port: Some(CONFIG.dispatch_config.port as u32),
        is_ts: 0,
        timestamp: common::time::now_timestamp_s() as u32,
        monday_0oclock_timestamp: monday_0oclock as u32,
        cdn_list: vec![],
    }
}

pub fn user_login(req: Cs10020) -> Sc10021 {
    // Cs10020:
    //   arg1: PLATFORM_AIRIUS | PLATFORM_AIRIJP | PLATFORM_TXWY | PLATFORM_BILIBILI
    //         "yostarus"      | "yostarjp"      | "txwykr"      | "bilibili"
    //   arg2: UID
    //   arg3: ACCESS_TOKEN
    //   arg4: "0"
    //   check_key: md5(arg1 + salt)
    //   device: PLATFORM_ANDROID(11) | PLATFORM_IPHONEPLAYER(8) | PLATFORM_WINDOWSEDITOR(7)
    let hash = Md5::hash(&req.arg1, Some(&CONFIG.dispatch_config.salt));
    let result = if hash != req.check_key { 1 } else { 0 };
    if result != 0 {
        logging::error!("GetServerInfoError: check key failed");
    }

    Sc10021 {
        // 6: login_game_login_full
        // 13: login_gate_not_ready
        // 15: login_game_rigister_full
        // 18: system_database_busy
        // _: USER_LOGIN_FAILED + result => facade:sendNotification
        result,
        serverlist: SERVERS.get().unwrap().0.clone(),
        account_id: req.arg2.unwrap().parse::<u32>().unwrap(),
        server_ticket: req.arg3.unwrap(),
        notice_list: vec![],
        device: req.device,
        limit_server_ids: vec![],
    }
}

pub fn check_server_state(_req: Cs10018) -> Sc10019 {
    Sc10019 { serverlist: SERVERS.get().unwrap().0.clone() }
}

// Server list:
// status: NORMAL(0) | VINDICATE(1) | FULL(2) | REGISTER_FULL(3)
