//! CRS Transformer using PROJ library

use seisly_core::Crs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Failed to initialize PROJ transformer: {0}")]
    InitError(String),
    #[error("Transform failed: {0}")]
    TransformError(String),
    #[error("PROJ library error: {0}")]
    ProjError(String),
    #[error("CRS transform not implemented: {source_crs} -> {target_crs}")]
    NotImplemented {
        source_crs: String,
        target_crs: String,
    },
}

/// Transformer between two CRS definitions
#[allow(dead_code)]
pub struct Transformer {
    source: Crs,
    target: Crs,
}

impl Transformer {
    /// Create a new transformer between source and target CRS
    pub fn new(source: &Crs, target: &Crs) -> Result<Self, TransformError> {
        // Validate that we have valid CRS definitions
        if source.definition.is_empty() || target.definition.is_empty() {
            return Err(TransformError::InitError(
                "Empty CRS definition".to_string(),
            ));
        }

        Ok(Self {
            source: source.clone(),
            target: target.clone(),
        })
    }

    /// Transform points from source to target CRS
    ///
    /// Note: This is a stub implementation that returns an explicit error.
    /// A full implementation would use the proj crate to perform actual coordinate transformations.
    pub fn transform_points(&self, points: &[[f64; 3]]) -> Result<Vec<[f64; 3]>, TransformError> {
        // Return explicit error instead of silent identity transform
        Err(TransformError::NotImplemented {
            source_crs: self.source.definition.clone(),
            target_crs: self.target.definition.clone(),
        })
    }

    /// Transform a single point
    pub fn transform_point(&self, point: [f64; 3]) -> Result<[f64; 3], TransformError> {
        let points = self.transform_points(&[point])?;
        Ok(points[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_creation() {
        let source = Crs::from_epsg(4326);
        let target = Crs::from_epsg(32648);
        let transformer = Transformer::new(&source, &target);
        assert!(transformer.is_ok());
    }

    #[test]
    fn test_transformer_empty_crs() {
        let source = Crs {
            authority: None,
            code: None,
            definition: "".to_string(),
        };
        let target = Crs::from_epsg(32648);
        let result = Transformer::new(&source, &target);
        assert!(result.is_err());
    }

    #[test]
    fn test_transform_not_implemented_returns_error() {
        let source = Crs::from_epsg(4326);
        let target = Crs::from_epsg(32648);
        let transformer = Transformer::new(&source, &target).unwrap();

        let points = vec![[100.0, 0.0, 50.0]];
        let result = transformer.transform_points(&points);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            TransformError::NotImplemented {
                source_crs,
                target_crs,
            } => {
                assert!(!source_crs.is_empty());
                assert!(!target_crs.is_empty());
            }
            _ => panic!("Expected NotImplemented error, got: {:?}", err),
        }
    }

    #[test]
    fn test_transform_point_propagates_error() {
        let source = Crs::from_epsg(4326);
        let target = Crs::from_epsg(32648);
        let transformer = Transformer::new(&source, &target).unwrap();

        let result = transformer.transform_point([100.0, 0.0, 50.0]);
        assert!(result.is_err());
    }
}
