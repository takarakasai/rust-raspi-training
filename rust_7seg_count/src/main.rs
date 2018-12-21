extern crate rppal;

use std::thread;
use std::time::{Duration, SystemTime};
use rppal::{gpio};
use rppal::gpio::{Gpio, Level};

const GPIO_PIN_NUM : u8 = 13;
const BLINK_NUM : u8 = 24;
const CTRLS : [u8; 4] = [16, 17, 18, 19];
const LEDS  : [u8; 7] = [21, 22, 23, 24, 25, 26, 27];
const SEG1  : [u8; 2] = [25, 26];
const SEG2  : [u8; 5] = [21,     23, 24,     26, 27];
const SEG3  : [u8; 5] = [21,         24, 25, 26, 27];
const SEG4  : [u8; 4] = [21, 22,         25, 26];
const SEG5  : [u8; 5] = [21, 22,     24, 25,     27];
const SEG6  : [u8; 6] = [21, 22, 23, 24, 25,     27];
const SEG7  : [u8; 3] = [                25, 26, 27];
const SEG8  : [u8; 7] = [21, 22, 23, 24, 25, 26, 27];
const SEG9  : [u8; 6] = [21, 22,     24, 25, 26, 27];
const SEG0  : [u8; 6] = [    22, 23, 24, 25, 26, 27];
const SEG_  : [u8; 0] = [];

fn led(gpio : &Gpio, nums : &[u8]) {
  for n in &LEDS {
    gpio.write(*n, Level::High);
  }
  for n in nums {
    gpio.write(*n, Level::Low);
  }
}

fn sel(gpio : &Gpio, n : u8) {
  for n in &CTRLS {
    gpio.write(*n, Level::Low);
  }
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

fn main() {
  let mut gpio = Gpio::new().expect( "Failed Gpio::new" );
  let mut blinking_cnt = 0;

  for n in &LEDS {
    gpio.set_mode(*n, gpio::Mode::Output);
  }
  for n in &CTRLS {
    gpio.set_mode(*n, gpio::Mode::Output);
    gpio.write(*n, Level::Low);
  }

  gpio.write(16, Level::Low);
  gpio.write(17, Level::Low);
  gpio.write(18, Level::Low);
  gpio.write(19, Level::Low);
  let now = SystemTime::now();
  loop {
      match now.elapsed() {
        Ok(n) => {
          // println!("{}.{}", n.as_secs(), n.subsec_millis()),
          let mins = (n.as_secs() / 60) as i32; 
          let secs = (n.as_secs() % 60) as i32; 
          seln(&gpio, 0);
          ledn(&gpio, secs % 10);
          std::thread::sleep_ms(1);
          seln(&gpio, 1);
          ledn(&gpio, secs / 10);
          std::thread::sleep_ms(1);
          seln(&gpio, 2);
          ledn(&gpio, mins % 10);
          std::thread::sleep_ms(1);
          seln(&gpio, 2);
          seln(&gpio, 3);
          ledn(&gpio, mins / 10);
          std::thread::sleep_ms(1);
        },
        Err(_) => panic!("Error"),
      }
  }

  for n in &LEDS {
    gpio.set_mode(*n, gpio::Mode::Input);
  }
  for n in &CTRLS {
    gpio.set_mode(*n, gpio::Mode::Input);
  }
}  
