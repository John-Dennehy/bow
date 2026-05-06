use miette::Result;

pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub is_installed: bool,
    pub repository: String,
}

pub trait PackageProvider {
    fn name(&self) -> &'static str;
    
    /// Search for packages by a query string
    fn search(&self, query: &str) -> Result<Vec<Package>>;
    
    /// Install the given packages
    fn install(&self, packages: &[String]) -> Result<()>;
    
    /// Remove the given packages
    fn remove(&self, packages: &[String]) -> Result<()>;
    
    /// Update all packages on the system
    fn update(&self) -> Result<()>;
    
    /// Clean the system of orphans and package caches
    fn clean(&self) -> Result<()>;
}
