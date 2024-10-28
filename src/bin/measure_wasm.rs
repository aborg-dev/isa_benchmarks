//! Running a WASI compiled WebAssembly module with Wasmer.
//!
//! This example illustrates how to run WASI modules with
//! Wasmer. To run WASI we have to have to do mainly 3 steps:
//!
//!   1. Create a `WasiEnv` instance
//!   2. Attach the imports from the `WasiEnv` to a new instance
//!   3. Run the `WASI` module.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example wasi-manual-setup --release --features "cranelift,wasi"
//! ```
//!
//! Ready?

use std::sync::Arc;

use wasmer::wasmparser::Operator;
use wasmer::{CompilerConfig, Cranelift, EngineBuilder, Instance, Module, Store};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_middlewares::Metering;
use wasmer_wasix::WasiEnv;

const START_POINTS: u64 = 1_000_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_path = std::env::args().nth(1).expect("no path given");

    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read(wasm_path)?;

    // Let's define our cost function.
    //
    // This function will be called for each `Operator` encountered during
    // the Wasm module execution. It should return the cost of the operator
    // that it received as it first argument.
    let cost_function = |operator: &Operator| -> u64 {
        match operator {
            Operator::LocalGet { .. } | Operator::I32Const { .. } | Operator::I64Const { .. } => 0,
            _ => 1,
        }
    };

    // Now let's create our metering middleware.
    //
    // `Metering` needs to be configured with a limit and a cost function.
    //
    // For each `Operator`, the metering middleware will call the cost
    // function and subtract the cost from the remaining points.
    let metering = Arc::new(Metering::new(START_POINTS, cost_function));
    let mut compiler_config = Cranelift::default();
    compiler_config.push_middleware(metering);

    // Create a Store.
    let mut store = Store::new(EngineBuilder::new(compiler_config));

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    println!("Starting `tokio` runtime...");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = runtime.enter();

    println!("Creating `WasiEnv`...");
    // First, we create the `WasiEnv`
    let mut wasi_env = WasiEnv::builder("hello")
        // .args(&["world"])
        // .env("KEY", "Value")
        .finalize(&mut store)?;

    println!("Instantiating module with WASI imports...");
    // Then, we get the import object related to our WASI
    // and attach it to the Wasm instance.
    let import_object = wasi_env.import_object(&mut store, &module)?;
    let instance = Instance::new(&mut store, &module, &import_object)?;

    println!("Attach WASI memory...");
    // // Attach the memory export
    // let memory = instance.exports.get_memory("memory")?;
    // wasi_env.data_mut(&mut store).set_memory(memory.clone());

    wasi_env.initialize(&mut store, instance.clone())?;

    println!("Call WASI `_start` function...");
    // And we just call the `_start` function!
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    wasi_env.on_exit(&mut store, None);

    let remaining_points_after_first_call = get_remaining_points(&mut store, &instance);
    if let MeteringPoints::Remaining(value) = remaining_points_after_first_call {
        println!(
            "Remaining points after the first call: {:?}, spent: {}",
            value,
            START_POINTS - value
        );
    } else {
        println!("All points exhausted");
    }

    Ok(())
}
