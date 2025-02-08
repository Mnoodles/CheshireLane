use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use chrono::{Datelike, Duration, Utc};
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::common::Kvdata;
use proto::p27::{ChildAttr, ChildFavor, ChildInfo, ChildTask, ChildTime, Cs27000, Cs27010, Sc27001, Sc27011};
use proto::p29::{Cs29001, Sc29002, Tbbenefit, Tbbf, Tbinfo, Tbpermanent, Tbres, Tbround};

pub fn educate_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs27000>() {
            logging::debug!("EducateRequest: {:?}", req);

            let now = Utc::now();
            let month = now.month();
            let first_day = now.with_day(1).unwrap();
            let week = ((now.day() + first_day.weekday().num_days_from_sunday() - 1) / 7) + 1;
            let yesterday = now - Duration::days(1);
            let day = yesterday.weekday().num_days_from_sunday();
            message_output.send(Sc27001 {
                child: ChildInfo {
                    attrs: (101..=104)
                        .chain(201..=203)
                        .chain(301..=306)
                        .map(|id| ChildAttr { id, val: 0 }).collect(),
                    mood: 50,
                    new_game_plus_count: 1,
                    money: 20,
                    tasks: (101..=103).map(|id| ChildTask {
                        id,
                        progress: 0,
                    }).collect(),
                    can_trigger_home_event: 1,
                    cur_time: ChildTime {
                        day,
                        week,
                        month,
                    },
                    favor: ChildFavor {
                        exp: 30,
                        lv: 1,
                    },
                    tid: 1,
                    ..Default::default()
                },
                result: 0,
            }, message.id());
        }
    }
}

pub fn new_educate_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs29001>() {
            logging::debug!("NewEducateRequest: {:?}", req);

            message_output.send(Sc29002 {
                permanent: Tbpermanent {
                    ng_plus_count: 1,
                    ..Default::default()
                },
                result: 0,
                tb: Tbinfo {
                    id: 1,
                    res: Tbres {
                        attrs: (101..=104).map(|key| Kvdata { key, value: 0 })
                            .chain(std::iter::once(Kvdata { key: 201, value: 155 }))
                            .collect(),
                        resource: (1..=4).map(|key| Kvdata {
                            key,
                            value: if key == 3 { 0 } else { 50 },
                        }).collect(),
                    },
                    round: Tbround {
                        round: 1,
                    },
                    benefit: Tbbenefit {
                        actives: vec![Tbbf {
                            id: 10000,
                            round: 1,
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }, message.id());
        }
    }
}

pub fn get_endings(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs27010>() {
            logging::debug!("GetEndings: {:?}", req);

            message_output.send(Sc27011::default(), message.id());
        }
    }
}
