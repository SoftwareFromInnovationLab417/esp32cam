use anyhow::Result;
use embedded_svc::{
  ipv4,
  wifi::{AccessPointConfiguration, ClientConfiguration, Configuration, Wifi},
};
use esp_idf_hal::peripheral;
use esp_idf_svc::{
  eventloop::EspSystemEventLoop,
  netif::{EspNetif, EspNetifWait},
  nvs::EspDefaultNvsPartition,
  ping::EspPing,
  wifi::{EspWifi, WifiWait},
};
use std::{net::Ipv4Addr, time::Duration};

use crate::{PASS, SSID};

pub fn wifi(
  modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
  sysloop: EspSystemEventLoop,
  default_nvs: Option<EspDefaultNvsPartition>,
) -> Result<Box<EspWifi<'static>>> {
  println!("hello -----------------------------> wifi?");
  let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), default_nvs)?);

  println!("Wifi created, about to scan...");

  let ap_infos = wifi.scan()?;

  let list = ap_infos.iter().map(|a| a.ssid.clone()).collect::<Vec<_>>();
  for a in list {
    println!("{}", a);
  }

  let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);


  let channel = if let Some(ours) = ours {
    println!(
      "Found configured access point {} on channel {}",
      SSID, ours.channel
    );
    Some(ours.channel)
  } else {
    println!("Configured access point {} not found during scanning, will go with unknown channel", SSID);
    None
  };

  wifi.set_configuration(&Configuration::Mixed(
    ClientConfiguration {
      ssid: SSID.into(),
      password: PASS.into(),
      channel,
      ..Default::default()
    },
    AccessPointConfiguration {
      ssid: "aptest".into(),
      channel: channel.unwrap_or(1),
      ..Default::default()
    },
  ))?;

  wifi.start()?;

  println!("Starting wifi...");

  if !WifiWait::new(&sysloop)?
    .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
  {
    anyhow::bail!("Wifi did not start");
  }

  println!("Connecting wifi...");

  wifi.connect()?;

  if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?
    .wait_with_timeout(Duration::from_secs(20), || {
      wifi.is_connected().unwrap()
        && wifi.sta_netif().get_ip_info().unwrap().ip
          != Ipv4Addr::new(0, 0, 0, 0)
    })
  {
    anyhow::bail!("Wifi did not connect or did not receive a DHCP lease");
  }

  let ip_info = wifi.sta_netif().get_ip_info()?;

  ping(ip_info.subnet.gateway)?;

  Ok(wifi)
}

fn ping(ip: ipv4::Ipv4Addr) -> Result<()> {
  println!("About to do some pings for {:?}", ip);

  let ping_summary = EspPing::default().ping(ip, &Default::default())?;
  if ping_summary.transmitted != ping_summary.received {
    anyhow::bail!("Pinging gateway {} resulted in timeouts", ip);
  }

  println!("Pinging done");

  Ok(())
}
