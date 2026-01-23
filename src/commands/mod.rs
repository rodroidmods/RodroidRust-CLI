pub mod completions;
pub mod doctor;
pub mod list;
pub mod new;

pub use completions::generate_completions;
pub use doctor::run_doctor;
pub use list::list_templates;
pub use new::init_project;
pub use new::new_project;
