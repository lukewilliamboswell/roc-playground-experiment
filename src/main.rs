use roc_can::expr::Expr;
use roc_load::ExecutionMode;
use roc_load::FunctionKind;
use roc_load::Threading;
use roc_packaging::cache::RocCacheDir;
use roc_reporting::report::{RenderTarget, DEFAULT_PALETTE};
use roc_target::Target::Wasm32;
use std::path::PathBuf;

fn main() {
    let arena = &bumpalo::Bump::new();
    let load_config = roc_load::LoadConfig {
        target: Wasm32,
        function_kind: FunctionKind::LambdaSet,
        threading: Threading::Single,
        render: RenderTarget::ColorTerminal,
        palette: DEFAULT_PALETTE,
        exec_mode: ExecutionMode::Check,
    };

    let opt_main_path = None;
    let loaded = roc_load::load_and_typecheck(
        arena,
        PathBuf::from("app.roc"),
        opt_main_path,
        RocCacheDir::Disallowed,
        load_config,
    );

    match loaded {
        Ok(module) => {
            println!("----DEBUGGING LOADED MODULE----");

            module.declarations_by_id.iter().for_each(|(id, decl)| {
                println!("MODULE ID {:?}", &id);
                decl.expressions.iter().for_each(|loc_expr| {
                    print_expr(&loc_expr.value);
                });
            });
        }
        Err(loading_problem) => {
            println!("ERROR DURING LOAD AND TYPECHECK {:?}", loading_problem);
        }
    }
}

fn print_expr(loc_expr: &Expr) {
    match loc_expr {
        Expr::When { loc_cond, .. } => {
            println!("WHEN");
            print_expr(&loc_cond.value);
        }
        Expr::Call(hash_fn_data, hash_arguments, called_via) => {
            println!(
                "CALL({:?}, {:?}, {:?})",
                &hash_fn_data, &hash_arguments, &called_via
            );
        }
        _ => {
            println!("UNSUPPORTED {:?}", loc_expr);
        }
    }
}
