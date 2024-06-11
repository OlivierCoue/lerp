use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

#[derive(Clone, Debug)]
pub struct StateUser {
    pub uuid: String,
    pub username: String,
    pub auth_token: String,
    pub game_server_handshake_challenge: String,
}

pub enum LocalToGlobalEvent {
    SetUser(oneshot::Sender<()>, Option<StateUser>),
    SetIsConnectedToGameServer(oneshot::Sender<()>, bool),
}

pub struct GlobalStateInner {
    user: Option<StateUser>,
    is_connected_to_game_server: bool,
}

#[derive(Clone)]
pub struct GlobalState {
    inner: Arc<Mutex<GlobalStateInner>>,
    tx_local_to_global_state_events: crossbeam_channel::Sender<LocalToGlobalEvent>,
}
impl GlobalState {
    pub fn new(
        tx_local_to_global_state_events: crossbeam_channel::Sender<LocalToGlobalEvent>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(GlobalStateInner {
                user: None,
                is_connected_to_game_server: false,
            })),
            tx_local_to_global_state_events,
        }
    }

    pub fn get_user(&self) -> Option<StateUser> {
        self.inner.lock().unwrap().user.clone()
    }

    pub fn get_is_connected_to_game_server(&self) -> bool {
        self.inner.lock().unwrap().is_connected_to_game_server
    }

    pub async fn set_user(&mut self, user: Option<StateUser>) {
        let (tx, rx) = oneshot::channel();
        self.tx_local_to_global_state_events
            .send(LocalToGlobalEvent::SetUser(tx, user))
            .unwrap();
        rx.await.unwrap();
    }

    pub async fn set_is_connected_to_game_server(&mut self, is_connected_to_game_server: bool) {
        let (tx, rx) = oneshot::channel();
        self.tx_local_to_global_state_events
            .send(LocalToGlobalEvent::SetIsConnectedToGameServer(
                tx,
                is_connected_to_game_server,
            ))
            .unwrap();
        rx.await.unwrap();
    }
}

pub struct GlobalStateManager {
    global_state: GlobalState,
}
impl GlobalStateManager {
    pub fn new(global_state: GlobalState) -> Self {
        Self { global_state }
    }

    pub async fn start(
        &mut self,
        rx_local_to_global_state_events: crossbeam_channel::Receiver<LocalToGlobalEvent>,
    ) {
        for event in &rx_local_to_global_state_events {
            match event {
                LocalToGlobalEvent::SetUser(tx, user) => {
                    self.set_user(user);
                    tx.send(()).unwrap();
                }
                LocalToGlobalEvent::SetIsConnectedToGameServer(tx, is_connected_to_game_server) => {
                    self.set_is_connected_to_game_server(is_connected_to_game_server);
                    tx.send(()).unwrap();
                }
            }
        }
    }

    fn set_user(&mut self, user: Option<StateUser>) {
        let mut inner_lock = self.global_state.inner.lock().unwrap();
        if user.is_none() {
            inner_lock.is_connected_to_game_server = false;
        }
        inner_lock.user = user;
    }

    fn set_is_connected_to_game_server(&mut self, is_connected_to_game_server: bool) {
        let mut inner_lock = self.global_state.inner.lock().unwrap();
        inner_lock.is_connected_to_game_server = is_connected_to_game_server;
    }
}
