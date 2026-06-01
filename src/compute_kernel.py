"""IoT sensor simulation and analysis (numpy)."""

from __future__ import annotations

import numpy as np


def simulate_iot_values(n_points: int, n_sensors: int, seed: int = 42) -> np.ndarray:
    rng = np.random.default_rng(seed)
    out = np.zeros((n_points, n_sensors))
    for s in range(n_sensors):
        base = 50 + s * 10
        noise = rng.normal(0, 5, n_points)
        trend = np.linspace(0, 10, n_points) if s % 2 == 0 else np.zeros(n_points)
        out[:, s] = base + trend + noise
    return out.ravel(order="C")


def analyze_sensor_stats(values: np.ndarray, n_points: int, n_sensors: int) -> tuple[np.ndarray, np.ndarray]:
    m = values.reshape(n_points, n_sensors)
    return m.mean(axis=0), m.std(axis=0)
