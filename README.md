# Rust Call Logger

A simple Rust-based GUI application built with \[eframe/egui] to log call records directly into an Obsidian Markdown file. Features:

* **Date/Time** entry with a `Now` quick button
* **Multi-select** Categories with dynamic addition
* **Single-select** Project dropdown with dynamic addition
* **Contact**, **Notes**, and **Next Step** fields
* Reads & persists **Categories**, **Projects**, and **LogFile** path in an external `CallLoggerConfig.md`
* Appends formatted one-liners to your Obsidian note (e.g. `Recently Contacted.md`)

## Installation

1. Ensure you have \[Rust] installed.
2. Clone this repository:

   ```bash
   git clone https://github.com/YOUR-USERNAME/rust-call-logger.git
   cd rust-call-logger
   ```
3. Build the release binary:

   ```bash
   cargo build --release
   ```
4. (Optional) Copy the binary into your PATH:

   ```bash
   install -m755 target/release/rust-call-logger ~/.local/bin/
   ```

## Configuration

Create (or edit) `CallLoggerConfig.md` in the project root or your INBOX folder:

```markdown
## Categories
- sales
- client
- marketing
- support

## Projects
- Project Alpha
- Project Beta
- Project Gamma

## LogFile
- /Users/yourname/Library/Mobile Documents/iCloud~md~obsidian/Documents/WORK/INBOX/Recently Contacted.md
```

* **Categories** and **Projects** lists control the dropdowns.
* **LogFile** specifies the target Markdown file for appending logs.

## Usage

Run the GUI application:

```bash
cargo run --release
```

In the window:

1. Click **🕒 Now** to set the current timestamp.
2. Select one or more categories from the menu, or add a new one.
3. Choose a project, or add a new one at the bottom of the menu.
4. Enter **Contact**, **Notes**, and **Next Step**.
5. Click **Save** to append a formatted bullet point to your configured log file.

Example entry:

```
- 📅2025-05-21T14:30:00+07:00 | 🏷️sales,client | 👤[[John Doe]] | 📁[[Project Alpha]] | 📝Reviewed scope | 🔜Send proposal
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License

MIT © situmorang.com
