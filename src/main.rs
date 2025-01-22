use roc_can::expr::Expr;
use roc_load::ExecutionMode;
use roc_load::FunctionKind;
use roc_load::LoadedModule;
use roc_load::Threading;
// use roc_module::symbol::{Interns, Symbol};
use roc_packaging::cache::RocCacheDir;
use roc_reporting::report::{RenderTarget, DEFAULT_PALETTE};
// use roc_solve::module::Solved;
use roc_target::Target::Wasm32;
use roc_types::subs::Variable;
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

            let thing = Thing { module };
            thing.print_stuff();
        }
        Err(loading_problem) => {
            println!("ERROR DURING LOAD AND TYPECHECK {:?}", loading_problem);
        }
    }
}

struct Thing {
    module: LoadedModule,
}

impl Thing {
    fn var_str(&self, var: &Variable) -> String {
        format!(
            "{:?} = {:?};",
            var,
            self.module.solved.inner().dbg(var.clone())
        )
    }
    fn print_stuff(&self) {
        dbg!(&self.module.filename);

        dbg!(&self.module.exposed_to_host);

        self.module
            .declarations_by_id
            .iter()
            .for_each(|(id, decl)| {
                println!("MODULE ID {:?}", &id);
                println!("VARIABLES");
                decl.variables.iter().for_each(|var| {
                    println!("{}", self.var_str(var));
                });
                println!("EXPRESSIONS");
                decl.expressions.iter().for_each(|loc_expr| {
                    self.print_expr(&loc_expr.value);
                });
            });
    }

    fn print_expr(&self, expr: &Expr) {
        match expr {
            Expr::Str(str) => {
                println!("STR {:?};", str);
            }
            Expr::Var(symbol, var) => {
                println!(
                    "VAR {:?} = {:?};",
                    &symbol.as_str(&self.module.interns),
                    &self.var_str(var)
                );
            }
            Expr::Call(fn_data, arguments, _called_via) => {
                println!("BEGIN CALL");
                println!("  - function var {}", self.var_str(&fn_data.0));
                println!("  - closure var {}", self.var_str(&fn_data.2));
                println!("  - return var {}", self.var_str(&fn_data.3));
                println!("  - fx_var {}", self.var_str(&fn_data.4));
                println!("  - callee_expr {:?}", self.print_expr(&fn_data.1.value));

                for (arg_var, loc_expr) in arguments.iter() {
                    println!(
                        "  - arg: {:?} loc_expr: {:?}",
                        self.var_str(arg_var),
                        self.print_expr(&loc_expr.value)
                    );
                }
                println!("END CALL;");
            }
            // Expr::When {
            //     loc_cond, branches, ..
            // } => {
            //     println!("WHEN");
            //     print_expr(&loc_cond.value, self.var_str);

            //     for branch in branches.iter() {
            //         println!("  - when branch");
            //         print_expr(&branch.value.value, self.var_str);
            //     }
            // }
            _ => {
                println!("UNSUPPORTED {:?}", expr);
            }
        }
    }
}
