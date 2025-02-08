use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p11::{Noticeinfo, Sc11300};

pub struct ServerNoticeRegisterPlugin;

impl Plugin for ServerNoticeRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyServerNoticeRegisterEvent>()
            .add_systems(Update, notify_server_notice_register);
    }
}

#[derive(Event)]
pub struct NotifyServerNoticeRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_server_notice_register(
    mut events: EventReader<NotifyServerNoticeRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc11300 {
                    notice_list: vec![
                        Noticeinfo {
                            tag_type: 1,
                            id: 6,
                            version: "1".to_string(),
                            icon: 2,
                            content: CONTENT.to_string(),
                            title: "Welcome to Cheshire Lane | 欢迎使用 Cheshire Lane".to_string(),
                            track: "".to_string(),
                            priority: 80,
                            btn_title: "Welcome".to_string(),
                            title_image: "https://azurusapi.yo-star.com/static/cheshire-banner.png"
                                .to_string(), // kjmkł <— Coding by a cute cat🐱
                            time_desc: "2/1/2025".to_string(),
                        },
                    ],
                },
                event.1,
                event.2,
                event.3);
        }
    }
}

const CONTENT: &str = r#"
<size=35>Disclaimer</size>
        ※This project is intended for educational and research purposes only. Do not use it for any illegal or inappropriate activities.
        ※When using this project, ensure compliance with local laws and regulations and take full responsibility for your actions.
        ※The author is not responsible for any misuse, illegal use, or consequences arising from it.
        ※This project is open-source and free. If you have paid to use this software, please request a refund immediately.

<size=35>免责声明</size>
        ※本项目仅用于教育和研究目的，请勿将其用于任何非法或不当用途。
        ※使用本项目时，请确保遵守所在地区的法律法规，并承担相应责任。
        ※作者不对任何滥用、违法使用或由此产生的后果负责。
        ※本项目为开源免费项目，如您因使用本软件而支付费用，请立即申请退款。
"#;
