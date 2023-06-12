use auto_launch::AutoLaunchBuilder;
use std::env::current_exe;
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
            let cloned_state = Arc::clone(&state);

            let app_name = &app.package_info().name;
            let current_exe = current_exe().unwrap();
            let auto_launch = AutoLaunchBuilder::new()
                .set_app_name(&app_name)
                .set_app_path(current_exe.to_str().unwrap())
                .set_use_launch_agent(true)
                .build()
                .unwrap();

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

            cloned_state.lock().unwrap().initialize();

            watch_cli(cloned_state);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Error while running tauri application");
    app.set_activation_policy(ActivationPolicy::Accessory);
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn watch_cli(cloned_state: Arc<Mutex<Numlock>>) {
    thread::spawn(move || {
        let socket_path = "/tmp/numlock-cli-socket";
        if std::fs::metadata(socket_path).is_ok() {
            println!("A socket is already present. Deleting...");
            std::fs::remove_file(socket_path).expect("Unable to delete file");
        }
        let listener = UnixListener::bind(socket_path).expect("Couldn't bind socket");
        println!("Socket open. Listening...");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1];
            stream.read(&mut buffer).expect("Unable to read data");
            let mut state = cloned_state.lock().unwrap();
            match &buffer {
                b"1" => state.set_state(true),
                b"0" => state.set_state(false),
                _ => (),
            };
        }
    });
}

pub struct Numlock {
    enabled: bool,
    app_handle: AppHandle,
}

impl Numlock {
    pub fn new(app_handle: AppHandle) -> Self {
        Numlock {
            app_handle,
            enabled: Command::new("jq")
            .arg(".variables.numlock")
            .arg("/Library/Application Support/org.pqrs/tmp/karabiner_grabber_manipulator_environment.json")
            .output()
            .expect("Failed to execute process").stdout == b"1\n",
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
        Command::new("/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli")
            .arg("--set-variables")
            .arg(format!("{{\"numlock\":{flag} }}"))
            .output()
            .expect("Failed to execute process");
        self.app_handle
            .tray_handle()
            .set_icon(tauri::Icon::Raw(if self.enabled {
                include_bytes!("../icons/enabled.png").to_vec()
            } else {
                include_bytes!("../icons/disabled.png").to_vec()
            }))
            .expect("Failed to set tray icon");
    }
}
