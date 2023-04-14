// use std::{ptr::copy_nonoverlapping, sync::Arc};

use core::ptr;
use std::{ffi::CString, thread, time::Duration};

use anyhow::Result;
// use camera::camera_init;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;
use network::{build_client, get_request, post_request, wifi};

use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use crate::{camera::camera_init, network::{jpg_stream_httpd_handler, default_configuration}};
// use test::{test_fs, test_tcp};

pub mod camera;
pub mod network;
// pub mod test;

pub const SSID: &str = r#"WEACSOFT"#;
pub const PASS: &str = r#"#weacsoft#417"#;

fn main() -> Result<()> {
  esp_idf_sys::link_patches();
  let peripherals = Peripherals::take().unwrap();

  /* camera initialize */
  camera_init();

  /* wifi initialize */
  let sysloop = EspSystemEventLoop::take()?;
  let default_nvs = EspDefaultNvsPartition::take().ok();
  let mut _wifi = wifi(peripherals.modem, sysloop, default_nvs)?;

  /* webserver */
  let c_str = CString::new("/stream.jpg").unwrap();
  let uri_handler_jpg: esp_idf_sys::httpd_uri_t = esp_idf_sys::httpd_uri_t {
    uri: c_str.as_ptr(),
    method: esp_idf_sys::http_method_HTTP_GET,
    handler: Some(jpg_stream_httpd_handler),
    user_ctx: ptr::null_mut(),
  };
  let mut server: esp_idf_sys::httpd_handle_t = ptr::null_mut();
  let server_ref = &mut server;

  let config: esp_idf_sys::httpd_config_t = default_configuration(80, 443);
  println!("{:?}", config);
  let status = unsafe { esp_idf_sys::httpd_start(server_ref, &config) };
  println!("{}--{:?}", status, server);
  unsafe { esp_idf_sys::httpd_register_uri_handler(server, &uri_handler_jpg) };

  // test_fs()?;
  // test_tcp("0.0.0.0", "8080", "/esp");
  // test_tcp_bind()?;

  // let mut client = build_client()?;
  // get_request(&mut client)?;
  // post_request(&mut client)?;

  loop {
    thread::sleep(Duration::from_secs(20));
    println!("rebooting eps32");
    thread::sleep(Duration::from_secs(5));
  }
}
