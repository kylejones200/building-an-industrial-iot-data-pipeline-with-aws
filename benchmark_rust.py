#!/usr/bin/env python3
import argparse, sys, time
from pathlib import Path
import numpy as np
ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import simulate_iot_values, analyze_sensor_stats

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--n", type=int, default=10_000)
    p.add_argument("--sensors", type=int, default=5)
    p.add_argument("--iterations", type=int, default=200)
    a = p.parse_args()
    t0 = time.perf_counter()
    for _ in range(a.iterations):
        v = simulate_iot_values(a.n, a.sensors, 42)
        analyze_sensor_stats(v, a.n, a.sensors)
    py_s = time.perf_counter() - t0
    try:
        import building_an_industrial_iot_data_pipeline_with_aws_rs as rs
    except ImportError:
        print("Build: cd rust && maturin develop --release -m py/Cargo.toml")
        print(f"Python: {py_s:.3f}s"); return
    rs_s = rs.bench_kernel_py(a.n, a.sensors, 42, a.iterations)
    print(f"Python: {py_s:.3f}s Rust: {rs_s:.3f}s speedup: {py_s/rs_s:.1f}x")
    v = simulate_iot_values(100, 3, 42)
    py_m, py_s2 = analyze_sensor_stats(v, 100, 3)
    rs_m, rs_s2 = rs.analyze_sensor_stats_py(v, 100, 3)
    np.testing.assert_allclose(py_m, rs_m, rtol=1e-10)
    np.testing.assert_allclose(py_s2, rs_s2, rtol=1e-10)
    print("Correctness: OK")
if __name__ == "__main__": main()
