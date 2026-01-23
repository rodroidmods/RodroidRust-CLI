use console::{Emoji, Style, Term, style};

pub static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");
pub static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
pub static WRENCH: Emoji<'_, '_> = Emoji("🔧 ", "");
pub static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "* ");

pub struct Theme {
    pub primary: Style,
    pub success: Style,
    pub warning: Style,
    pub dim: Style,
    pub bold: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Style::new().cyan().bold(),
            success: Style::new().green().bold(),
            warning: Style::new().yellow(),
            dim: Style::new().dim(),
            bold: Style::new().bold(),
        }
    }
}

pub fn print_banner() {
    let theme = Theme::default();
    let term = Term::stdout();
    let _ = term.write_line("");
    let banner = r#"
   ╭──────────────────────────────────────────╮
   │                                          │
   │   ╔═╗╔╗╔╔╦╗╦═╗╔═╗╦╔╦╗  ╦═╗╦ ╦╔═╗╔╦╗      │
   │   ╠═╣║║║ ║║╠╦╝║ ║║ ║║  ╠╦╝║ ║╚═╗ ║       │
   │   ╩ ╩╝╚╝═╩╝╩╚═╚═╝╩═╩╝  ╩╚═╚═╝╚═╝ ╩       │
   │                                          │
   ╰──────────────────────────────────────────╯"#;
    println!("{}", theme.primary.apply_to(banner));
    println!();
}

pub fn print_hint(text: &str) {
    let theme = Theme::default();
    println!(
        "    {} {}",
        theme.dim.apply_to("→"),
        theme.dim.apply_to(text)
    );
}

pub fn print_key_value(key: &str, value: &str) {
    let theme = Theme::default();
    println!(
        "  {} {}",
        theme.dim.apply_to(format!("{}:", key)),
        theme.bold.apply_to(value)
    );
}

pub fn print_section_title(emoji: Emoji<'_, '_>, text: &str) {
    let theme = Theme::default();
    println!();
    println!("  {}{}", emoji, theme.bold.apply_to(text));
    println!();
}

pub fn print_completion_message(project_dir: &str, package: &str) {
    let theme = Theme::default();

    println!();
    println!(
        "  {}{}",
        theme.success.apply_to(SPARKLES),
        theme.success.apply_to("Project created successfully!")
    );
    println!();

    let location_label = "Location: ";
    let package_label = "Package:  ";
    let content_width = std::cmp::max(
        project_dir.len() + location_label.len(),
        package.len() + package_label.len(),
    );
    let box_width = content_width + 4;
    let border = "─".repeat(box_width);

    println!("  {}", theme.dim.apply_to(format!("╭{}╮", border)));
    println!(
        "  {}  {}{}",
        theme.dim.apply_to("│"),
        theme.dim.apply_to(location_label),
        style(project_dir).cyan()
    );
    println!(
        "  {}  {}{}",
        theme.dim.apply_to("│"),
        theme.dim.apply_to(package_label),
        style(package).cyan()
    );
    println!("  {}", theme.dim.apply_to(format!("╰{}╯", border)));

    println!();
    println!("  {}", theme.bold.apply_to("Next steps:"));
    println!(
        "    {}  {}",
        theme.dim.apply_to("1."),
        format!("cd {}", style(project_dir).cyan())
    );
    println!(
        "    {}  {}",
        theme.dim.apply_to("2."),
        "Open in Android Studio"
    );
    println!(
        "    {}  {}",
        theme.dim.apply_to("3."),
        format!("{} Build and run!", ROCKET)
    );
    println!();
}

pub fn print_dry_run_header() {
    let theme = Theme::default();
    println!();
    println!(
        "  {} {}",
        theme.warning.apply_to("⚡"),
        theme.warning.apply_to("DRY RUN - No files will be created")
    );
}
