use super::{Address, BiStream, Connection};
use bytes::Bytes;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender},
    oneshot::{self, Receiver as OneshotReceiver, Sender as OneshotSender},
    Mutex as AsyncMutex,
};

type ConnectResponseSender = OneshotSender<BiStream>;
type ConnectResponseReceiver = OneshotReceiver<BiStream>;
type AssociateSendPacketSender = MpscSender<(Bytes, Address)>;
type AssociateSendPacketReceiver = MpscReceiver<(Bytes, Address)>;
type AssociateRecvPacketSender = MpscSender<(Bytes, Address)>;
type AssociateRecvPacketReceiver = MpscReceiver<(Bytes, Address)>;

pub async fn listen_request(conn: Arc<AsyncMutex<Connection>>, mut req_rx: MpscReceiver<Request>) {
    while let Some(req) = req_rx.recv().await {}
}

pub enum Request {
    Connect {
        addr: Address,
        tx: ConnectResponseSender,
    },
    Associate {
        assoc_id: u32,
        pkt_send_rx: AssociateSendPacketReceiver,
        pkt_receive_tx: AssociateRecvPacketSender,
    },
}

impl Request {
    pub fn new_connect(addr: Address) -> (Self, ConnectResponseReceiver) {
        let (tx, rx) = oneshot::channel();
        (Request::Connect { addr, tx }, rx)
    }

    pub fn new_associate() -> (Self, AssociateSendPacketSender, AssociateRecvPacketReceiver) {
        let assoc_id = get_random_u32();
        let (pkt_send_tx, pkt_send_rx) = mpsc::channel(1);
        let (pkt_receive_tx, pkt_receive_rx) = mpsc::channel(1);

        (
            Self::Associate {
                assoc_id,
                pkt_send_rx,
                pkt_receive_tx,
            },
            pkt_send_tx,
            pkt_receive_rx,
        )
    }
}

static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));

fn get_random_u32() -> u32 {
    RNG.lock().next_u32()
}