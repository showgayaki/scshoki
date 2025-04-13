use log::{error, info};

use super::constants::{DEVICE_DENSITY, DEVICE_OS, DEVICE_UDID, IOS_VERSION};
use super::density::get_physical_density;
use super::os::{detect_device_os, ios_version};
use super::udid::get_udid;

use rusb::{Context, Device, Hotplug, HotplugBuilder, UsbContext};
use std::thread;

pub fn start_usb_hotplug_monitor() {
    // USBのHotplug監視を別スレッドで実行（非同期イベント処理）
    thread::spawn(move || {
        let context = Context::new().expect("Failed to create libusb context");

        if !rusb::has_hotplug() {
            println!("Hotplug not supported on this platform.");
            return;
        }

        // Hotplug イベントハンドラを登録
        let callback = Box::new(UsbEventHandler);
        let _registration: rusb::Registration<rusb::Context> = HotplugBuilder::new()
            .enumerate(true)
            .register(&context, callback)
            .expect("Failed to register hotplug callback");

        // イベントループ
        loop {
            if let Err(e) = context.handle_events(None) {
                eprintln!("Hotplug event error: {}", e);
            }
        }
    });
}

// コールバック実装
struct UsbEventHandler;

impl<T: UsbContext> Hotplug<T> for UsbEventHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        if let Ok(desc) = device.device_descriptor() {
            info!("{:?} connected", DEVICE_OS);
            detect_device();
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        if let Ok(desc) = device.device_descriptor() {
            info!("{:?} disconnected", DEVICE_OS);
        }
    }
}

fn detect_device() {
    // USBで接続されたデバイスを取得
    match detect_device_os() {
        Ok(os) => {
            // デバイスOSを更新
            let mut device_os_lock = DEVICE_OS.lock().unwrap();
            *device_os_lock = Some(os.clone());

            // DPIスケールを取得
            let mut density_cache = DEVICE_DENSITY.lock().unwrap();
            match get_physical_density(&os) {
                Ok(density) => {
                    // デバイスの物理密度を取得
                    *density_cache = Some(density);
                }
                Err(ref e) => error!("Failed to get density: {}", e),
            }

            if os == "iOS" {
                // iOSのバージョンを取得
                match ios_version() {
                    Ok(version) => {
                        // iOSのバージョンを更新
                        let mut ios_version_lock = IOS_VERSION.lock().unwrap();
                        *ios_version_lock = Some(version.clone());
                    }
                    Err(ref e) => error!("Failed to get iOS version: {}", e),
                }
                // UDIDを取得
                match get_udid() {
                    Ok(udid) => {
                        let mut udid_cache = DEVICE_UDID.lock().unwrap();
                        *udid_cache = Some(udid.clone());
                    }
                    Err(ref e) => error!("Failed to get UDID: {}", e),
                }
            }
        }
        Err(_) => {
            info!("No new device detected.");
        }
    }
}
