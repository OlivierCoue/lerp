use std::sync::{mpsc, Arc, Mutex};
use tokio::sync::oneshot;

#[derive(Clone, Debug)]
pub struct StateUser {
    pub uuid: String,
    pub username: String,
    pub auth_token: String,
}

pub enum LocalToGlobalEvent {
    SetUser(oneshot::Sender<()>, Option<StateUser>),
}

pub struct GlobalStateInner {
    user: Option<StateUser>,
}

#[derive(Clone)]
pub struct GlobalState {
    inner: Arc<Mutex<GlobalStateInner>>,
    tx_local_to_global_state_events: mpsc::Sender<LocalToGlobalEvent>,
}
impl GlobalState {
    pub fn new(tx_local_to_global_state_events: mpsc::Sender<LocalToGlobalEvent>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(GlobalStateInner { user: None })),
            tx_local_to_global_state_events,
        }
    }

    pub fn get_user(&self) -> Option<StateUser> {
        self.inner.lock().unwrap().user.clone()
    }

    pub async fn set_user(&mut self, user: Option<StateUser>) {
        let (tx, rx) = oneshot::channel();
        self.tx_local_to_global_state_events
            .send(LocalToGlobalEvent::SetUser(tx, user))
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
        rx_local_to_global_state_events: mpsc::Receiver<LocalToGlobalEvent>,
    ) {
        for event in &rx_local_to_global_state_events {
            match event {
                LocalToGlobalEvent::SetUser(tx, user) => {
                    self.set_user(user);
                    tx.send(()).unwrap();
                }
            }
        }
    }

    fn set_user(&mut self, user: Option<StateUser>) {
        self.global_state.inner.lock().unwrap().user = user;
    }
}
