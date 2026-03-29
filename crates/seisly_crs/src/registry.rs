//! CRS Registry for managing known CRS definitions

use sf_core::Crs;
use std::collections::HashMap;

/// Registry of known CRS definitions
pub struct CrsRegistry {
    cache: HashMap<String, Crs>,
}

impl CrsRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            cache: HashMap::new(),
        };
        registry.register_common();
        registry
    }

    /// Register common CRS definitions
    fn register_common(&mut self) {
        // WGS84 - Geographic
        self.register(Crs::from_epsg(4326));

        // UTM zones (common examples)
        self.register(Crs::from_epsg(32648)); // UTM 48N
        self.register(Crs::from_epsg(32649)); // UTM 49N
        self.register(Crs::from_epsg(32650)); // UTM 50N

        // Web Mercator (for web mapping)
        self.register(Crs::from_epsg(3857));
    }

    /// Register a CRS definition
    pub fn register(&mut self, crs: Crs) {
        let key = crs.definition.clone();
        self.cache.insert(key, crs);
    }

    /// Get a CRS by definition string
    pub fn get(&self, definition: &str) -> Option<&Crs> {
        self.cache.get(definition)
    }

    /// Get a CRS by EPSG code
    pub fn from_epsg(&self, code: u32) -> Option<Crs> {
        Some(Crs::from_epsg(code))
    }

    /// Check if a CRS is registered
    pub fn contains(&self, definition: &str) -> bool {
        self.cache.contains_key(definition)
    }
}

impl Default for CrsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let registry = CrsRegistry::new();
        assert!(registry.get("EPSG:4326").is_some());
        assert!(registry.get("EPSG:32648").is_some());
        assert!(registry.get("EPSG:3857").is_some());
    }

    #[test]
    fn test_registry_contains() {
        let registry = CrsRegistry::new();
        assert!(registry.contains("EPSG:4326"));
        assert!(!registry.contains("EPSG:99999"));
    }

    #[test]
    fn test_from_epsg() {
        let registry = CrsRegistry::new();
        let crs = registry.from_epsg(4326).unwrap();
        assert_eq!(crs.epsg_code(), Some(4326));
    }
}
