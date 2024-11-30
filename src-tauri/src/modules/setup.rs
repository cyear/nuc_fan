use std::{fs, process, thread, time::Duration};
use tauri::{App, Manager};
use std::error::Error;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::plugin::PermissionState;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconBuilder};
use tauri_plugin_notification::NotificationExt;
use crate::{fan_reset, get_config_dir};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let config_tdp = get_config_dir().join("debug.config");
    if config_tdp.exists() {
        let debug = fs::read_to_string(config_tdp).map_err(|e| e.to_string()).unwrap().parse::<i64>().unwrap();
        if debug == 1 {
            let w = app.get_webview_window("tdp").unwrap();
            window_vibrancy::apply_acrylic(&w, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            w.show().unwrap();
        }
    }
    if app.notification().permission_state()? == PermissionState::Denied {
        app.notification().request_permission()?;
    }
    if app.notification().permission_state()? == PermissionState::Granted {
        app.notification()
            .builder()
            .body("可以隐藏到托盘图标，退出前请点击stop按钮!")
            .show()?;
    }
    let window = app.get_webview_window("main").unwrap();
    let h = MenuItemBuilder::with_id("h", "显示界面").build(app)?;
    let q = MenuItemBuilder::with_id("q", "退出程序").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&h, &q]).build()?;
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .title("NUC X15 Fan")
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "h" => {
                println!("显示 clicked");
                if let Some(webview_window) = app.get_webview_window("main") {
                    let _ = webview_window.show();
                    let _ = webview_window.set_focus();
                }
            },
            "q" => {
                if app.notification().permission_state().unwrap() == PermissionState::Granted {
                    app.notification()
                        .builder()
                        .body("安全退出！")
                        .show().unwrap();
                }
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(1));
                    fan_reset();
                    println!("退出");
                    process::exit(0);
                });
            }
            _ => (),
        })
        .on_tray_icon_event(|_tray, event| {
            if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
            } = event {

            }
        })
        .build(app)?;
    window_vibrancy::apply_acrylic(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    Ok(())
}