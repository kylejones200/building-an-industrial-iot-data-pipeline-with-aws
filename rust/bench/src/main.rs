use building_an_industrial_iot_data_pipeline_with_aws_core::{analyze_sensor_stats, simulate_iot_values};
use std::time::Instant;
fn main() {
    let n = 10_000usize;
    let s = 5usize;
    let start = Instant::now();
    for _ in 0..200 {
        let v = simulate_iot_values(n, s, 42);
        let _ = analyze_sensor_stats(&v, n, s);
    }
    println!("elapsed: {:.3}s", start.elapsed().as_secs_f64());
}
