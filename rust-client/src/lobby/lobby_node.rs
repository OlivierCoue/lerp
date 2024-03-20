use std::{
    sync::{Arc, Mutex},
    thread,
};

use godot::{
    engine::{Button, Label, LineEdit},
    prelude::*,
};
use rust_common::proto::{MsgUpHandshake, MsgUpWrapper, UdpMsgDownWrapper};

use crate::{
    global_state::GlobalState,
    root::{Root, Scenes, PATH_LOBBY, PATH_ROOT},
    udp::prelude::UdpState,
};

use super::lobby_state::{LobbyNodeEvent, LobbyState, LobbyStateEvent, LobbyStateManager};

#[derive(GodotClass)]
#[class(no_init, base=Node2D)]
pub struct LobbyNode {
    base: Base<Node2D>,
    global_state: GlobalState,
    state: Arc<Mutex<LobbyState>>,
    rx_state_events: crossbeam_channel::Receiver<LobbyStateEvent>,
    tx_node_events: crossbeam_channel::Sender<LobbyNodeEvent>,
    root: OnReady<Gd<Root>>,
    label_username: OnReady<Gd<Label>>,
    button_create_game: OnReady<Gd<Button>>,
    button_logout: OnReady<Gd<Button>>,
    line_edit_join_game_id: Option<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for LobbyNode {
    fn ready(&mut self) {
        self.root.init(self.base().get_node_as::<Root>(PATH_ROOT));

        let lobby_ui_scene = load::<PackedScene>("res://lobby_ui.tscn");
        let lobby_ui = lobby_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(lobby_ui.upcast());

        self.label_username.init(self.base().get_node_as::<Label>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/label_username",
        ));
        if let Some(current_user) = self.global_state.get_user() {
            self.label_username.set_text(current_user.username.into());
        }

        self.button_create_game
            .init(self.base().get_node_as::<Button>(
                String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_create_game",
            ));
        let on_button_create_game_pressed = self.base().callable("on_button_create_game_pressed");
        self.button_create_game
            .connect("pressed".into(), on_button_create_game_pressed);

        let mut button_join_game = self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_join_game",
        );
        button_join_game.connect(
            "pressed".into(),
            self.base().callable("on_button_join_game_pressed"),
        );

        self.button_logout.init(self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_logout",
        ));
        let on_button_logout_pressed = self.base().callable("on_button_logout_pressed");
        self.button_logout
            .connect("pressed".into(), on_button_logout_pressed);

        self.line_edit_join_game_id = Some(self.base().get_node_as::<LineEdit>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/line_edit_join_game_id",
        ));
    }

    fn process(&mut self, _: f64) {
        while let Ok(event) = self.rx_state_events.try_recv() {
            match event {
                LobbyStateEvent::LogoutSuccess => {
                    self.root.bind_mut().change_scene(Scenes::Auth);
                }
                LobbyStateEvent::IsLoadingChanged => self.on_is_loading_changed(),
                LobbyStateEvent::CreateWorldInstanceSuccess(id) => {
                    self.root.bind_mut().change_scene(Scenes::Play(id));
                }
            }
        }
    }
}

impl LobbyNode {
    pub fn init(base: Base<Node2D>, global_state: GlobalState, udp_state: UdpState) -> Self {
        let state = Arc::new(Mutex::new(LobbyState { is_loading: false }));

        let (tx_state_events, rx_state_events) = crossbeam_channel::unbounded();
        let (tx_node_events, rx_node_events) = crossbeam_channel::unbounded();

        let mut state_manager = LobbyStateManager::new(
            global_state.clone(),
            udp_state,
            state.clone(),
            tx_state_events,
        );

        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .thread_name("lobby-pool")
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    state_manager.start(rx_node_events).await;
                });
        });

        Self {
            base,
            global_state,
            state,
            rx_state_events,
            tx_node_events,
            root: OnReady::manual(),
            label_username: OnReady::manual(),
            button_create_game: OnReady::manual(),
            button_logout: OnReady::manual(),
            line_edit_join_game_id: None,
        }
    }

    fn on_is_loading_changed(&mut self) {
        let is_loading = self.state.lock().unwrap().is_loading;
        self.button_create_game.set_disabled(is_loading);
        self.button_logout.set_disabled(is_loading);
    }
}

#[godot_api]
impl LobbyNode {
    #[func]
    fn on_button_create_game_pressed(&mut self) {
        self.tx_node_events
            .send(LobbyNodeEvent::ButtonCreateGamePressed)
            .unwrap();
    }

    #[func]
    fn on_button_join_game_pressed(&mut self) {
        let input_wolrd_instance_id = self.line_edit_join_game_id.as_ref().unwrap().get_text();
        if input_wolrd_instance_id.is_empty() {
            return;
        }

        self.root
            .bind_mut()
            .change_scene(Scenes::Play(input_wolrd_instance_id.into()));
    }

    #[func]
    fn on_button_logout_pressed(&mut self) {
        self.tx_node_events
            .send(LobbyNodeEvent::ButtonLogoutPressed)
            .unwrap();
    }
}

impl Drop for LobbyNode {
    fn drop(&mut self) {
        self.tx_node_events.send(LobbyNodeEvent::Closed).unwrap();
    }
}
