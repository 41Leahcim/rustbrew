use serde::Deserialize;

/// Formula's read from the formula file
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
    /// Returns the name as String instead of &str to prevent allocations
    pub fn take_name(self) -> String {
        self.name
    }

    /// Returns the name as &str
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns dependencies required to build the package
    pub fn build_dependencies(&self) -> &[String] {
        &self.build_dependencies
    }

    /// Returns runtime dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    /// Returns dependencies required for testing
    pub fn test_dependencies(&self) -> &[String] {
        &self.test_dependencies
    }

    /// Returns recommended dependencies
    pub fn recommended_dependencies(&self) -> &[String] {
        &self.recommended_dependencies
    }

    /// Returns optional dependencies
    pub fn optional_dependencies(&self) -> Option<&[String]> {
        self.opional_dependencies.as_deref()
    }
}
