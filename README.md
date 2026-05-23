# Building an Industrial IoT Data Pipeline with AWS

Published: yes
Medium: [https://medium.com/@kyle-t-jones/building-an-industrial-iot-data-pipeline-with-aws-dfbca02ed743](https://medium.com/@kyle-t-jones/building-an-industrial-iot-data-pipeline-with-aws-dfbca02ed743)


This project demonstrates building an industrial IoT data pipeline using AWS services.

## Business context

Industrial IoT (IIoT) is here to stay. Collecting data is largely a solved problem. The real challenge lies in building a reliable, scalable pipeline that moves sensor data from remote sites to dashboards --- while ensuring it's cleaned, contextualized, and secure along the way.

Let's walk through how to build that kind of system using AWS. This pipeline turns raw signals into insights you can act on --- all while giving you control over the edge, the cloud, and the final visualization layer.

It all starts in the field. Devices mounted on industrial equipment --- pumps, compressors, valves --- gather a steady stream of operational data. But instead of sending everything directly to the cloud, we process and filter it on the edge using AWS IoT Greengrass.

## Project Structure

```
.
├── README.md           # This file
├── main.py            # Main entry point
├── config.yaml        # Configuration file
├── requirements.txt   # Python dependencies
├── src/               # Core functions
│   ├── core.py        # IoT pipeline functions
│   └── plotting.py    # Tufte-style plotting utilities
├── tests/             # Unit tests
├── data/              # Data files
├── images/            # Generated plots and figures
├── rust/                   # Rust port (core + PyO3 + CLI bench)
├── benchmark_rust.py       # Python vs Rust benchmark
├── src/compute_kernel.py   # Python/numpy reference kernel
```

## Configuration

Edit `config.yaml` to customize:
- Data source or synthetic generation
- Number of sensors
- AWS services configuration
- Output settings

## AWS IoT Pipeline

AWS services for IoT:
- IoT Core: Device connectivity and management
- Kinesis: Real-time data streaming
- S3: Long-term data storage
- Lambda: Serverless data processing
- DynamoDB: Time series data storage

## Caveats

- By default, generates synthetic IoT sensor data.
- Full AWS pipeline requires AWS credentials and infrastructure setup.
- Real-time processing requires proper scaling configuration.

## Rust performance port

Side-by-side **Python vs Rust** implementation of the numeric hot loop — IoT sensor simulation and summary statistics. Reference PyO3 benchmark: **comparable (see `benchmark_rust.py`)** on a release build (local machine; run `benchmark_rust.py` to reproduce).

| Path | Role |
|------|------|
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/core/` | Pure Rust library |
| `rust/py/` | PyO3 bindings |
| `rust/bench/` | Standalone CLI benchmark |
| `benchmark_rust.py` | Python vs Rust timing + correctness check |

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p building_an_industrial_iot_data_pipeline_with_aws_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python ML training, solvers, and orchestration stay in Python; Rust targets the numeric hot loops. Stochastic generators validate output shapes; deterministic kernels match at tight floating-point tolerance.


## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).