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

    line_edit_login_username: OnReady<Gd<LineEdit>>,
    label_login_error: OnReady<Gd<Label>>,
    button_login: OnReady<Gd<Button>>,

    line_edit_register_username: OnReady<Gd<LineEdit>>,
    label_register_error: OnReady<Gd<Label>>,
    button_register: OnReady<Gd<Button>>,
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

        // LOGIN UI
        self.label_login_error
            .init(self.base().get_node_as::<Label>(
                String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/label_login_error",
            ));
        self.line_edit_login_username
            .init(self.base().get_node_as::<LineEdit>(
                String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/line_edit_login_username",
            ));
        self.button_login.init(self.base().get_node_as::<Button>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/button_login",
        ));

        let on_button_login_pressed = self.base().callable("on_button_login_pressed");
        self.button_login
            .connect("pressed".into(), on_button_login_pressed);

        // REGISTER UI
        self.label_register_error
            .init(self.base().get_node_as::<Label>(
                String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/label_register_error",
            ));
        self.line_edit_register_username
            .init(self.base().get_node_as::<LineEdit>(
                String::from(PATH_AUTH)
                    + "/AuthUi/Control/VBoxContainer/line_edit_register_username",
            ));
        self.button_register.init(self.base().get_node_as::<Button>(
            String::from(PATH_AUTH) + "/AuthUi/Control/VBoxContainer/button_register",
        ));

        let on_button_register_pressed = self.base().callable("on_button_register_pressed");
        self.button_register
            .connect("pressed".into(), on_button_register_pressed);
    }

    fn process(&mut self, _: f64) {
        let rx_state_events = Rc::clone(&self.rx_state_events);
        while let Ok(state_event) = &rx_state_events.try_recv() {
            match state_event {
                AuthStateEvent::IsLoadingChanged => self.on_is_loading_changed(),
                AuthStateEvent::LoginErrorChanged => self.on_login_error_changed(),
                AuthStateEvent::LoginSuccess => self.on_login_success(),
                AuthStateEvent::RegisterErrorChanged => self.on_register_error_changed(),
                AuthStateEvent::RegisterSuccess => self.on_register_success(),
            }
        }
    }
}

#[godot_api]
impl AuthNode {
    #[func]
    fn on_button_login_pressed(&mut self) {
        self.tx_node_events
            .send(AuthNodeEvent::LoginButtonPressed(
                self.line_edit_login_username.get_text().to_string(),
            ))
            .unwrap();
    }

    #[func]
    fn on_button_register_pressed(&mut self) {
        self.tx_node_events
            .send(AuthNodeEvent::RegisterButtonPressed(
                self.line_edit_register_username.get_text().to_string(),
            ))
            .unwrap();
    }
}

impl AuthNode {
    pub fn init(base: Base<Node2D>, global_state: GlobalState) -> Self {
        let state = Arc::new(Mutex::new(AuthState {
            is_loading: false,
            login_error: None,
            register_error: None,
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

            label_login_error: OnReady::manual(),
            line_edit_login_username: OnReady::manual(),
            button_login: OnReady::manual(),

            line_edit_register_username: OnReady::manual(),
            label_register_error: OnReady::manual(),
            button_register: OnReady::manual(),
        }
    }

    fn on_login_error_changed(&mut self) {
        let login_error = &self.state.lock().unwrap().login_error;
        if let Some(login_error) = login_error {
            self.label_login_error.set_text(login_error.into());
        } else {
            self.label_login_error.set_text("".into());
        }
    }

    fn on_register_error_changed(&mut self) {
        let register_error = &self.state.lock().unwrap().register_error;
        if let Some(register_error) = register_error {
            self.label_register_error.set_text(register_error.into());
        } else {
            self.label_register_error.set_text("".into());
        }
    }

    fn on_is_loading_changed(&mut self) {
        let is_loading = self.state.lock().unwrap().is_loading;
        self.button_login.set_disabled(is_loading);
        self.button_register.set_disabled(is_loading);
    }

    fn on_login_success(&mut self) {
        self.root.bind_mut().change_scene(Scenes::Lobby);
    }

    fn on_register_success(&mut self) {
        self.line_edit_register_username.set_text("".into());
    }
}

impl Drop for AuthNode {
    fn drop(&mut self) {
        self.tx_node_events.send(AuthNodeEvent::Closed).unwrap();
    }
}
