// use std::{ptr::copy_nonoverlapping, sync::Arc};

use anyhow::Result;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;
use network::wifi;

use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
mod camera;
mod network;

pub const SSID: &str = "wifi name";
pub const PASS: &str = "wifi password";

fn main() -> Result<()> {
  esp_idf_sys::link_patches();
  let peripherals = Peripherals::take().unwrap();

  let sysloop = EspSystemEventLoop::take()?;
  let default_nvs = EspDefaultNvsPartition::take().ok();
  let mut _wifi = wifi(peripherals.modem, sysloop, default_nvs)?;

  // let mut client =
  //   HttpClient::wrap(EspHttpConnection::new(&HttpConfiguration {
  //     crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
  //     ..Default::default()
  //   })?);

  // // GET
  // get_request(&mut client)?;

  // // POST
  // post_request(&mut client)?;

  /* camera */
  // let camera_config = esp_idf_sys::camera_config_t {
  //   pin_pwdn: 32,
  //   pin_reset: -1,
  //   pin_xclk: 0,
  //   __bindgen_anon_1: esp_idf_sys::camera_config_t__bindgen_ty_1 { pin_sscb_sda: 26 },
  //   __bindgen_anon_2: esp_idf_sys::camera_config_t__bindgen_ty_2 { pin_sscb_scl: 27 },
  //   pin_d7: 35,
  //   pin_d6: 34,
  //   pin_d5: 39,
  //   pin_d4: 36,
  //   pin_d3: 21,
  //   pin_d2: 19,
  //   pin_d1: 18,
  //   pin_d0: 5,
  //   pin_vsync: 25,
  //   pin_href: 23,
  //   pin_pclk: 22,

  //   //XCLK 20MHz or 10MHz for OV2640 double FPS (Experimental)
  //   xclk_freq_hz: 20000000,
  //   ledc_timer: esp_idf_sys::ledc_timer_t_LEDC_TIMER_0,
  //   ledc_channel: esp_idf_sys::ledc_channel_t_LEDC_CHANNEL_0,

  //   pixel_format: esp_idf_sys::pixformat_t_PIXFORMAT_JPEG, //YUV422,GRAYSCALE,RGB565,JPEG
  //   frame_size: esp_idf_sys::framesize_t_FRAMESIZE_QVGA, //QQVGA-UXGA Do not use sizes above QVGA when not JPEG

  //   jpeg_quality: 12, //0-63 lower number means higher quality
  //   fb_count: 1,      //if more than one, i2s runs in continuous mode. Use only with JPEG
  //   fb_location: esp_idf_sys::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
  //   grab_mode: esp_idf_sys::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,

  //   sccb_i2c_port: 0,
  // };

  // if unsafe { esp_idf_sys::esp_camera_init(&camera_config) } != 0 {
  //   println!("camera init failed!");
  //   return;
  // } else {
  //   println!("camera ready! >>>>>>>>>>>>>");
  // }

  // /* wifi */
  // loop {
  //   let fb = unsafe { esp_idf_sys::esp_camera_fb_get() };
  //   let mut v = Vec::<u8>::with_capacity(unsafe { (*fb).len });
  //   unsafe {
  //     copy_nonoverlapping((*fb).buf, v.as_mut_ptr(), (*fb).len);
  //     v.set_len((*fb).len);
  //   }
  //   println!("Picture taken! Its size was: {} bytes", unsafe {
  //     (*fb).len
  //   });
  // }
  Ok(())
}
