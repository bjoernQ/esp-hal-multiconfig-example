#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use log::{debug, info};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 32 * 1024);

    // We can have conditional compilation based on the target/config features
    #[cfg(feature = "esp32-psram")]
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    // Chips can differ in the supported feature set. Use conditional compilation if needed.
    let timer = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timer.timer0);

    info!("Embassy initialized! Running on {}", esp_hal::chip!());

    loop {
        info!("Hello world!");
        debug!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
