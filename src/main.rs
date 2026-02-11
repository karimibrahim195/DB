use notify_rust::Notification;
use chrono::Local;
use std::{thread, time};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut minutes = 60;
    
    if args.len() > 1 {
        minutes = args[1].parse().unwrap_or(60);
    }
    
    let interval = time::Duration::from_secs(minutes * 60);
    
    println!("=================================");
    println!("💧 Water Reminder v1.0");
    println!("=================================");
    println!("Reminding every {} minutes", minutes);
    println!("Press Ctrl+C to stop");
    println!("");

    loop {
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();
        
        println!("[{}] ⏰ Drink water! ({} min) 💧", time_str, minutes);
        
        // Desktop notification
        let _ = Notification::new()
            .summary("💧 Water Reminder")
            .body(&format!("Drink water! ({} minutes)", minutes))
            .timeout(5000)
            .show();
        
        // Terminal bell
        print!("\x07");
        thread::sleep(time::Duration::from_millis(200));
        print!("\x07");
        
        thread::sleep(interval);
    }
}
