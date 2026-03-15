
use notify_rust::{Notification, Urgency};

use crate::config::Config;
use crate::timer::{Phase, Timer};

pub fn send_notification(timer:&Timer, config:&Config) {
    if !config.notifications.enabled {
        return;
    }

    if config.notifications.bell {
        print!("\x07");
    }

    let (title, body) = build_message(timer);
    let urgency = parse_urgency(&config.notifications.urgency);

    let result = Notification::new()
        .summary(&title)
        .body(&body)
        .icon(&config.notifications.icon)
        .urgency(urgency)
        .timeout(notify_rust::Timeout::Milliseconds(8000))
        .show();

    if let Err(e) = result {
        eprintln!("⚠ Notification failed: {e}")
    }


}
fn build_message(timer: &Timer) -> (String, String) {
    match timer.phase {
        Phase::Focus => (
            " FOCUS TIME!".into(),
            format!(
                "Time to concentrate for {} minutes.\nPomodoro #{} incoming.",
                timer.phase_durations_mins(),
                timer.pomodoros_done + 1
            ),
        ),
        Phase::ShortBreak => (
            " SHORT BREAK".into(),
            format!(
                "Nice work! Take {} minutes to rest.\nPomodoros completed: {}",
                timer.phase_durations_mins(),
                timer.pomodoros_done
            ),
        ),
        Phase::LongBreak => (
            "🛋 LONG BREAK".into(),
            format!(
                "You finished {} pomodoros. Enjoy a {} min break.",
                timer.pomodoros_done,
                timer.phase_durations_mins()
            ),
        ),
    }
}

fn parse_urgency(s: &str) -> Urgency{
    match s.to_lowercase().as_str() {
        "low" => Urgency::Low,
        "critical" => Urgency::Critical,
        _ => Urgency::Normal,
    }
}