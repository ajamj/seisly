//! Tracking Quality Control

use sf_core::domain::surface::Surface;

/// Tracking Quality Metrics
pub struct TrackingQuality {
    pub confidence: f32,
    pub continuity: f32,
    pub geological_plausibility: f32,
}

impl TrackingQuality {
    pub fn new(confidence: f32, continuity: f32, plausibility: f32) -> Self {
        Self {
            confidence,
            continuity,
            geological_plausibility: plausibility,
        }
    }
    
    /// Overall quality score (weighted average)
    pub fn overall_score(&self) -> f32 {
        0.5 * self.confidence + 0.3 * self.continuity + 0.2 * self.geological_plausibility
    }
    
    /// Quality rating
    pub fn rating(&self) -> QualityRating {
        let score = self.overall_score();
        if score > 0.8 {
            QualityRating::Excellent
        } else if score > 0.6 {
            QualityRating::Good
        } else if score > 0.4 {
            QualityRating::Fair
        } else {
            QualityRating::Poor
        }
    }
}

/// Quality Rating
#[derive(Debug, PartialEq)]
pub enum QualityRating {
    Excellent,
    Good,
    Fair,
    Poor,
}

/// Quality Control Analyzer
pub struct QualityAnalyzer;

impl QualityAnalyzer {
    /// Analyze tracking quality
    pub fn analyze(surface: &Surface, seismic: &dyn sf_compute::seismic::TraceProvider) -> TrackingQuality {
        let confidence = Self::compute_confidence(surface, seismic);
        let continuity = Self::compute_continuity(surface);
        let plausibility = Self::compute_geological_plausibility(surface);
        
        TrackingQuality::new(confidence, continuity, plausibility)
    }
    
    /// Compute confidence from tracking algorithm
    fn compute_confidence(surface: &Surface, _seismic: &dyn sf_compute::seismic::TraceProvider) -> f32 {
        // In production: use ML confidence values
        // For now, based on surface point density
        let num_points = surface.points().len();
        (num_points as f32 / 1000.0).min(1.0)
    }
    
    /// Compute horizon continuity
    fn compute_continuity(surface: &Surface) -> f32 {
        // Analyze surface smoothness and gaps
        // In production: implement proper continuity analysis
        0.8 // Dummy value
    }
    
    /// Compute geological plausibility
    fn compute_geological_plausibility(surface: &Surface) -> f32 {
        // Check for:
        // - Unrealistic dips (> 90 degrees)
        // - Depth inversions
        // - Stratigraphic consistency
        // In production: implement proper checks
        0.9 // Dummy value
    }
}

/// Quality Control Report
pub struct QCReport {
    pub quality: TrackingQuality,
    pub issues: Vec<QCIssue>,
    pub recommendations: Vec<String>,
}

/// QC Issue
#[derive(Debug)]
pub struct QCIssue {
    pub issue_type: IssueType,
    pub severity: Severity,
    pub location: String,
    pub description: String,
}

#[derive(Debug)]
pub enum IssueType {
    LowConfidence,
    Discontinuity,
    UnrealisticDip,
    StratigraphicViolation,
    FaultIntersection,
}

#[derive(Debug)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl QualityAnalyzer {
    /// Generate QC report
    pub fn generate_report(
        surface: &Surface,
        seismic: &dyn sf_compute::seismic::TraceProvider,
    ) -> QCReport {
        let quality = Self::analyze(surface, seismic);
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check for issues
        if quality.confidence < 0.5 {
            issues.push(QCIssue {
                issue_type: IssueType::LowConfidence,
                severity: Severity::High,
                location: "Overall".to_string(),
                description: "Low tracking confidence".to_string(),
            });
            recommendations.push("Review seed point selection".to_string());
        }
        
        if quality.continuity < 0.5 {
            issues.push(QCIssue {
                issue_type: IssueType::Discontinuity,
                severity: Severity::Medium,
                location: "Surface".to_string(),
                description: "Horizon discontinuities detected".to_string(),
            });
            recommendations.push("Consider fault-guided tracking".to_string());
        }
        
        QCReport {
            quality,
            issues,
            recommendations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_rating() {
        let quality = TrackingQuality::new(0.9, 0.8, 0.9);
        assert_eq!(quality.rating(), QualityRating::Excellent);
        
        let quality2 = TrackingQuality::new(0.5, 0.5, 0.5);
        assert_eq!(quality2.rating(), QualityRating::Fair);
        
        let quality3 = TrackingQuality::new(0.2, 0.3, 0.2);
        assert_eq!(quality3.rating(), QualityRating::Poor);
    }

    #[test]
    fn test_overall_score() {
        let quality = TrackingQuality::new(1.0, 0.5, 0.5);
        let score = quality.overall_score();
        
        // 0.5*1.0 + 0.3*0.5 + 0.2*0.5 = 0.5 + 0.15 + 0.1 = 0.75
        assert!((score - 0.75).abs() < 0.01);
    }
}
