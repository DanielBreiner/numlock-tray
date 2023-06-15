use auto_launch::AutoLaunchBuilder;
use serde_json::Value;
use std::env::current_exe;
use std::fs::File;
use std::os::unix::net::UnixListener;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{io::prelude::Read, thread};
use tauri::{
    ActivationPolicy, AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

fn main() {
    let mut app = tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let state: Arc<Mutex<Numlock>> = Arc::new(Mutex::new(Numlock::new(handle)));
            let handle = app.handle();
            let state_ = Arc::clone(&state);

            let auto_launch = setup_auto_launch(app);

            let mut startup_menu_item =
                CustomMenuItem::new("startup".to_string(), "Run on startup");
            if auto_launch.is_enabled().unwrap() {
                startup_menu_item = startup_menu_item.selected();
            }

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(startup_menu_item)
                        .add_native_item(SystemTrayMenuItem::Separator)
                        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
                )
                .with_menu_on_left_click(false)
                .on_event(move |event| match event {
                    SystemTrayEvent::LeftClick { .. } => {
                        let mut state = state.lock().unwrap();
                        state.switch();
                    }
                    SystemTrayEvent::MenuItemClick { id, .. } => {
                        if id == "quit" {
                            handle.exit(0);
                        } else if id == "startup" {
                            if auto_launch.is_enabled().unwrap() {
                                auto_launch.disable().unwrap();
                                handle
                                    .tray_handle()
                                    .get_item(id.as_str())
                                    .set_selected(false)
                                    .unwrap();
                            } else {
                                auto_launch.enable().unwrap();
                                handle
                                    .tray_handle()
                                    .get_item(id.as_str())
                                    .set_selected(true)
                                    .unwrap();
                            }
                        }
                    }
                    _ => (),
                })
                .build(app)?;

            state_.lock().unwrap().initialize();

            watch_cli(state_);

            Ok(())
        })
        .build(tauri::generate_context!())
        .unwrap();
    app.set_activation_policy(ActivationPolicy::Accessory);
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn setup_auto_launch(app: &mut tauri::App) -> auto_launch::AutoLaunch {
    let app_name = &app.package_info().name;
    let current_exe = current_exe().unwrap();
    AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap()
}

fn watch_cli(state: Arc<Mutex<Numlock>>) {
    thread::spawn(move || {
        let socket_path = "/tmp/numlock-cli-socket";
        if std::fs::metadata(socket_path).is_ok() {
            println!("A socket is already present. Deleting...");
            std::fs::remove_file(socket_path).unwrap();
        }
        let listener = UnixListener::bind(socket_path).unwrap();
        println!("Socket open. Listening...");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1];
            stream.read(&mut buffer).unwrap();
            let mut state = state.lock().unwrap();
            match &buffer {
                b"1" => state.set_state(true),
                b"0" => state.set_state(false),
                _ => (),
            };
        }
    });
}

fn get_initial_enabled() -> bool {
    let mut file = File::open(
        "/Library/Application Support/org.pqrs/tmp/karabiner_grabber_manipulator_environment.json",
    )
    .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: Value = serde_json::from_str(&contents).unwrap();

    if let Some(variables) = data.get("variables") {
        if let Some(numlock) = variables.get("numlock") {
            if let Some(numlock_value) = numlock.as_i64() {
                return numlock_value == 1;
            }
        }
    }
    false
}

pub struct Numlock {
    enabled: bool,
    app_handle: AppHandle,
}

impl Numlock {
    pub fn new(app_handle: AppHandle) -> Self {
        Numlock {
            app_handle,
            enabled: get_initial_enabled(),
        }
    }

    pub fn initialize(&mut self) {
        self.set_state(self.enabled);
    }

    pub fn switch(&mut self) {
        self.set_state(!self.enabled);
    }

    pub fn set_state(&mut self, state: bool) {
        self.enabled = state;
        let flag = if self.enabled { "1" } else { "0" };
        self.app_handle
            .tray_handle()
            .set_icon(tauri::Icon::Raw(if self.enabled {
                include_bytes!("../icons/enabled.png").to_vec()
            } else {
                include_bytes!("../icons/disabled.png").to_vec()
            }))
            .unwrap();
        Command::new("/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli")
            .arg("--set-variables")
            .arg(format!("{{\"numlock\":{flag} }}"))
            .output()
            .unwrap();
    }
}
