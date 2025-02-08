mod proto_handlers;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use common::logging;
use network::packet::Packet;

pub async fn handler(mut stream: TcpStream, _state: ()) {
    let mut buffer = [0; 2048];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                logging::info!("Client disconnected");
                break;
            }
            Ok(size) => {
                logging::debug!("Received {} bytes", size);
                let req = Packet::new((&buffer[0..size]).to_vec()).ok();
                if let Some(rsp) = packet_handler(req) {
                    if let Err(e) = stream.write_all(&rsp.to_bytes().unwrap()).await {
                        logging::error!("Failed to send response: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                logging::error!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

macro_rules! gen_packet_handler {
    ($($set:ident::$name:ident,$category:ident::$func_name:ident;)*) => {
        fn packet_handler(packet: Option<Packet>) -> Option<Packet> {
            use proto::CmdID;
            match packet {
                None => None,
                Some(packet) => {
                    match packet.cmd_id {
                        $(
                            proto::$set::$name::CMD_ID => {
                                if let Some(req) = packet.decode::<proto::$set::$name>() {
                                    logging::debug!("-> Req: {:?}", req);
                                    let rsp = crate::handler::$category::$func_name(req);
                                    logging::debug!("<- Rsp: {:?}", rsp);
                                    Some(Packet::encode(&rsp, packet.id))
                                } else { None }
                            }
                        )*
                        _ => {
                            logging::error!("Invalid command id: {}", packet.cmd_id);
                            None
                        }
                    }
                }
            }
        }
    };
}

gen_packet_handler! {
    // il2cpp
    p10::Cs10800, proto_handlers::get_assets_hash;
    // lua
    p10::Cs10020, proto_handlers::user_login;
    p10::Cs10018, proto_handlers::check_server_state;
}
