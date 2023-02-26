use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    ops::AddAssign,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, PlayerTx>>>;

#[derive(Debug)]
struct PlayerTx {
    tx: Tx,
    data: PlayerData,
}

#[derive(Debug)]
struct PlayerData {
    player_id: usize,
    pallet_pos: f32,
}

impl PlayerTx {
    fn new(tx: Tx, id: usize) -> Self {
        Self {
            tx,
            data: PlayerData {
                player_id: id,
                pallet_pos: 0.0,
            },
        }
    }
}

async fn handle_connection(
    peer_map: PeerMap,
    next_id: Arc<Mutex<usize>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();

    send_state(peer_map.clone(), tx.clone()).await;
    peer_map
        .lock()
        .unwrap()
        .insert(addr, PlayerTx::new(tx, *next_id.lock().unwrap()));

    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        let msg_text = msg.to_text().unwrap();
        let msg_vec: Vec<&str> = msg_text.split(',').collect();
        match msg_vec[0] {
            "pallet_pos" => {
                let mut peers = peer_map.lock().unwrap();
                let player = peers.get_mut(&addr).unwrap();
                player.data.pallet_pos = msg_vec[1].parse::<f32>().unwrap();
            }
            _ => {}
        }
        println!("Received a message from {}: {:?}", addr, msg_vec);

        let peers = peer_map.lock().unwrap();
        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, player)| player);

        let player = peers.get(&addr).unwrap();
        let response_msg = Message::Text(format!(
            "{},{},{}",
            "pallet_pos", player.data.player_id, player.data.pallet_pos
        ));
        for recp in broadcast_recipients {
            recp.tx.unbounded_send(response_msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

async fn send_state(peer_map: PeerMap, tx: Tx) {
    for (_, player) in peer_map.lock().unwrap().iter() {
        let msg = Message::Text(format!(
            "{},{},{}",
            "pallet_pos", player.data.player_id, player.data.pallet_pos
        ));
        tx.unbounded_send(msg).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let next_id = Arc::new(Mutex::new(0));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        next_id.lock().unwrap().add_assign(1);

        tokio::spawn(handle_connection(
            state.clone(),
            next_id.clone(),
            stream,
            addr,
        ));
    }

    Ok(())
}
