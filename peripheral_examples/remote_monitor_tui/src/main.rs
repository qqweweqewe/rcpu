use anyhow::Result;
use crossterm::{
    cursor,
    queue,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand,
};
use reqwest;
use serde_json::Value;
use std::{
    io::{stdout, Write},
    time::Duration,
};
use tokio::time;

async fn fetch_cpu_usage() -> Result<u8> {
    let resp = reqwest::get("http://localhost:3000/cpu").await?;
    let json: Value = resp.json().await?;
    Ok(json["cpu"].as_u64().unwrap() as u8)
}

async fn fetch_ram_usage() -> Result<u8> {
    let resp = reqwest::get("http://localhost:3000/ram").await?;
    let json: Value = resp.json().await?;
    Ok(json["ram"].as_u64().unwrap() as u8)
}

async fn fetch_disk_usage() -> Result<u8> {
    let resp = reqwest::get("http://localhost:3000/disk/percentage").await?;
    let json: Value = resp.json().await?;
    Ok(json["percentage"].as_u64().unwrap() as u8)
}

fn draw_bar(label: &str, percentage: u8, width: usize) -> String {
    let filled = (percentage as usize * width) / 100;
    let empty = width - filled;
    format!(
        "{}: [{}{}] {}%",
        label,
        "#".repeat(filled),
        " ".repeat(empty),
        percentage
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
        queue!(stdout, cursor::MoveTo(0, 0))?;

        let cpu = fetch_cpu_usage().await.unwrap_or(0);
        let ram = fetch_ram_usage().await.unwrap_or(0);
        let disk = fetch_disk_usage().await.unwrap_or(0);

        let bar_width = 20;
        let cpu_bar = draw_bar("CPU", cpu, bar_width);
        let ram_bar = draw_bar("RAM", ram, bar_width);
        let disk_bar = draw_bar("DISK", disk, bar_width);

        queue!(
            stdout,
            style::PrintStyledContent(cpu_bar.green()),
            cursor::MoveToNextLine(1),
            style::PrintStyledContent(ram_bar.blue()),
            cursor::MoveToNextLine(1),
            style::PrintStyledContent(disk_bar.yellow()),
            cursor::MoveToNextLine(1),
            cursor::MoveToNextLine(1),
            style::Print("Press 'q' to exit"),
        )?;

        stdout.flush()?;

        // Check for exit key
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        interval.tick().await;
    }

    // Cleanup
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok(())
}