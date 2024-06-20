use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Formula {
    name: String,
    build_dependencies: Vec<String>,
    dependencies: Vec<String>,
    test_dependencies: Vec<String>,
    recommended_dependencies: Vec<String>,
    opional_dependencies: Option<Vec<String>>,
}

impl Formula {
    pub fn take_name(self) -> String {
        self.name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn build_dependencies(&self) -> &[String] {
        &self.build_dependencies
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn test_dependencies(&self) -> &[String] {
        &self.test_dependencies
    }

    pub fn recommended_dependencies(&self) -> &[String] {
        &self.recommended_dependencies
    }

    pub fn optional_dependencies(&self) -> Option<&[String]> {
        self.opional_dependencies.as_deref()
    }
}
