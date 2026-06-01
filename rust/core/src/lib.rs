//! Simulate IoT sensor values and compute per-sensor mean/std.

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_f64(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.0 >> 33) as f64 / (1u64 << 31) as f64
    }

    fn normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-12);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// Flat sensor matrix: row-major `n_points * n_sensors`.
pub fn simulate_iot_values(n_points: usize, n_sensors: usize, seed: u64) -> Vec<f64> {
    let mut rng = Lcg::new(seed);
    let mut out = vec![0.0; n_points * n_sensors];
    for s in 0..n_sensors {
        let base = 50.0 + s as f64 * 10.0;
        let trend_slope = if s % 2 == 0 { 10.0 / n_points.max(1) as f64 } else { 0.0 };
        for t in 0..n_points {
            let noise = rng.normal() * 5.0;
            let trend = trend_slope * t as f64;
            out[t * n_sensors + s] = base + trend + noise;
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensorStats {
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
}

pub fn analyze_sensor_stats(values: &[f64], n_points: usize, n_sensors: usize) -> SensorStats {
    assert_eq!(values.len(), n_points * n_sensors);
    let mut means = vec![0.0; n_sensors];
    let mut stds = vec![0.0; n_sensors];

    for s in 0..n_sensors {
        let mut sum = 0.0;
        for t in 0..n_points {
            sum += values[t * n_sensors + s];
        }
        let mean = sum / n_points.max(1) as f64;
        means[s] = mean;

        let mut var = 0.0;
        for t in 0..n_points {
            let d = values[t * n_sensors + s] - mean;
            var += d * d;
        }
        stds[s] = (var / n_points.max(1) as f64).sqrt();
    }

    SensorStats { means, stds }
}
