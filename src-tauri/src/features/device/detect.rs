use log::{error, info};
use rusb::{Context, Device, Hotplug, HotplugBuilder, UsbContext};
use std::thread;

use super::constants::{
    DEVICE_DENSITY, DEVICE_MANUFACTURE, DEVICE_OS, DEVICE_PRODUCT_NAME, DEVICE_UDID, IOS_VERSION,
};
use super::density::get_physical_density;
use super::info::{detect_device_info, ios_version};
use super::udid::get_udid;

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
        if let Ok(_desc) = device.device_descriptor() {
            detect_device();
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        if let Ok(_desc) = device.device_descriptor() {
            let os = DEVICE_OS.lock().unwrap();
            let name = DEVICE_PRODUCT_NAME.lock().unwrap();
            let manufacturer = DEVICE_MANUFACTURE.lock().unwrap();

            if let (Some(os), Some(name), Some(manufacturer)) = (&*os, &*name, &*manufacturer) {
                info!("{}({}) {} disconnected", os, name, manufacturer);
            }
        }
    }
}

fn detect_device() {
    // USBで接続されたデバイスを取得
    match detect_device_info() {
        Ok((os, product_name, manufacturer)) => {
            // デバイス情報を更新
            let mut device_os_lock = DEVICE_OS.lock().unwrap();
            let mut device_product_name_lock = DEVICE_PRODUCT_NAME.lock().unwrap();
            let mut device_manufacturer_lock = DEVICE_MANUFACTURE.lock().unwrap();

            *device_os_lock = Some(os.clone());
            *device_product_name_lock = Some(product_name.clone());
            *device_manufacturer_lock = Some(manufacturer.clone());

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
