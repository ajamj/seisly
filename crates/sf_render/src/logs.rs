use sf_compute::resampling::interpolate_station;
use sf_core::domain::log::Curve;
use sf_core::domain::surface::Mesh;
use sf_core::domain::trajectory::Trajectory;

pub struct LogRenderer;

impl LogRenderer {
    pub fn generate_strip_mesh(traj: &Trajectory, curve: &Curve, width: f32) -> Mesh {
        if traj.stations.len() < 2 || curve.values.is_empty() {
            return Mesh::new(Vec::new(), Vec::new());
        }

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut colors = Vec::new();

        let n = curve.values.len();
        let md_min = traj.stations.first().unwrap().md;
        let md_max = traj.stations.last().unwrap().md;

        // Find curve range for normalization
        let mut min_val = f32::MAX;
        let mut max_val = f32::MIN;
        let mut has_valid = false;
        for &v in &curve.values {
            if v != curve.null_value {
                min_val = min_val.min(v);
                max_val = max_val.max(v);
                has_valid = true;
            }
        }

        if !has_valid {
            min_val = 0.0;
            max_val = 1.0;
        }

        let range = if max_val > min_val {
            max_val - min_val
        } else {
            1.0
        };

        for i in 0..n {
            let md_norm = if n > 1 {
                i as f64 / (n - 1) as f64
            } else {
                0.0
            };
            let md = md_min + md_norm * (md_max - md_min);

            let station = interpolate_station(traj, md);

            // Width offset (just in X for now)
            let v_left = [
                station.x as f32 - width / 2.0,
                station.y as f32,
                station.z as f32,
            ];
            let v_right = [
                station.x as f32 + width / 2.0,
                station.y as f32,
                station.z as f32,
            ];

            let val = curve.values[i];
            let norm_val = if val == curve.null_value {
                0.0
            } else {
                (val - min_val) / range
            };

            // Simple gray-scale color mapping
            let color = [norm_val, norm_val, norm_val];

            vertices.push(v_left);
            vertices.push(v_right);
            colors.push(color);
            colors.push(color);

            if i > 0 {
                let curr = i as u32 * 2;
                let prev = (i - 1) as u32 * 2;

                indices.push(prev);
                indices.push(prev + 1);
                indices.push(curr);

                indices.push(prev + 1);
                indices.push(curr + 1);
                indices.push(curr);
            }
        }

        let mut mesh = Mesh::new(vertices, indices);
        mesh.colors = Some(colors);
        mesh
    }
}
