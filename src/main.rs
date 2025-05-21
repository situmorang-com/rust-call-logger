use chrono::Local;
use eframe::{egui, App, Frame, NativeOptions, run_native};
use egui::TextEdit;
use std::fs::{create_dir_all, OpenOptions, write};
use std::io::{Write as IoWrite, Result as IoResult};
use std::path::PathBuf;

/// Load categories, projects, and log file path from config
/// File structure example:
/// ## Categories
/// - sales
/// - client
/// ## Projects
/// - Project Alpha
/// - Project Beta
/// ## LogFile
/// - /path/to/Recently Contacted.md
fn load_config(path: &PathBuf) -> (Vec<String>, Vec<String>, String) {
    let mut cats = Vec::new();
    let mut projs = Vec::new();
    let mut logfile = String::new();
    if let Ok(text) = std::fs::read_to_string(path) {
        enum Section { None, Cats, Projs, LogFile }
        let mut sec = Section::None;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with("##") {
                let header = line.trim_start_matches('#').trim();
                sec = match header {
                    h if h.eq_ignore_ascii_case("Categories") => Section::Cats,
                    h if h.eq_ignore_ascii_case("Projects")   => Section::Projs,
                    h if h.eq_ignore_ascii_case("LogFile")    => Section::LogFile,
                    _ => Section::None,
                };
            } else if line.starts_with('-') {
                let item = line.trim_start_matches('-').trim().to_string();
                match sec {
                    Section::Cats    => cats.push(item),
                    Section::Projs   => projs.push(item),
                    Section::LogFile => if logfile.is_empty() { logfile = item; },
                    _ => {}
                }
            }
        }
    }
    // Defaults
    if cats.is_empty() {
        cats = vec!["sales".into(), "client".into(), "marketing".into(), "support".into()];
    }
    if projs.is_empty() {
        projs = vec!["Project Alpha".into(), "Project Beta".into(), "Project Gamma".into()];
    }
    if logfile.is_empty() {
        logfile = String::from(
            "/Users/edmundsitumorang/Library/Mobile Documents/iCloud~md~obsidian/Documents/WORK/INBOX/Recently Contacted.md"
        );
    }
    (cats, projs, logfile)
}

