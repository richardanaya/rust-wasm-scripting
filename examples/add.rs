extern crate parity_wasm;
extern crate wasmi;

use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() {
    let module = parity_wasm::deserialize_file("experiment.wasm").expect("File to be deserialized");
    let loaded_module = wasmi::Module::from_parity_wasm_module(module).expect("Module to be valid");
    let main = ModuleInstance::new(&loaded_module, &ImportsBuilder::default())
        .expect("Failed to instantiate module")
        .run_start(&mut NopExternals)
        .expect("Failed to run start function in module");
    println!(
        "Result: {:?}",
        main.invoke_export(
            "add",
            &vec![RuntimeValue::I32(2), RuntimeValue::I32(2)],
            &mut NopExternals
        ).expect("")
    );
}
