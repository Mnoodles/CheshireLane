use ecs_command::CommandKind;
use ecs_message::output::ClientOutput;
use ecs_persistence::player_info::PlayerInfo;

pub enum LogicCommand {
    CreateWorld {
        uid: u32,
        player_info: PlayerInfo,
        out: ClientOutput,
    },
    ClientInput {
        uid: u32,
        cmd_id: u16,
        id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    },
    WorldUpdate(u32),
    ExecuteCommand {
        uid: u32,
        kind: CommandKind,
    },
}
