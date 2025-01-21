use roc_load::FunctionKind;
use roc_packaging::cache::RocCacheDir;
use roc_reporting::report::RenderTarget;
use roc_target::Target::Wasm32;
use std::path::PathBuf;

fn main() {
    println!("RUNNING FROM WASI");

    let arena = &bumpalo::Bump::new();
    let roc_cache = std::path::PathBuf::from(".cache").join("roc");
    let loaded = roc_load::load_and_typecheck_str(
        arena,
        PathBuf::from("ExampleModule.roc"),
        r#"
        module [one]

        one = 1 + 2
        "#,
        PathBuf::from("."),
        None,
        Wasm32,
        FunctionKind::LambdaSet,
        RenderTarget::Generic,
        RocCacheDir::Persistent(roc_cache.as_path()),
        roc_reporting::report::DEFAULT_PALETTE,
    );

    match loaded {
        Ok(module) => {
            println!("DEBUGGING LOADED MODULE----");
            println!("DECLARATIONS:");

            module.declarations_by_id.iter().for_each(|(id, decl)| {
                println!("{:?}: {:?}", &id, &decl.expressions);
            });
        }
        Err(loading_problem) => {
            println!("ERROR DURING LOAD AND TYPECHECK {:?}", loading_problem);
        }
    }
}
