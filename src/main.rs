fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Missing args. Usage: {} app.roc", args[0]);
        ExitCode::MissingArgs.exit();
    }

    let file_path = std::path::PathBuf::from(&args[1]);

    if !file_path.exists() {
        println!("File not found. Usage: {} app.roc", args[0]);
        ExitCode::FileNotFound.exit();
    }

    if !matches!(
        file_path.extension().and_then(|ext| ext.to_str()),
        Some("roc")
    ) {
        println!(
            "File doesn't end with '.roc' extension. Usage: {} app.roc",
            args[0]
        );
        ExitCode::InvalidExtension.exit();
    }

    // Load and typecheck the roc file
    let arena = &bumpalo::Bump::new();

    let load_config = roc_load::LoadConfig {
        target: roc_target::Target::Wasm32,
        function_kind: roc_load::FunctionKind::LambdaSet,
        threading: roc_load::Threading::Single,
        render: roc_reporting::report::RenderTarget::ColorTerminal,
        palette: roc_reporting::report::DEFAULT_PALETTE,
        exec_mode: roc_load::ExecutionMode::Check,
    };

    let opt_main_path = None;
    let loaded = roc_load::load_and_typecheck(
        arena,
        file_path.clone(),
        opt_main_path,
        roc_packaging::cache::RocCacheDir::Disallowed,
        load_config,
    );

    // Process the loaded module
    match loaded {
        Ok(module) => {
            println!("---- DEBUGGING {} 'main': ----", file_path.display());

            let roc_loaded_module = RocLoadedModule { module };

            if let Some(main_expr) = roc_loaded_module.find_symbol("main") {
                let mut buf = String::new();
                roc_loaded_module.print_expr(&mut buf, main_expr);
                println!("{}", buf);
            } else {
                println!("No 'main' function found");
            }
        }
        Err(roc_load_internal::file::LoadingProblem::FormattedReport(report, _)) => {
            print!("{}", report);
        }
        Err(loading_problem) => {
            println!("ERROR DURING LOAD AND TYPECHECK {:?}", loading_problem);
        }
    }
}

// We wrap the LoadedModule in a struct so we can implement some helper methods
struct RocLoadedModule {
    module: roc_load::LoadedModule,
}

impl RocLoadedModule {
    /// Look for a symbol in the typechecked `CheckedModule` which includes
    /// the sovlved (types) in `Subs`titusions and declarations.
    fn find_symbol(&self, name: &str) -> Option<&roc_can::expr::Expr> {
        for (_, module) in self.module.typechecked.iter() {
            for (i, symbol) in module.decls.symbols.iter().enumerate() {
                if self.symbol_str(&symbol.value) == name {
                    return Some(&module.decls.expressions[i].value);
                }
            }
        }
        None
    }

    /// Represents a roc internal `Symbol` as a string
    fn symbol_str(&self, symbol: &roc_module::symbol::Symbol) -> &str {
        symbol.as_str(&self.module.interns)
    }

    /// Represents a roc internal module `Symbol` as a string
    fn module_str(&self, symbol: &roc_module::symbol::Symbol) -> Option<&str> {
        if symbol.module_string(&self.module.interns).as_str() == "#UserApp" {
            None
        } else {
            Some(symbol.module_string(&self.module.interns))
        }
    }

    /// Recursively traverse the expression tree and convert it to
    /// an S-Expression like string
    fn print_expr(&self, buf: &mut String, expr: &roc_can::expr::Expr) {
        use roc_can::expr::Expr;
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
                    buf.push_str(self.symbol_str(symbol));
                }
            }
            Expr::Call(fn_data, arguments, _called_via) => {
                buf.push('(');
                self.print_expr(buf, &fn_data.1.value);
                for (_arg_var, loc_expr) in arguments {
                    buf.push(' ');
                    self.print_expr(buf, &loc_expr.value);
                }
                buf.push(')');
            }
            Expr::LetNonRec(_, loc_expr) => {
                // for now ... ignore non-recursive definition e.g. apples = 2
                self.print_expr(buf, &loc_expr.value);
            }
            _ => {
                // we don't handle all the node types yet...
                buf.push_str(&format!("(UNSUPPORTED {:?})", expr));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ExitCode {
    MissingArgs = 1,
    FileNotFound = 2,
    InvalidExtension = 3,
}

impl ExitCode {
    fn exit(&self) -> ! {
        std::process::exit(*self as i32)
    }
}