/// Save categories, projects, and log file path back to config
fn save_config(path: &PathBuf, cats: &Vec<String>, projs: &Vec<String>, logfile: &String) -> IoResult<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut content = String::new();
    content.push_str("## Categories
");
    for cat in cats { content.push_str(&format!("- {}
", cat)); }
    content.push_str("
## Projects
");
    for proj in projs { content.push_str(&format!("- {}
", proj)); }
    content.push_str("
## LogFile
");
    content.push_str(&format!("- {}", logfile));
    write(path, content)
}

struct CallLoggerApp {
    config_path: PathBuf,
    date: String,
    available_categories: Vec<String>,
    new_category: String,
    selected_categories: Vec<String>,
    projects: Vec<String>,
    new_project: String,
    selected_project: String,
    contact: String,
    notes: String,
    next_step: String,
    status: String,
    log_file: String,
}

impl Default for CallLoggerApp {
    fn default() -> Self {
        Self {
            config_path: PathBuf::new(),
            date: Local::now().to_rfc3339(),
            available_categories: Vec::new(),
            new_category: String::new(),
            selected_categories: Vec::new(),
            projects: Vec::new(),
            new_project: String::new(),
            selected_project: String::new(),
            contact: String::new(),
            notes: String::new(),
            next_step: String::new(),
            status: String::new(),
            log_file: String::new(),
        }
    }
}

impl App for CallLoggerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Log a New Call");

            // Date/time
            ui.horizontal(|ui| {
                ui.label("📅 Date/Time:");
                ui.text_edit_singleline(&mut self.date);
                // Quick-set to current timestamp
                if ui.button("🕒 Now").clicked() {
                    self.date = Local::now().to_rfc3339();
                }
            });

            // Categories (multi-select + add)
            let cats_display = if self.selected_categories.is_empty() {
                "Select…".to_owned()
            } else {
                self.selected_categories.join(",")
            };
            ui.horizontal(|ui| {
                ui.label("🏷️ Categories:");
                ui.menu_button(cats_display, |ui| {
                    for cat in &self.available_categories {
                        let mut chosen = self.selected_categories.contains(cat);
                        if ui.checkbox(&mut chosen, cat).changed() {
                            if chosen {
                                self.selected_categories.push(cat.clone());
                            } else {
                                self.selected_categories.retain(|c| c != cat);
                            }
                        }
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(&mut self.new_category).hint_text("Add new…"));
                        if ui.button("Add").clicked() {
                            let t = self.new_category.trim().to_string();
                            if !t.is_empty() && !self.available_categories.contains(&t) {
                                self.available_categories.push(t.clone());
                                self.selected_categories.push(t.clone());
                                let _ = save_config(&self.config_path,
                                                   &self.available_categories,
                                                   &self.projects,
                                                   &self.log_file);
                            }
                            self.new_category.clear();
                        }
                    });
                });
            });

            // Projects (single-select + add)
            let proj_display = if self.selected_project.is_empty() {
                "Select…".to_owned()
            } else {
                self.selected_project.clone()
            };
            ui.horizontal(|ui| {
                ui.label("📁 Project:");
                ui.menu_button(proj_display, |ui| {
                    for proj in &self.projects {
                        if ui.selectable_label(self.selected_project == *proj, proj).clicked() {
                            self.selected_project = proj.clone();
                        }
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(&mut self.new_project).hint_text("Add new project…"));
                        if ui.button("Add").clicked() {
                            let t = self.new_project.trim().to_string();
                            if !t.is_empty() && !self.projects.contains(&t) {
                                self.projects.push(t.clone());
                                self.selected_project = t.clone();
                                let _ = save_config(&self.config_path,
                                                   &self.available_categories,
                                                   &self.projects,
                                                   &self.log_file);
                            }
                            self.new_project.clear();
                        }
                    });
                });
            });

            // Contact
            ui.horizontal(|ui| { ui.label("👤 Contact:"); ui.text_edit_singleline(&mut self.contact); });

            // Notes
            ui.horizontal(|ui| { ui.label("📝 Notes:"); ui.text_edit_singleline(&mut self.notes); });

            // Next step
            ui.horizontal(|ui| { ui.label("🔜 Next step:"); ui.text_edit_singleline(&mut self.next_step); });

            // Save call log
            if ui.button("Save to Log File").clicked() {
                let cats = self.selected_categories.join(",");
                let line = format!(
                    "- 📅{} | 🏷️{} | 👤[[{}]] | 📁[[{}]] | 📝{} | 🔜{}",
                    self.date,
                    cats,
                    self.contact,
                    self.selected_project,
                    self.notes,
                    self.next_step
                );
                let target = PathBuf::from(&self.log_file);
                if let Some(parent) = target.parent() { create_dir_all(parent).ok(); }
                let _ = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target)
                    .map(|mut f| writeln!(f, "{}", line));
                self.status = format!("✅ Saved: {}", line);
                let _ = save_config(&self.config_path,
                                   &self.available_categories,
                                   &self.projects,
                                   &self.log_file);
            }

            // Status
            if !self.status.is_empty() { ui.label(&self.status); }
        });
    }
}

fn main() {
    let config_path = PathBuf::from(
        "/Users/edmundsitumorang/Library/Mobile Documents/iCloud~md~obsidian/Documents/WORK/INBOX/CallLoggerConfig.md"
    );
    let (cats_list, projs_list, logfile) = load_config(&config_path);
    let mut app = CallLoggerApp::default();
    app.config_path = config_path;
    app.available_categories = cats_list;
    app.projects = projs_list;
    app.log_file = logfile;

    let _ = run_native(
        "Call Logger",
        NativeOptions::default(),
        Box::new(move |_cc| Box::new(app)),
    );
}

