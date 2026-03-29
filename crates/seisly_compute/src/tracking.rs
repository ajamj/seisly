//! Event tracking and snapping algorithms for seismic interpretation.

/// Snaps an initial sample index to the nearest local extrema (peak or trough) within a window.
pub fn snap_to_extrema(trace: &[f32], initial_idx: usize, window: usize, is_peak: bool) -> usize {
    let start = initial_idx.saturating_sub(window);
    let end = (initial_idx + window + 1).min(trace.len());

    let mut best_idx = initial_idx;
    let mut best_val = if initial_idx < trace.len() {
        trace[initial_idx]
    } else {
        0.0
    };

    #[allow(clippy::needless_range_loop)]
    for i in start..end {
        let val = trace[i];
        if is_peak {
            if val > best_val {
                best_val = val;
                best_idx = i;
            }
        } else {
            if val < best_val {
                best_val = val;
                best_idx = i;
            }
        }
    }

    best_idx
}

use crate::seismic::SeismicVolume;
use std::collections::{HashSet, VecDeque};

/// Follows an event across a seismic volume starting from a seed point.
pub fn track_event(
    volume: &SeismicVolume,
    seed_inline: i32,
    seed_xline: i32,
    seed_sample: usize,
    is_peak: bool,
    threshold: f32, // Similarity threshold (e.g. min amplitude as fraction of seed)
) -> Vec<(i32, i32, usize)> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let seed_trace = volume.provider.get_trace(seed_inline, seed_xline);
    if seed_trace.is_none() {
        return results;
    }
    let seed_val = seed_trace.as_ref().unwrap()[seed_sample];

    queue.push_back((seed_inline, seed_xline, seed_sample));
    visited.insert((seed_inline, seed_xline));

    while let Some((curr_il, curr_xl, curr_s)) = queue.pop_front() {
        results.push((curr_il, curr_xl, curr_s));

        // Check 4 neighbors
        let neighbors = [
            (curr_il + 1, curr_xl),
            (curr_il - 1, curr_xl),
            (curr_il, curr_xl + 1),
            (curr_il, curr_xl - 1),
        ];

        for (nil, nxl) in neighbors {
            if visited.contains(&(nil, nxl)) {
                continue;
            }

            if let Some(ntrace) = volume.provider.get_trace(nil, nxl) {
                // Snap to nearest extrema
                let snapped = snap_to_extrema(&ntrace, curr_s, 3, is_peak);
                let nval = ntrace[snapped];

                // Simple similarity check: amplitude must be at least threshold * seed_val
                // (Assuming seed_val is non-zero and same polarity)
                let matches = if is_peak {
                    nval >= threshold * seed_val
                } else {
                    nval <= threshold * seed_val
                };

                if matches {
                    visited.insert((nil, nxl));
                    queue.push_back((nil, nxl, snapped));
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seismic::InMemoryProvider;

    #[test]
    fn test_snap_to_peak() {
        let trace = vec![0.0, 0.5, 1.0, 0.8, 0.2, -0.5, -1.0, -0.5, 0.0];
        // Peak is at index 2 (value 1.0)
        // If we start at index 1, window 2, it should find index 2
        assert_eq!(snap_to_extrema(&trace, 1, 2, true), 2);
        // If we start at index 3, window 2, it should find index 2
        assert_eq!(snap_to_extrema(&trace, 3, 2, true), 2);
    }

    #[test]
    fn test_snap_to_trough() {
        let trace = vec![0.0, 0.5, 1.0, 0.8, 0.2, -0.5, -1.0, -0.5, 0.0];
        // Trough is at index 6 (value -1.0)
        // If we start at index 5, window 2, it should find index 6
        assert_eq!(snap_to_extrema(&trace, 5, 2, false), 6);
        // If we start at index 7, window 2, it should find index 6
        assert_eq!(snap_to_extrema(&trace, 7, 2, false), 6);
    }

    #[test]
    fn test_track_event() {
        // Create a 2x2x5 volume
        // Peak is at sample 2 in all traces
        let trace = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let mut data = Vec::new();
        for _ in 0..4 {
            data.extend_from_slice(&trace);
        }

        let provider = InMemoryProvider {
            data,
            inline_range: (0, 1),
            crossline_range: (0, 1),
            sample_count: 5,
        };
        let volume = SeismicVolume::new(Box::new(provider));

        let results = track_event(&volume, 0, 0, 2, true, 0.8);

        // Should find 4 points (0,0), (1,0), (0,1), (1,1)
        assert_eq!(results.len(), 4);

        // All should be at sample 2
        for (_, _, s) in results {
            assert_eq!(s, 2);
        }
    }
}
