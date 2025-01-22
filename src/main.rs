#![allow(unreachable_code)]

use rppal::gpio::Gpio;
use rppal::gpio::Level;

use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
	sound::static_sound::StaticSoundData,
};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;
const GPIO_BUTTON: u8 = 12;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    let mut pin_led = Gpio::new()?.get(GPIO_LED)?.into_output();
    let pin_button = Gpio::new()?.get(GPIO_BUTTON)?.into_input_pullup();

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let sound_data = StaticSoundData::from_file(&filepath)?.volume(-4.0);
    let duration = sound_data.duration();

    // default is High because input_pullup
    let mut current_button_state = Level::High;

    pin_led.set_low();
    thread::sleep(Duration::from_millis(500));
    pin_led.set_high();
    thread::sleep(Duration::from_millis(500));
    pin_led.set_low();
    thread::sleep(Duration::from_millis(500));
    pin_led.set_high();

    println!("ready to go");


    loop {
      current_button_state = pin_button.read();

      if current_button_state == Level::Low {
         println!("playing");

 	  pin_led.set_low();
          manager.play(sound_data.clone())?;
	  thread::sleep(duration);
          pin_led.set_high();

         println!("done playing");
      }
    }

    Ok(())
}
