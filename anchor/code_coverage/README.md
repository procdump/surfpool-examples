# Simple Anchor App

A simple Anchor application for Solana with code coverage support using Surfpool.

## Prerequisites

- [Anchor](https://www.anchor-lang.com/)
- [Surfpool](https://github.com/txtx/surfpool) with register-tracing feature
- [sbpf-coverage](https://crates.io/crates/sbpf-coverage)
- [lcov](https://github.com/linux-test-project/lcov) - for generating HTML coverage reports (`brew install lcov` on macOS, `apt install lcov` on Ubuntu, or `dnf install lcov` on Fedora)

## Build

### Build Surfpool with register-tracing

```bash
git clone https://github.com/txtx/surfpool.git
cd surfpool
cargo build --features register-tracing --release
```

### Configure Cargo.toml for coverage

To generate accurate coverage reports, you need to disable optimizations, enable debug symbols, and disable LTO in your workspace `Cargo.toml`:

```toml
[profile.release]
overflow-checks = true
lto = "off"
codegen-units = 1
debug = true
opt-level = 0

[profile.release.build-override]
opt-level = 0
incremental = false
codegen-units = 1
debug = true
```

### Build the Anchor project

```bash
cargo clean ; anchor keys sync
cargo build-sbf --tools-version v1.53 --arch v1 --debug
mkdir -p target/idl && anchor idl build -o target/idl/simple_anchor_app.json
```

> **Why this complexity?** Anchor's `anchor build` always compiles with SBPFv0. Instead, we sync the keys, build directly with `cargo build-sbf` using SBPFv1 (`--arch v1`) for better coverage results, and generate the IDL separately with `anchor idl build`.

> **Note:** At the time of writing, best coverage results are achieved with SBPFv1 (dynamic stack frames), which is why we use `--arch v1`. Only with dynamic stack frames can we safely disable optimizations (`opt-level = 0`) without hitting stack size limits. The `--tools-version` can be v1.51 or higher, and `--debug` is required for coverage to work. Starting with `cargo-build-sbf` 4.0.0, the `--debug` flag outputs artifacts to `target/deploy/debug` instead of `target/deploy`. If you are using an older version of `cargo-build-sbf`, replace `target/deploy/debug` with `target/deploy` in all the steps.

> **Warning:** If you run `anchor clean` or `cargo clean`, you must repeat the custom build steps above. Do not use `anchor build` as it will revert to SBPFv0.

## Run Tests

### Start Surfpool

Run this in the anchor project directory:

```bash
SBF_TRACE_DIR=$PWD/target/sbf_trace_dir SBF_OUT_DIR=$PWD/target/deploy/debug surfpool-tracing start
```

> **Note:** Setting `SBF_TRACE_DIR` is what signals `LiteSVM` to enable register tracing dumps. The `SBF_OUT_DIR=$PWD/target/deploy/debug` environment variable won't be necessary once Surfpool catches up with [LiteSVM's ELF data reading from program accounts](https://github.com/LiteSVM/litesvm/pull/278).

### Deploy the program manually

```bash
solana config set --url localhost
solana config get # !! ensure using localhost - debugging on mainnet is expensive !!
solana program deploy target/deploy/debug/simple_anchor_app.so
```

### Run Anchor tests

Use Surfpool instead of solana-test-validator:

```bash
anchor test --skip-local-validator --skip-build --skip-deploy
```

## Generate Coverage

Install sbpf-coverage if not already installed:

```bash
cargo install sbpf-coverage
```

Generate and view coverage report:

```bash
sbpf-coverage --src-path=$PWD/programs/simple_anchor_app/src --sbf-path=$PWD/target/deploy/debug --sbf-trace-dir=$PWD/target/sbf_trace_dir
genhtml --output-directory coverage target/sbf_trace_dir/*.lcov --rc branch_coverage=1 && open coverage/index.html
```

## License

[MIT](LICENSE) © [LimeChain](https://limechain.tech)
