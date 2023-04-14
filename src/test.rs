use anyhow::Result;

use std::thread;
use std::{fs, path::PathBuf};

use base64::{engine::general_purpose, Engine as _};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ptr::copy_nonoverlapping;

#[allow(unused)]
pub fn test_fs() -> Result<()> {
  assert_eq!(fs::canonicalize(PathBuf::from("."))?, PathBuf::from("/"));
  assert_eq!(
    fs::canonicalize(
      PathBuf::from("/")
        .join("foo")
        .join("bar")
        .join(".")
        .join("..")
        .join("baz")
    )?,
    PathBuf::from("/foo/baz")
  );

  Ok(())
}

#[allow(unused)]
pub fn test_tcp(host: &str, port: &str, uri: &str) {
  println!("About to open a TCP connection to {} : {}", host, port);

  let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

  let err = stream.try_clone();
  if let Err(err) = err {
    println!(
            "Duplication of file descriptors does not work (yet) on the ESP-IDF, as expected: {}",
            err
        );
  }

  loop {
    let fb = unsafe { esp_idf_sys::esp_camera_fb_get() };
    let mut v = Vec::<u8>::with_capacity(unsafe { (*fb).len });
    unsafe {
      copy_nonoverlapping((*fb).buf, v.as_mut_ptr(), (*fb).len);
      v.set_len((*fb).len);
    }
    println!("Picture taken! Its size was: {} bytes", unsafe {
      (*fb).len
    });
    //base64::encode_config_buf(v, base64::URL_SAFE, &mut buf);
    let data = b"hello internet";
    let buf: String = general_purpose::URL_SAFE.encode(data);
    // base64::encode_config_buf(, base64::URL_SAFE, &mut buf);
    println!("{}", buf);
    stream.write_all(format!("POST {} HTTP/1.1\r\nHost: 127.0.0.1:9515\r\nContent-Type: application/json\r\nContent-Length: 17\r\n\r\n{{\"image\":\"hello\"}}",uri).as_bytes()).unwrap();

    let mut result = Vec::new();

    stream.read_to_end(&mut result).unwrap();

    println!(
            "returned:\n=================\n{}\n=================\nSince it returned something, all is OK",
            std::str::from_utf8(&result).unwrap());
  }
}

#[allow(unused)]
pub fn test_tcp_bind() -> Result<()> {
  fn test_tcp_bind_accept() -> Result<()> {
    println!("About to bind a simple echo service to port 8080");

    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          println!("Accepted client");

          thread::spawn(move || {
            test_tcp_bind_handle_client(stream);
          });
        }
        Err(e) => {
          eprintln!("Error: {}", e);
        }
      }
    }

    unreachable!()
  }

  fn test_tcp_bind_handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
      let mut read = [0; 128];

      match stream.read(&mut read) {
        Ok(n) => {
          if n == 0 {
            // connection was closed
            break;
          }
          stream.write_all(&read[0..n]).unwrap();
        }
        Err(err) => {
          panic!("{}", err);
        }
      }
    }
  }

  thread::spawn(|| test_tcp_bind_accept().unwrap());

  Ok(())
}
