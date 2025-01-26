use roc_can::expr::Expr;
use roc_load::ExecutionMode;
use roc_load::FunctionKind;
use roc_load::LoadedModule;
use roc_load::Threading;
use roc_module::symbol;
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
            println!("---- DEBUGGING 'main': ----");

            let thing = Thing { module };

            if let Some(main_expr) = thing.find_symbol("main") {
                let mut buf = String::new();
                thing.print_expr(&mut buf, main_expr);
                println!("{}", buf);
            }
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
    fn find_symbol(&self, name: &str) -> Option<&Expr> {
        for (_, module) in self.module.typechecked.iter() {
            for (i, symbol) in module.decls.symbols.iter().enumerate() {
                if self.symbol_str(&symbol.value) == name {
                    return Some(&module.decls.expressions[i].value);
                }
            }
        }
        None
    }

    fn symbol_str(&self, symbol: &symbol::Symbol) -> &str {
        symbol.as_str(&self.module.interns)
    }

    fn module_str(&self, symbol: &symbol::Symbol) -> Option<&str> {
        if symbol.module_string(&self.module.interns).as_str() == "#UserApp" {
            None
        } else {
            Some(symbol.module_string(&self.module.interns))
        }
    }

    fn print_expr(&self, buf: &mut String, expr: &Expr) {
        match expr {
            Expr::Str(str) => {
                buf.push_str(&format!("{:?}", str));
            }
            Expr::Var(symbol, _var) => {
                if let Some(bultin_module_name) = self.module_str(symbol) {
                    buf.push_str(&format!(
                        "{}.{}",
                        bultin_module_name,
                        self.symbol_str(symbol),
                    ));
                } else {
                    buf.push_str(&format!("{}", self.symbol_str(symbol),));
                }
            }
            Expr::Call(fn_data, arguments, _called_via) => {
                buf.push_str("(");
                // Print function expression
                self.print_expr(buf, &fn_data.1.value);

                // Print arguments
                for (_arg_var, loc_expr) in arguments {
                    buf.push(' ');
                    self.print_expr(buf, &loc_expr.value);
                }
                buf.push(')');
            }
            Expr::Int(_, _, _, n, _) => {
                buf.push_str(&format!("(Int {})", n));
            }
            Expr::Float(_, _, _, n, _) => {
                buf.push_str(&format!("(Float {})", n));
            }
            Expr::List { loc_elems, .. } => {
                buf.push_str("(List");
                for elem in loc_elems {
                    buf.push(' ');
                    self.print_expr(buf, &elem.value);
                }
                buf.push(')');
            }
            _ => {
                buf.push_str(&format!("(UNSUPPORTED NODE {:?})", expr));
            }
        }
    }
}
