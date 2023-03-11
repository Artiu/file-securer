#![windows_subsystem = "windows"]

mod images;

use std::{path::Path, sync};

use auto_launch::AutoLaunchBuilder;
use base64::{engine::general_purpose, Engine};
use directories::UserDirs;
use images::{AMOGUS_BASE64, FBI_BASE64, SUSSY_BASE64};
use notify::{EventKind, Watcher};
use windows::{
    core::PCWSTR,
    w,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_ICONWARNING, MESSAGEBOX_STYLE},
};

fn open_base64_image(name: &str, encoded_string: &str) {
    let buffer = general_purpose::STANDARD.decode(encoded_string).unwrap();
    let img = image::load_from_memory(&buffer).unwrap();
    let path = UserDirs::new()
        .unwrap()
        .picture_dir()
        .unwrap()
        .join(name.to_owned() + ".png");
    img.save(&path).unwrap();
    open::that(path).unwrap();
}

fn create_message_box(title: PCWSTR, text: PCWSTR, style: MESSAGEBOX_STYLE) {
    unsafe {
        MessageBoxW(None, text, title, style);
    }
}

fn on_modify_or_remove() {
    loop {
        create_message_box(
            w!("FBI"),
            w!("You modified file owned by FBI!"),
            MB_ICONERROR,
        );
        open_base64_image("FBI", FBI_BASE64);
        open_base64_image("sussy", SUSSY_BASE64);
        open_base64_image("amogus", AMOGUS_BASE64);
    }
}

fn handle_change_event(event: EventKind) {
    match event {
        EventKind::Access(_) => create_message_box(
            w!("FBI Alert"),
            w!("This file is owned by FBI!"),
            MB_ICONWARNING,
        ),
        EventKind::Create(_) => {
            create_message_box(w!("FBI Alert"), w!("We now own this file!"), MB_ICONWARNING)
        }
        EventKind::Modify(_) => on_modify_or_remove(),
        EventKind::Remove(_) => on_modify_or_remove(),
        _ => {}
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path_to_secure = args.get(1).unwrap();
    let exe_path = std::env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let auto_launch = AutoLaunchBuilder::new()
        .set_app_name("FBI")
        .set_app_path(&exe_path)
        .set_args(&[path_to_secure])
        .build()
        .unwrap();

    if !auto_launch.is_enabled().unwrap() {
        auto_launch.enable().unwrap();
    }

    let (tx, rx) = sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(tx).unwrap();

    watcher
        .watch(Path::new(path_to_secure), notify::RecursiveMode::Recursive)
        .unwrap();

    for res in rx {
        if let Ok(event) = res {
            handle_change_event(event.kind);
        }
    }
}
