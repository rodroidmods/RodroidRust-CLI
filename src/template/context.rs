use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TemplateContext {
    pub package_name: String,
    pub package_path: String,
    pub jni_package: String,
    pub project_name: String,
}

impl TemplateContext {
    pub fn new(package_name: String, project_name: String) -> Self {
        let package_path = package_name.replace('.', "/");
        let jni_package = encode_jni_package(&package_name);
        Self {
            package_name,
            package_path,
            jni_package,
            project_name,
        }
    }
}

fn encode_jni_package(package: &str) -> String {
    package.replace('_', "_1").replace('.', "_")
}
