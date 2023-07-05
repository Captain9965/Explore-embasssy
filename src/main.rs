
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

/**
 * @author: Lenny Weda
 * circa 2023
 * Embassy demo script demonstrating:
 * 1. async multitasking -> button task + led task
 * 2. Input via a button
 * 3. Debugging via an st-link debugger.
 * 4. Output via an led
 */

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, AnyPin, Pull, Input, Pin};
use embassy_time::{Duration, Timer, Instant};
use {defmt_rtt as _, panic_probe as _};

// defining constants: 
const TIMER_INTERVAL : u64 = 1000;
const TASK_DELAY_TIME : u64 = 10;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // intializing peripherals
    let p = embassy_stm32::init(Default::default());
    info!("======================= Initializing resources =======================");

    //spawn task
    _spawner.spawn(read_input(p.PB7.degrade())).unwrap();

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    // initializing timer instance
    let blink_timer = Instant::now();
    let mut state : bool = false;
    let mut time = TIMER_INTERVAL + blink_timer.as_millis();

    // blink loop: 
    loop {

        if blink_timer.elapsed().as_millis() >= time{
            if state{
                info!("high");
                led.set_high();
                state = !state;
            } else{
                info!("low");
                led.set_low();
                state = !state;
            }

            time += TIMER_INTERVAL;
        }
        
        Timer::after(Duration::from_millis(TASK_DELAY_TIME)).await;
    }
}

#[embassy_executor::task]
async fn read_input(pin : AnyPin){

    //button instance is input pullup: 
    let button = Input::new(pin, Pull::Up);
    loop{
        if button.is_low(){
            info!("Button has been pressed!");
        }
        Timer::after(Duration::from_millis(TASK_DELAY_TIME)).await;
    }
}
