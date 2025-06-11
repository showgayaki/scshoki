use log::{debug, error, info};
use rusb::{Context, Device, Hotplug, HotplugBuilder, UsbContext};
use std::thread;
use tauri::AppHandle;
use thirtyfour::prelude::*;

use crate::constants::DEVICE_DENSITY;

use super::infrastructure::detect::{detect_device, emit_device_event};
use super::infrastructure::display::{get_android_density, get_ios_screen_info};

struct UsbEventHandler {
    pub app_handle: AppHandle,
}

impl<T: UsbContext> Hotplug<T> for UsbEventHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        debug!("device_arrived called!!!");
        if let Ok(()) = detect_device(&device) {
            emit_device_event(&self.app_handle, "connected");
        }
    }

    fn device_left(&mut self, _device: Device<T>) {
        debug!("device_left called!!!");
        emit_device_event(&self.app_handle, "disconnected");
    }
}

pub fn start_usb_hotplug_monitor(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let context = Context::new().expect("Failed to create libusb context");

        if !rusb::has_hotplug() {
            println!("Hotplug not supported on this platform.");
            return;
        }

        let callback = Box::new(UsbEventHandler {
            app_handle: app_handle.clone(),
        });

        let _registration: rusb::Registration<rusb::Context> = HotplugBuilder::new()
            .enumerate(true)
            .register(&context, callback)
            .expect("Failed to register hotplug callback");

        loop {
            if let Err(e) = context.handle_events(None) {
                eprintln!("Hotplug event error: {}", e);
            }
        }
    });
}

/// OSを指定してdensityを取得する関数
pub async fn get_display_info(driver: &WebDriver, os: &str) {
    debug!("get_physical_density(OS: {}) called!!!", os);
    let density = match os {
        "Android" => get_android_density().map_err(|_| "Failed to get Android density"),
        "iOS" => get_ios_screen_info(driver)
            .await
            .map_err(|_| "Failed to get iOS density"),
        _ => {
            error!("Unsupported OS({}): Failed to get density", os);
            Err("Unsupported OS")
        }
    };

    let mut density_cache = DEVICE_DENSITY.lock().unwrap();
    if let Ok(val) = density {
        *density_cache = val;
        info!("{} density: {:.1}", os, val);
    } else {
        *density_cache = 2.0;
        error!("{} density: failed to retrieve", os);
    }
}
