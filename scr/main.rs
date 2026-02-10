use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("========================================");
    println!("   WhatsApp Bulk Sender v1.0");
    println!("========================================");
    println!();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_help();
        return;
    }

    let numbers_file = &args[1];
    let message = &args[2];

    println!("📱 Numbers file: {}", numbers_file);
    println!("💬 Message: {}", message);
    println!();

    match generate_links(numbers_file, message) {
        Ok(output_file) => {
            println!("✓ Links generated successfully!");
            println!("📄 Output: {}", output_file);
            println!();
            println!("Open the HTML file in your browser and click each link!");
        }
        Err(e) => {
            println!("✗ Error: {}", e);
        }
    }
}

fn print_help() {
    println!("WhatsApp Bulk Sender - Generate WhatsApp links for bulk messaging");
    println!();
    println!("Usage:");
    println!("  whatsapp-sender <numbers_file> <message>");
    println!();
    println!("Examples:");
    println!("  whatsapp-sender numbers.txt \"Hello! This is a test message\"");
    println!("  whatsapp-sender contacts.csv \"Special offer just for you!\"");
    println!();
    println!("Numbers file format:");
    println!("  - One number per line");
    println!("  - International format (e.g., 201234567890)");
    println!("  - No spaces or special characters");
    println!();
    println!("The tool will generate an HTML file with clickable WhatsApp links.");
}

fn generate_links(numbers_file: &str, message: &str) -> io::Result<String> {
    if !Path::new(numbers_file).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Numbers file not found"
        ));
    }

    let content = fs::read_to_string(numbers_file)?;
    let numbers: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    if numbers.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No valid numbers found in file"
        ));
    }

    println!("📊 Found {} numbers", numbers.len());
    println!("🔗 Generating WhatsApp links...");

    let output_file = "whatsapp_links.html";
    let html = generate_html(&numbers, message);

    fs::write(output_file, html)?;

    Ok(output_file.to_string())
}

