use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;



#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum State {
    Init,
    Running,
    Stopped,
}

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Proposal,
    Acknowledgement,
    Commit,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    sender_id: u64,
    message_type: MessageType,
    proposed_state: State,
    proposal_id: String,
}

struct Node {
    id: u64,
    state: Arc<Mutex<State>>,
    peers: HashMap<u64, String>,
    address: String,
    tx: mpsc::Sender<Message>,
    proposal_acknowledgments: Arc<Mutex<HashMap<String, HashSet<u64>>>>,
}
fn main() {
    println!("Hello, world!");
}