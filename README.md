WhatsApp Sender Automation (Rust)
Overview

WhatsApp Sender Automation is a personal automation project developed using Rust, designed to automate sending WhatsApp messages to multiple phone numbers through a simple and efficient command-line interface (CLI).

The project focuses on productivity automation, system-level tooling, and practical use of Rust for building fast and reliable CLI applications.

Motivation

Sending the same WhatsApp message to multiple contacts manually is inefficient and repetitive.
This project was created to:

Save time

Reduce human error

Demonstrate real-world automation using Rust

Features

📩 Send WhatsApp messages to multiple numbers automatically

⚙️ CLI-based interface

📄 Supports text files for phone numbers

🚀 Fast execution and low memory usage (Rust)

🐧 Linux-compatible build

🔒 Runs locally (no cloud dependency)

Technology Stack

Language: Rust

CLI Parsing: clap

Automation Concept: WhatsApp link-based message triggering

Target Platform: Linux (x86_64)

Project Structure
whatsapp-sender/
│
├── Cargo.toml
├── src/
│   └── main.rs
│
├── numbers.txt
├── whatsapp_links.html
├── whatsapp-sender-linux-x86_64
└── README.md

How It Works

Phone numbers are read from a text file

A WhatsApp message link is generated for each number

Links are opened sequentially with a defined delay

WhatsApp Web handles message delivery

Input File Format

numbers.txt

+201xxxxxxxxx
+966xxxxxxxxx
+971xxxxxxxxx

CLI Usage
./whatsapp-sender-linux-x86_64


(The message content and numbers are preconfigured through input files or arguments depending on setup.)

Binary Distribution

A precompiled Linux binary is included to allow running the tool without building from source.

Platform: Linux x86_64

Execution:

chmod +x whatsapp-sender-linux-x86_64
./whatsapp-sender-linux-x86_64

Website Content

The project includes a simple HTML page (whatsapp_links.html) used to demonstrate generated WhatsApp message links.
This page can also serve as a lightweight project website or demo interface.

Use Cases

Personal message broadcasting

Study group notifications

Event reminders

Small-scale personal automation tasks

Limitations

Requires an active WhatsApp Web session

Intended for personal and educational use only

Not designed for spam or large-scale commercial messaging

Future Enhancements

CLI argument customization

Message templates

Scheduling support

CSV file support

Multi-platform binaries

Repository & Website

GitHub Repository:

(Add your GitHub repo link here)

Website / Demo Page:

(Add your GitHub Pages / HTML link here)

Author

This project was developed as a personal automation project to demonstrate practical automation, Rust CLI development, and system-level programming skills.
