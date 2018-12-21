extern crate rppal;

use std::io;
use std::{thread, time};
use std::time::{Duration};
use rppal::{gpio};
use rppal::gpio::{Gpio, Level};
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::sync::mpsc;

//
//    27
// 22    26
//    21
// 23    25
//    24
//
const CTRLS : [u8; 4] = [16, 17, 18, 19];
const LEDS  : [u8; 8] = [20, 21, 22, 23, 24, 25, 26, 27];
const SEG1  : [u8; 2] = [    25, 26];
const SEG2  : [u8; 5] = [    21,     23, 24,     26, 27];
const SEG3  : [u8; 5] = [    21,         24, 25, 26, 27];
const SEG4  : [u8; 4] = [    21, 22,         25, 26];
const SEG5  : [u8; 5] = [    21, 22,     24, 25,     27];
const SEG6  : [u8; 6] = [    21, 22, 23, 24, 25,     27];
const SEG7  : [u8; 3] = [                    25, 26, 27];
const SEG8  : [u8; 7] = [    21, 22, 23, 24, 25, 26, 27];
const SEG9  : [u8; 6] = [    21, 22,     24, 25, 26, 27];
const SEG0  : [u8; 6] = [        22, 23, 24, 25, 26, 27];
const SEG_  : [u8; 0] = [];
const SEGA  : [u8; 7] = [20, 21, 22, 23,     25, 26, 27];
const SEGB  : [u8; 8] = [20, 21, 22, 23, 24, 25, 26, 27];
const SEGC  : [u8; 6] = [20, 21, 22, 23, 24,         27];
const SEGD  : [u8; 7] = [20,     22, 23, 24, 25, 26, 27];
const SEGE  : [u8; 6] = [20, 21, 22, 23, 24,         27];
const SEGF  : [u8; 5] = [20, 21, 22, 23,             27];
const DELAY : time::Duration = time::Duration::from_millis(1);

fn led(gpio : &Gpio, nums : &[u8]) {
  for n in &LEDS {
    gpio.write(*n, Level::High);
  }
  for n in nums {
    gpio.write(*n, Level::Low);
  }
}

fn unsel_all(gpio : &Gpio) {
  for n in &CTRLS {
    gpio.write(*n, Level::Low);
  }
}

fn sel(gpio : &Gpio, n : u8) {
  unsel_all(gpio);
  gpio.write(n, Level::High);
}

fn seln(gpio : &Gpio, n : u8) {
  match n {
    0    => { sel(gpio, n + 16) }
    1    => { sel(gpio, n + 16) }
    2    => { sel(gpio, n + 16) }
    3    => { sel(gpio, n + 16) }
    _    => { println!("error")}
  }
}

fn ledn(gpio : &Gpio, n : i32) {
  match n {
    0 => { led(gpio, &SEG0) }
    1 => { led(gpio, &SEG1) }
    2 => { led(gpio, &SEG2) }
    3 => { led(gpio, &SEG3) }
    4 => { led(gpio, &SEG4) }
    5 => { led(gpio, &SEG5) }
    6 => { led(gpio, &SEG6) }
    7 => { led(gpio, &SEG7) }
    8 => { led(gpio, &SEG8) }
    9 => { led(gpio, &SEG9) }
    _ => { led(gpio, &SEG_) }
  }
}

fn ctrl_7segs(gpio : &Gpio, n : i32) {
  seln(&gpio, 0);
  ledn(&gpio, n % 10);
  std::thread::sleep(DELAY);

  let n = n / 10;
  seln(&gpio, 1);
  ledn(&gpio, n % 10);
  std::thread::sleep(DELAY);

  let n = n / 10;
  seln(&gpio, 2);
  ledn(&gpio, n % 10);
  std::thread::sleep(DELAY);

  let n = n / 10;
  seln(&gpio, 3);
  ledn(&gpio, n % 10);
  std::thread::sleep(DELAY);
}

fn handle_client(stream: TcpStream) -> String {
  let mut stream = io::BufReader::new(stream);

  let mut buf = [0; 10];
  let dsize;
  match stream.read(&mut buf) {
    Ok(size) => { 
      // println!("read size : {}", size);
      dsize = size;
    },
    _ => panic!("read error!!!"),
  }

  // let msg = str::from_utf8(&buf[0..dsize-1]).unwrap().to_string();
  let msg = String::from_utf8(buf[0..dsize-1].to_vec()).unwrap();

  return msg;
}

fn socket(tx: mpsc::Sender<String>) {
  
  let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let msg = handle_client(stream);
        // println!(" data : {}", msg);
        tx.send(msg).unwrap();
      }
      Err(_) => {
        panic!()
      }
    };
  }
}

fn check_msg (rx: &mpsc::Receiver<String>) -> String {
  match rx.recv_timeout(Duration::from_millis(1)) {
    Err(mpsc::RecvTimeoutError::Timeout) => {
      return "Err".to_string();
    }
    Err(mpsc::RecvTimeoutError::Disconnected) => {
      return "Err".to_string();
    }
    Ok(msg) => {
      return msg;
    }
  }
}

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    socket(tx);
  });

  let mut gpio = Gpio::new().expect( "Failed Gpio::new" );

  for n in &LEDS {
    gpio.set_mode(*n, gpio::Mode::Output);
  }
  for n in &CTRLS {
    gpio.set_mode(*n, gpio::Mode::Output);
    gpio.write(*n, Level::Low);
  }

  unsel_all(&gpio);

  let mut n = 1;
  loop {
    let msg = check_msg(&rx); 
    if msg != "Err" {
      // println!("Got : {}", msg);
      println!("Got : {}", msg.bytes[0]);
      match msg.parse::<i32>() {
        Ok(val) => {
          n = val;
          println!("  valid request : {}", n);
        },
        Err(_) => {
          println!("invalid request : {}", msg);
        },
      }
    }
    if msg == "exit" {
      break;
    }

    ctrl_7segs(&gpio, n);
  }
 
  for n in &LEDS {
    gpio.set_mode(*n, gpio::Mode::Input);
  }
  for n in &CTRLS {
    gpio.set_mode(*n, gpio::Mode::Input);
  }
}  
