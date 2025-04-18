use log::{debug, error, info};
use rusb::{Context, Device, Hotplug, HotplugBuilder, UsbContext};
use std::thread;
use tauri::{AppHandle, Emitter};

use super::constants::{
    DEFAULT_DEVICE_VALUE, DEVICE_DENSITY, DEVICE_MANUFACTURE, DEVICE_OS, DEVICE_PRODUCT_NAME,
    DEVICE_UDID, IOS_VERSION,
};
use super::density::get_physical_density;
use super::info::detect_device_info;
use super::ios::{get_udid, ios_version};
use crate::utils::retry::retry;

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

fn emit_device_event(app_handle: &AppHandle, event_type: &str) {
    debug!("emit_device_event called!!!");
    let os = DEVICE_OS.lock().unwrap();
    let product_name = DEVICE_PRODUCT_NAME.lock().unwrap();
    let manufacturer = DEVICE_MANUFACTURE.lock().unwrap();

    if os.as_deref() == Some("iOS")
        && (product_name.as_deref() == Some(DEFAULT_DEVICE_VALUE)
            || manufacturer.as_deref() == Some(DEFAULT_DEVICE_VALUE))
    {
        return;
    }

    if let (Some(os), Some(product_name), Some(manufacturer)) =
        (&*os, &*product_name, &*manufacturer)
    {
        let message = format!("{}({}: {}) {}", os, manufacturer, product_name, event_type);
        info!("{}", message);
        let _ = app_handle.emit(&format!("device_{}", event_type), message);
    }
}

fn detect_device<T: UsbContext>(device: &Device<T>) -> Result<(), String> {
    match detect_device_info(device) {
        Ok((os, product_name, manufacturer)) => {
            let mut device_os_lock = DEVICE_OS.lock().unwrap();
            let mut device_product_name_lock = DEVICE_PRODUCT_NAME.lock().unwrap();
            let mut device_manufacturer_lock = DEVICE_MANUFACTURE.lock().unwrap();

            *device_os_lock = Some(os.clone());
            *device_product_name_lock = Some(product_name.clone());
            *device_manufacturer_lock = Some(manufacturer.clone());

            let mut density_cache = DEVICE_DENSITY.lock().unwrap();
            match get_physical_density(&os) {
                Ok(density) => *density_cache = Some(density),
                Err(e) => {
                    *density_cache = Some(1.0);
                    error!("{}. Set default density {:?}", e, &density_cache);
                }
            }

            if os == "iOS" {
                match retry(ios_version, 5, 300) {
                    Ok(version) => {
                        let mut ios_version_lock = IOS_VERSION.lock().unwrap();
                        *ios_version_lock = Some(version.clone());
                    }
                    Err(ref e) => error!("Failed to get iOS version: {}", e),
                }
                match retry(get_udid, 5, 300) {
                    Ok(udid) => {
                        let mut udid_cache = DEVICE_UDID.lock().unwrap();
                        *udid_cache = Some(udid.clone());
                    }
                    Err(ref e) => error!("Failed to get UDID: {}", e),
                }
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
