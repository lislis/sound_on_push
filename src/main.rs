use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::io::BufReader;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rodio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    //pin.set_low();

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let mixer = stream_handle.mixer();

    let sound = {
      let file = std::fs::File::open("/home/raspi/test-loop.wav")?;	
      let buf = BufReader::new(file);
      println!("{:?}", buf.total_duration);
      let sink = rodio::play(&mixer, buf)?;
      sink.set_volume(0.2);
      sink
    };
 
    println!("SOUND playing");
//    println!("{:?}", sound.total_duration.expect("nope buffer"));	

    Ok(())
}
