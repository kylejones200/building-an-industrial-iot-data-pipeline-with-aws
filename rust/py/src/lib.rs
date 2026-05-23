use building_an_industrial_iot_data_pipeline_with_aws_core::{analyze_sensor_stats, simulate_iot_values};
use numpy::{PyArray1, PyReadonlyArray1, IntoPyArray};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (n_points, n_sensors, seed=42))]
fn simulate_iot_values_py<'py>(py: Python<'py>, n_points: usize, n_sensors: usize, seed: u64) -> PyResult<Bound<'py, PyArray1<f64>>> {
    Ok(simulate_iot_values(n_points, n_sensors, seed).into_pyarray(py))
}

#[pyfunction]
fn analyze_sensor_stats_py<'py>(
    py: Python<'py>,
    values: PyReadonlyArray1<f64>,
    n_points: usize,
    n_sensors: usize,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let s = analyze_sensor_stats(values.as_slice()?, n_points, n_sensors);
    Ok((s.means.into_pyarray(py), s.stds.into_pyarray(py)))
}

#[pyfunction]
#[pyo3(signature = (n_points, n_sensors, seed=42, iterations=500))]
fn bench_kernel_py(n_points: usize, n_sensors: usize, seed: u64, iterations: usize) -> PyResult<f64> {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let v = simulate_iot_values(n_points, n_sensors, seed);
        let _ = analyze_sensor_stats(&v, n_points, n_sensors);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn building_an_industrial_iot_data_pipeline_with_aws_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate_iot_values_py, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_sensor_stats_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}
