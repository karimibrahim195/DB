
readme_content = """# WhatsApp Sender Automation

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

> A fast, reliable CLI tool built with Rust for automating WhatsApp message delivery to multiple contacts.

![Project Banner](https://img.shields.io/badge/WhatsApp%20Sender-Automation-blue?style=for-the-badge)

---

## 📋 Overview

**WhatsApp Sender Automation** is a personal automation project developed using **Rust**, designed to automate sending WhatsApp messages to multiple phone numbers through a simple and efficient command-line interface (CLI).

This project focuses on:
- ⚡ Productivity automation
- 🛠️ System-level tooling
- 🦀 Practical use of Rust for building fast and reliable CLI applications

---

## 💡 Motivation

Sending the same WhatsApp message to multiple contacts manually is **inefficient and repetitive**.

This project was created to:

| Goal | Benefit |
|------|---------|
| ⏱️ **Save time** | Automate repetitive messaging tasks |
| 🎯 **Reduce human error** | Consistent message delivery |
| 🦀 **Demonstrate Rust** | Real-world automation using systems programming |

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 📩 **Bulk Messaging** | Send WhatsApp messages to multiple numbers automatically |
| ⚙️ **CLI Interface** | Simple, intuitive command-line interface |
| 📄 **File Input** | Supports text files for phone numbers |
| 🚀 **High Performance** | Fast execution and low memory usage (Rust) |
| 🐧 **Linux Compatible** | Native Linux build (x86_64) |
| 🔒 **Local Execution** | Runs locally with no cloud dependency |

---

## 🛠️ Technology Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **CLI Parsing:** [clap](https://crates.io/crates/clap)
- **Automation Concept:** WhatsApp link-based message triggering (`wa.me`)
- **Target Platform:** Linux (x86_64)

---

## 📁 Project Structure

```
whatsapp-sender/
│
├── Cargo.toml              # Rust dependencies & metadata
├── src/
│   └── main.rs             # Main application source
│
├── numbers.txt             # Input: Phone numbers list
├── whatsapp_links.html     # Demo page with generated links
├── whatsapp-sender-linux-x86_64  # Precompiled binary
└── README.md               # This file
```

---

## 🔧 How It Works

1. **📖 Read Input** — Phone numbers are loaded from `numbers.txt`
2. **🔗 Generate Links** — WhatsApp message links (`wa.me`) are created for each number
3. **🌐 Open Browser** — Links are opened sequentially with a defined delay
4. **💬 Send Message** — WhatsApp Web handles the final message delivery

---

## 📝 Input File Format

### `numbers.txt`

```
+201xxxxxxxxx
+966xxxxxxxxx
+971xxxxxxxxx
+1xxxxxxxxxx
+44xxxxxxxxxx
```

> **Note:** Include country codes (e.g., `+20` for Egypt, `+966` for Saudi Arabia)

---

## 🚀 Usage

### Quick Start (Precompiled Binary)

```bash
# 1. Make the binary executable
chmod +x whatsapp-sender-linux-x86_64

# 2. Run the tool
./whatsapp-sender-linux-x86_64
```

### Build from Source

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/whatsapp-sender.git
cd whatsapp-sender

# 2. Build the project
cargo build --release

# 3. Run the binary
./target/release/whatsapp-sender
```

---

## 📦 Binary Distribution

A precompiled Linux binary is included for convenience:

| Property | Value |
|----------|-------|
| **Platform** | Linux x86_64 |
| **Binary** | `whatsapp-sender-linux-x86_64` |
| **Dependencies** | None (statically linked) |

```bash
chmod +x whatsapp-sender-linux-x86_64
./whatsapp-sender-linux-x86_64
```

---

## 🌐 Website Content

The project includes a simple HTML page (`whatsapp_links.html`) that:
- Displays generated WhatsApp message links
- Serves as a lightweight project demo interface
- Can be hosted on GitHub Pages

---

## 💼 Use Cases

- 📢 **Personal message broadcasting**
- 📚 **Study group notifications**
- 📅 **Event reminders**
- ⚡ **Small-scale personal automation tasks**

---

## ⚠️ Limitations

- 🔐 Requires an active WhatsApp Web session
- 👤 Intended for **personal and educational use only**
- 🚫 **Not designed for spam** or large-scale commercial messaging

---

## 🔮 Future Enhancements

- [ ] CLI argument customization (custom message, delay settings)
- [ ] Message templates support
- [ ] Scheduling support (cron-like functionality)
- [ ] CSV file support for contacts
- [ ] Multi-platform binaries (Windows, macOS)
- [ ] Configuration file support (TOML/YAML)

---

## 📂 Repository & Website

| Resource | Link |
|----------|------|
| 🐙 **GitHub Repository** | [github.com/yourusername/whatsapp-sender](https://github.com/yourusername/whatsapp-sender) |
| 🌐 **Live Demo** | [yourusername.github.io/whatsapp-sender](https://yourusername.github.io/whatsapp-sender) |

---

## 👤 Author

Developed as a personal automation project to demonstrate:
- Practical automation skills
- Rust CLI development
- System-level programming

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Made with 🦀 Rust**

</div>
"""

# Write to file
with open('/mnt/kimi/output/README.md', 'w', encoding='utf-8') as f:
    f.write(readme_content)

print("README.md created successfully!")
print(f"File saved to: /mnt/kimi/output/README.md")
print(f"File size: {len(readme_content)} characters")
