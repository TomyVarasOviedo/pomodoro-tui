use std::time::Instant;
use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
}
impl Phase {
    pub fn label(&self) -> &'static str {
        match self {
            Phase::Focus => "Focus",
            Phase::ShortBreak => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Idle,
    Running,
    Paused,
}

pub struct Timer {
    pub phase:Phase,
    pub status: Status,

    pub pomodoros_done: u32,

    elapsed_secs:u64,
    last_resumen:Option<Instant>,
    phase_duration_secs:u64,
    config:Config,

}
impl Timer {
    pub fn new(config: Config) -> Self{
        let phase_duration_secs = config.focus_minutes*60;
        Self {
            phase: Phase::Focus,
            status: Status::Idle,
            pomodoros_done: 0,
            elapsed_secs:0,
            last_resumen: None,
            phase_duration_secs,
            config,
        }
    }

    pub fn toggle(&mut self) {
        match self.status {
            Status::Idle | Status::Paused => {
                self.status = Status::Running;
                self.last_resumen = Some(Instant::now());
            }
            Status::Running => {
                self.flush_elapshed();
                self.status = Status::Paused;
                self.last_resumen = None;
            }
        }
    }
    pub fn reset(&mut self) {
        self.elapsed_secs = 0;
        self.last_resumen = None;
        self.status = Status::Idle;
    }

    pub fn skip(&mut self) -> bool{
        self.advance_phase();
        true
    }
    pub fn tick(&mut self) -> bool{
        if self.status != Status::Running {
            return false;
        }
        let total = self.total_elapsed_secs();

        if total >= self.phase_duration_secs {
            self.flush_elapshed();
            self.advance_phase();
            return true;
        }
        false
    }
    pub fn remaing_secs(&self) -> u64{
        self.phase_duration_secs
            .saturating_sub(self.total_elapsed_secs())
    }
    pub fn progress(&self) -> f64{
        if self.phase_duration_secs == 0 {
            return 0.0;
        }
        (self.total_elapsed_secs() as f64 / self.phase_duration_secs as f64).min(1.0)
    }
    pub fn remaining_mmss(&self) -> (u64, u64){
        let r = self.remaing_secs();
        (r/60, r%60)
    }

    pub fn phase_durations_mins(&self) -> u64{
        self.phase_duration_secs /60
    }

    // -------FUNCIONES DE UTILIDADES------------
    fn total_elapsed_secs(&self) -> u64{
        let live = self
            .last_resumen
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0);
        self.elapsed_secs + live
    }
    fn advance_phase(&mut self) {
        if self.phase == Phase::Focus {
            self.pomodoros_done += 1;
        }
        self.phase = self.next_phase();
        self.phase_duration_secs = self.duration_for(&self.phase);
        self.elapsed_secs = 0;
        self.last_resumen = None;

        self.status = Status::Running;
        self.last_resumen = Some(Instant::now());
    }
    fn duration_for(&self, phase:&Phase) -> u64{
        match phase {
            Phase::Focus => self.config.focus_minutes*60,
            Phase::ShortBreak => self.config.short_break_minutes*60,
            Phase::LongBreak => self.config.long_break_minutes*60,
        }
    }

    fn next_phase(&self) -> Phase{
        match self.phase {
            Phase::Focus => {
                if self.pomodoros_done % self.config.long_break_interval == 0 {
                    Phase::LongBreak
                }else {
                    Phase::ShortBreak
                }
            }
            Phase::ShortBreak | Phase::LongBreak => Phase::Focus,
        }
    }

    fn flush_elapshed(&mut self) {
        if let Some(t) = self.last_resumen.take() {
            self.elapsed_secs += t.elapsed().as_secs();
        }
    }
}