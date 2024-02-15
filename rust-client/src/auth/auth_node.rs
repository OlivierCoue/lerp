use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use godot::{
    engine::{Button, Label, LineEdit},
    prelude::*,
};

use crate::{
    global_state::GlobalState,
    network::prelude::*,
    root::{Root, Scenes, PATH_AUTH, PATH_NETWORK, PATH_ROOT},
};

use super::auth_state::{AuthNodeEvent, AuthState, AuthStateEvent, AuthStateManager};

#[derive(GodotClass)]
#[class(no_init, base=Node2D)]
pub struct AuthNode {
    base: Base<Node2D>,
    state: Arc<Mutex<AuthState>>,
    rx_state_events: Rc<crossbeam_channel::Receiver<AuthStateEvent>>,
    tx_node_events: crossbeam_channel::Sender<AuthNodeEvent>,
    root: OnReady<Gd<Root>>,
    network: OnReady<Gd<NetworkManager>>,
    label_auth_error: OnReady<Gd<Label>>,
    line_edit_username: OnReady<Gd<LineEdit>>,
    connect_button: OnReady<Gd<Button>>,
}

#[godot_api]
impl INode2D for AuthNode {
    fn ready(&mut self) {
        self.root.init(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network
            .init(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));

        let auth_ui_scene = load::<PackedScene>("res://auth_ui.tscn");
        let auth_ui = auth_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(auth_ui.upcast());

        self.label_auth_error.init(self.base().get_node_as::<Label>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/label_auth_error",
        ));
        self.line_edit_username
            .init(self.base().get_node_as::<LineEdit>(
                String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/line_edit_username",
            ));
        self.connect_button.init(self.base().get_node_as::<Button>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/connect_button",
        ));

        let on_connect_button_pressed = self.base().callable("on_connect_button_pressed");
        self.connect_button
            .connect("pressed".into(), on_connect_button_pressed);
    }

    fn process(&mut self, _: f64) {
        let rx_state_events = Rc::clone(&self.rx_state_events);
        while let Ok(state_event) = &rx_state_events.try_recv() {
            match state_event {
                AuthStateEvent::ConnectErrorChanged => self.on_connect_error_changed(),
                AuthStateEvent::IsLoadingChanged => self.on_is_loading_changed(),
                AuthStateEvent::ConnectSuccess => self.on_connect_success(),
            }
        }
    }
}

#[godot_api]
impl AuthNode {
    #[func]
    fn on_connect_button_pressed(&mut self) {
        self.tx_node_events
            .send(AuthNodeEvent::ConnectButtonPressed(
                self.line_edit_username.get_text().to_string(),
            ))
            .unwrap();
    }
}

impl AuthNode {
    pub fn init(base: Base<Node2D>, global_state: GlobalState) -> Self {
        let state = Arc::new(Mutex::new(AuthState {
            is_loading: false,
            connect_error: None,
        }));

        let (tx_state_events, rx_state_events) = crossbeam_channel::unbounded();
        let (tx_node_events, rx_node_events) = crossbeam_channel::unbounded();

        let mut state_manager = AuthStateManager::new(global_state, state.clone(), tx_state_events);
        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .thread_name("auth-pool")
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    state_manager.start(rx_node_events).await;
                });
        });

        Self {
            base,
            state,
            rx_state_events: Rc::new(rx_state_events),
            tx_node_events,
            root: OnReady::manual(),
            network: OnReady::manual(),
            label_auth_error: OnReady::manual(),
            line_edit_username: OnReady::manual(),
            connect_button: OnReady::manual(),
        }
    }

    fn on_connect_error_changed(&mut self) {
        let connect_error = &self.state.lock().unwrap().connect_error;
        if let Some(connect_error) = connect_error {
            self.label_auth_error.set_text(connect_error.into());
        } else {
            self.label_auth_error.set_text("".into());
        }
    }

    fn on_is_loading_changed(&mut self) {
        let is_loading = self.state.lock().unwrap().is_loading;
        self.connect_button.set_disabled(is_loading);
    }

    fn on_connect_success(&mut self) {
        self.root.bind_mut().change_scene(Scenes::Lobby);
    }
}

impl Drop for AuthNode {
    fn drop(&mut self) {
        self.tx_node_events.send(AuthNodeEvent::Closed).unwrap();
    }
}
