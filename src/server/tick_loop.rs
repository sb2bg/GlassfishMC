use std::thread;
use std::time::Duration;

use log::{debug, trace};

use crate::server::exit_with_err::exit_with_err;
use crate::server::Server;

pub struct TickLoop<'a> {
    server: &'a mut Server,
    tick_rate: u32,
    auto_save_interval: u32,
    // todo: runnables, worlds, entities, redstone
}

impl<'a> TickLoop<'a> {
    pub fn new(server: &'a mut Server) -> Self {
        let tick_rate = server.get_config().get_tick_rate();
        let auto_save_interval = server.get_config().get_auto_save_interval();

        if tick_rate == 0 || tick_rate > 1000 {
            exit_with_err("Error starting server: Tick rate must be between 1 and 1000");
        }

        Self {
            server,
            tick_rate,
            auto_save_interval,
        }
    }

    pub fn run(&mut self) {
        let mut rolling_avg = RollingTickAverage::new(self.tick_rate);

        loop {
            rolling_avg.tick();

            let avg_tps = rolling_avg.get_average();
            self.server.set_tps(avg_tps);

            // todo: add catchup
            thread::sleep(Duration::from_millis((1000 / self.tick_rate) as u64));
        }
    }

    fn auto_save(&mut self) {}
}

struct RollingTickAverage {
    ticks: u32,
    time: Duration,
    target_tps: u32,
}

impl RollingTickAverage {
    fn new(target_tps: u32) -> Self {
        Self { ticks: 0, time: Duration::from_millis(0), target_tps }
    }

    fn tick(&mut self) {
        self.ticks += 1;
        self.time += Duration::from_millis(1000 / self.target_tps as u64);
    }

    fn get_average(&self) -> f32 {
        self.ticks as f32 / self.time.as_secs_f32()
    }
}