fn generate_html(numbers: &[&str], message: &str) -> String {
    let encoded_message = url_encode(message);
    
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>WhatsApp Bulk Sender</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: Arial, sans-serif;
            background: linear-gradient(135deg, #25D366 0%, #128C7E 100%);
            padding: 20px;
            min-height: 100vh;
        }
        
        .container {
            max-width: 900px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            padding: 40px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        
        h1 {
            color: #25D366;
            text-align: center;
            font-size: 2.5em;
            margin-bottom: 10px;
        }
        
        .subtitle {
            text-align: center;
            color: #666;
            margin-bottom: 30px;
            font-size: 1.1em;
        }
        
        .stats {
            background: #f0f9ff;
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 30px;
            border-left: 4px solid #25D366;
        }
        
        .stats p {
            margin: 5px 0;
            color: #333;
        }
        
        .message-preview {
            background: #fff9e6;
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 30px;
            border-left: 4px solid #ffc107;
        }
        
        .message-preview h3 {
            color: #f57c00;
            margin-bottom: 10px;
        }
        
        .message-text {
            color: #555;
            font-style: italic;
            line-height: 1.6;
        }
        
        .links-container {
            margin-top: 30px;
        }
        
        .link-card {
            background: #f8f9fa;
            padding: 15px 20px;
            margin: 10px 0;
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            transition: all 0.3s;
            border: 2px solid transparent;
        }
        
        .link-card:hover {
            background: #e8f5e9;
            border-color: #25D366;
            transform: translateX(5px);
        }
        
        .number {
            font-size: 1.1em;
            font-weight: bold;
            color: #333;
        }
        
        .whatsapp-btn {
            background: #25D366;
            color: white;
            padding: 12px 30px;
            border-radius: 25px;
            text-decoration: none;
            font-weight: bold;
            transition: all 0.3s;
            display: inline-flex;
            align-items: center;
            gap: 8px;
        }
        
        .whatsapp-btn:hover {
            background: #128C7E;
            transform: scale(1.05);
        }
        
        .instructions {
            background: #e3f2fd;
            padding: 20px;
            border-radius: 10px;
            margin-top: 30px;
            border-left: 4px solid #2196f3;
        }
        
        .instructions h3 {
            color: #1976d2;
            margin-bottom: 15px;
        }
        
        .instructions ol {
            margin-left: 20px;
            color: #555;
            line-height: 1.8;
        }
        
        .counter {
            position: fixed;
            top: 20px;
            right: 20px;
            background: white;
            padding: 15px 25px;
            border-radius: 10px;
            box-shadow: 0 5px 15px rgba(0,0,0,0.2);
        }
        
        .counter-number {
            font-size: 2em;
            font-weight: bold;
            color: #25D366;
        }
        
        .counter-label {
            color: #666;
            font-size: 0.9em;
        }
        
        footer {
            text-align: center;
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #ddd;
            color: #999;
        }
    </style>
</head>
<body>
    <div class="counter">
        <div class="counter-number" id="sent-count">0</div>
        <div class="counter-label">Messages Sent</div>
    </div>

    <div class="container">
        <h1>📱 WhatsApp Bulk Sender</h1>
        <p class="subtitle">Click each link to send your message</p>
        
        <div class="stats">
            <p><strong>📊 Total Recipients:</strong> "#
    );
    
    html.push_str(&format!("{}", numbers.len()));
    html.push_str("</p>\n");
    html.push_str(&format!("<p><strong>📅 Generated:</strong> {}</p>\n", get_current_time()));
    html.push_str("        </div>\n");
    
    html.push_str(r#"
        <div class="message-preview">
            <h3>💬 Your Message:</h3>
            <div class="message-text">"#);
    html.push_str(message);
    html.push_str(r#"</div>
        </div>
        
        <div class="instructions">
            <h3>📋 How to Use:</h3>
            <ol>
                <li>Click on the green "Send on WhatsApp" button for each number</li>
                <li>WhatsApp Web will open with the message ready</li>
                <li>Click the Send button in WhatsApp</li>
                <li>Close the tab and return here for the next number</li>
                <li>The counter at the top tracks your progress</li>
            </ol>
        </div>
        
        <div class="links-container">
            <h2 style="color:#333;margin-bottom:20px;">📞 Recipients:</h2>
"#);

    for (i, number) in numbers.iter().enumerate() {
        let clean_number = number.trim();
        let whatsapp_url = format!(
            "https://wa.me/{}?text={}",
            clean_number, encoded_message
        );
        
        html.push_str(&format!(
            r#"            <div class="link-card">
                <span class="number">{}. +{}</span>
                <a href="{}" class="whatsapp-btn" target="_blank" onclick="incrementCounter()">
                    📤 Send on WhatsApp
                </a>
            </div>
"#,
            i + 1,
            clean_number,
            whatsapp_url
        ));
    }

    html.push_str(r#"
        </div>
        
        <footer>
            Generated by WhatsApp Bulk Sender v1.0 | Created by Kareem Ali
        </footer>
    </div>
    
    <script>
        let sentCount = 0;
        
        function incrementCounter() {
            sentCount++;
            document.getElementById('sent-count').textContent = sentCount;
        }
        
        // Keyboard shortcut: Press 'n' for next
        document.addEventListener('keydown', function(e) {
            if (e.key === 'n' || e.key === 'N') {
                const links = document.querySelectorAll('.whatsapp-btn');
                if (sentCount < links.length) {
                    links[sentCount].click();
                }
            }
        });
    </script>
</body>
</html>"#);

    html
}

fn url_encode(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '\n' => "%0A".to_string(),
            '!' => "%21".to_string(),
            '#' => "%23".to_string(),
            '$' => "%24".to_string(),
            '&' => "%26".to_string(),
            '\'' => "%27".to_string(),
            '(' => "%28".to_string(),
            ')' => "%29".to_string(),
            '*' => "%2A".to_string(),
            '+' => "%2B".to_string(),
            ',' => "%2C".to_string(),
            '/' => "%2F".to_string(),
            ':' => "%3A".to_string(),
            ';' => "%3B".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            '@' => "%40".to_string(),
            '[' => "%5B".to_string(),
            ']' => "%5D".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

fn get_current_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    
    let days = secs / 86400;
    let year = 1970 + (days / 365);
    let month = ((days % 365) / 30) + 1;
    let day = ((days % 365) % 30) + 1;
    
    let time = secs % 86400;
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hours, minutes)
}
