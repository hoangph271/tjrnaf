use deno_core::{error::AnyError, op, Extension, FsModuleLoader, RuntimeOptions};
use std::rc::Rc;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Builder.build() failed");

    if let Err(error) = runtime.block_on(tirnaf_run("./example.js")) {
        eprintln!("error: {}", error);
    } else {
        println!("It went #OK, how...?")
    }
}

async fn tirnaf_run(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let tirnaf_extension = Extension::builder().ops(vec![op_read_file::decl()]).build();

    let mut js_runtime = deno_core::JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![tirnaf_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[tirnaf:runtime.js]", include_str!("../runtime.js"))?;

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}
