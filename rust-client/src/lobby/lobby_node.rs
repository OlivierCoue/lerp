use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use godot::{
    engine::{Button, Label, LineEdit},
    prelude::*,
};
use rust_common::proto::{MsgUp, MsgUpType, MsgUpWrapper};

use crate::{
    global_state::GlobalState,
    network::prelude::*,
    root::{Root, Scenes, PATH_LOBBY, PATH_NETWORK, PATH_ROOT},
};

use super::lobby_state::{LobbyNodeEvent, LobbyState, LobbyStateEvent, LobbyStateManager};

#[derive(GodotClass)]
#[class(no_init, base=Node2D)]
pub struct LobbyNode {
    base: Base<Node2D>,
    global_state: GlobalState,
    state: Arc<Mutex<LobbyState>>,
    rx_state_events: Rc<mpsc::Receiver<LobbyStateEvent>>,
    tx_node_events: mpsc::Sender<LobbyNodeEvent>,
    root: OnReady<Gd<Root>>,
    network: OnReady<Gd<NetworkManager>>,
    label_username: OnReady<Gd<Label>>,
    button_logout: OnReady<Gd<Button>>,
    line_edit_join_game_id: Option<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for LobbyNode {
    fn ready(&mut self) {
        self.root.init(self.base().get_node_as::<Root>(PATH_ROOT));
        self.network
            .init(self.base().get_node_as::<NetworkManager>(PATH_NETWORK));

        let lobby_ui_scene = load::<PackedScene>("res://lobby_ui.tscn");
        let lobby_ui = lobby_ui_scene.instantiate_as::<Node>();
        self.base_mut().add_child(lobby_ui.upcast());

        self.label_username.init(self.base().get_node_as::<Label>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/label_username",
        ));
        if let Some(current_user) = self.global_state.get_user() {
            self.label_username.set_text(current_user.username.into());
        }

        let mut button_create_game = self.base().get_node_as::<Button>(
            String::from(PATH_LOBBY) + "/LobbyUi/Control/VBoxContainer/button_create_game",
        );
        button_create_game.connect(
            "pressed".into(),
            self.base().callable("on_button_create_game_pressed"),
        );

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
        let rx_state_events = Rc::clone(&self.rx_state_events);
        while let Ok(event) = rx_state_events.try_recv() {
            match event {
                LobbyStateEvent::LogoutSuccess => {
                    self.root.bind_mut().change_scene(Scenes::Auth);
                }
                LobbyStateEvent::IsLoadingChanged => self.on_is_loading_changed(),
            }
        }

        // let rx_enet_receiver = Rc::clone(&self.network.bind().rx_udp_receiver);
        // while let Ok(udp_msg_down_wrapper) = rx_enet_receiver.try_recv() {
        //     for udp_msg_down in udp_msg_down_wrapper.messages {
        //         #[allow(clippy::single_match)]
        //         match UdpMsgDownType::try_from(udp_msg_down.r#type) {
        //             Ok(UdpMsgDownType::UserDisconnectSuccess) => {
        //                 self.root.bind_mut().change_scene(Scenes::Lobby);
        //             }
        //             Ok(UdpMsgDownType::UserCreateWordlInstanceSuccess) => {
        //                 let payload = udp_msg_down.user_create_world_instance_success.unwrap();
        //                 godot_print!("USER_CREATE_WORDL_INSTANCE_SUCCESS: (id: {})", payload.id);
        //                 self.root.bind_mut().change_scene(Scenes::Play(payload.id));
        //             }
        //             _ => {}
        //         }
        //     }
        // }
    }
}

impl LobbyNode {
    pub fn init(base: Base<Node2D>, global_state: GlobalState) -> Self {
        let state = Arc::new(Mutex::new(LobbyState { is_loading: false }));

        let (tx_state_events, rx_state_events) = mpsc::channel();
        let (tx_node_events, rx_node_events) = mpsc::channel();

        let mut state_manager =
            LobbyStateManager::new(global_state.clone(), state.clone(), tx_state_events);

        thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .max_blocking_threads(2)
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
            rx_state_events: Rc::new(rx_state_events),
            tx_node_events,
            root: OnReady::manual(),
            network: OnReady::manual(),
            label_username: OnReady::manual(),
            button_logout: OnReady::manual(),
            line_edit_join_game_id: None,
        }
    }

    fn on_is_loading_changed(&mut self) {
        let is_loading = self.state.lock().unwrap().is_loading;
        self.button_logout.set_disabled(is_loading);
    }
}

#[godot_api]
impl LobbyNode {
    #[func]
    fn on_button_create_game_pressed(&mut self) {
        self.network.bind().send_udp(MsgUpWrapper {
            messages: vec![MsgUp {
                r#type: MsgUpType::UserCreateWorldInstance.into(),
                ..Default::default()
            }],
        })
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
            .send(LobbyNodeEvent::LogoutButtonPressed)
            .unwrap();
    }
}

impl Drop for LobbyNode {
    fn drop(&mut self) {
        self.tx_node_events.send(LobbyNodeEvent::Closed).unwrap();
    }
}
