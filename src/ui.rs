use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, Gauge, Padding, Paragraph}};

use crate::timer::{Phase, Status, Timer};

const COLOR_FOCUS:       Color = Color::Rgb(224,  95,  95);  // tomato-red
const COLOR_SHORT:       Color = Color::Rgb( 95, 180, 130);  // mint
const COLOR_LONG:        Color = Color::Rgb( 95, 145, 224);  // sky-blue
const COLOR_PAUSED:      Color = Color::Rgb(160, 160, 160);  // grey
const COLOR_BG:          Color = Color::Reset;
const COLOR_LABEL:       Color = Color::DarkGray;

const DIGITS: [&[&str]; 10] = [
    &["▛▀▜", "▌ ▐", "▌ ▐", "▌ ▐", "▙▄▟"],  // 0
    &["  ▐", "  ▐", "  ▐", "  ▐", "  ▐"],  // 1
    &["▛▀▜", "  ▐", "▛▀▟", "▌  ", "▙▄▟"],  // 2
    &["▛▀▜", "  ▐", " ▀▟", "  ▐", "▙▄▟"],  // 3
    &["▌ ▐", "▌ ▐", "▙▄▟", "  ▐", "  ▐"],  // 4
    &["▛▀▜", "▌  ", "▙▀▜", "  ▐", "▙▄▟"],  // 5
    &["▛▀▜", "▌  ", "▙▀▜", "▌ ▐", "▙▄▟"],  // 6
    &["▛▀▜", "  ▐", "  ▐", "  ▐", "  ▐"],  // 7
    &["▛▀▜", "▌ ▐", "▙▀▟", "▌ ▐", "▙▄▟"],  // 8
    &["▛▀▜", "▌ ▐", "▙▄▟", "  ▐", "▙▄▟"],  // 9
];

const COLON: [&str; 5] = [" ", "•", " ", "•", " "];

fn big_time(mm:u64, ss:u64) -> Vec<String>{
    let digits = [mm/10, mm % 10, 99, ss/10, ss % 10];
    let mut rows = vec![String::new(); 5];
    for (i, &d) in digits.iter().enumerate() {
        let spacing = if i == 0 {""} else {" "};
        for row in 0..5 {
            let ch = if d == 99 {
                COLON[row]
            }else {
                DIGITS[d as usize][row]
            };
            rows[row].push_str(spacing);
            rows[row].push_str(ch);
        }
    }
    rows
}
fn phase_color(timer: &Timer) -> Color{
    if timer.status == Status::Paused {
        return COLOR_PAUSED;
    }

    match timer.phase {
        Phase::Focus => COLOR_FOCUS,
        Phase::ShortBreak => COLOR_SHORT,
        Phase::LongBreak => COLOR_LONG,
    }
}
fn pomodoro_dots(count:u32) -> String{
    let filled = "|-".repeat(count as usize);
    filled
}
pub fn render(f: &mut Frame, timer:&Timer) {
    let area = f.area();
    let color = phase_color(timer);

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Line::from(vec![
            Span::styled(" ", Style::default().fg(color).add_modifier(Modifier::BOLD)),
        ]))
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    let inner_area = outer.inner(area);
    f.render_widget(outer, area);

    let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(7),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(inner_area);
    render_phase_header(f, timer, chunks[0], color);
    render_big_timer(f, timer, chunks[2 - 1], color);
    render_progress_bar(f, timer, chunks[2], color);
    render_dots(f, timer, chunks[4]);
    render_help(f, timer, chunks[6]);
    
}

//-----RENDER FUNCTIONS---------
fn render_phase_header(f: &mut Frame, timer: &Timer, area: Rect, color: Color) {
    let status_str = match timer.status {
        Status::Running => "▶",
        Status::Paused  => "⏸",
        Status::Idle    => "●",
    };

    let lines = vec![
        Line::from(Span::styled(
            timer.phase.label(), 
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        )),
        Line::from(
            Span::styled(
                status_str, 
                Style::default().fg(COLOR_LABEL)
        )),
    ];
    f.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center), area,
    );
}
fn render_big_timer(f: &mut Frame, timer: &Timer, area: Rect, color: Color) {
    let (mm, ss) = timer.remaining_mmss();
    let rows = big_time(mm, ss);
 
    let lines: Vec<Line> = rows
        .iter()
        .map(|r| {
            Line::from(Span::styled(
                r.clone(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
        })
        .collect();
 
    f.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center),
        area,
    );
}
 
fn render_progress_bar(f: &mut Frame, timer: &Timer, area: Rect, color: Color) {
    let pct = (timer.progress() * 100.0) as u16;
    let (mm, ss) = timer.remaining_mmss();
 
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(COLOR_BG))
        .percent(pct)
        .label(format!("󱎫 {:02}:{:02} remaining", mm, ss));
 
    f.render_widget(gauge, area);
}
 
fn render_dots(f: &mut Frame, timer: &Timer, area: Rect) {
    let dots = if timer.pomodoros_done == 0 {
        "No pomodoros completed yet".to_string()
    } else {
        pomodoro_dots(timer.pomodoros_done)
    };
 
    f.render_widget(
        Paragraph::new(dots).alignment(Alignment::Center),
        area,
    );
}
 
fn render_help(f: &mut Frame, _timer: &Timer, area: Rect) {
    let keys = vec![
        ("[Space]", "/󰏤"),
        ("[S]", "󰒭"),
        ("[R]", ""),
        ("[Q]", ""),
    ];
 
    let mut spans: Vec<Span> = vec![];
    for (i, (key, label)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default()));
        }
        spans.push(Span::styled(
            *key,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {}", label),
            Style::default().fg(COLOR_LABEL),
        ));
    }
 
    f.render_widget(
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
        area,
    );
}