use std::path::Path;
use typed_arena::Arena;
use zokrates_ast::ir::Statement;
use zokrates_core::compile::{compile, CompilationArtifacts, CompileConfig};
use zokrates_field::Field;
use zokrates_fs_resolver::FileSystemResolver;

pub fn api_compile<'a, T: Field>(
    code: &'a str,
    program_path: &'a Path,
    arena: &'a Arena<String>,
) -> Result<CompilationArtifacts<'a, T, impl IntoIterator<Item = Statement<'a, T>> + 'a>, String> {
    let stdlib_path = "ZoKrateslib/zokrates_stdlib/stdlib";
    match Path::new(stdlib_path).exists() {
        true => Ok(()),
        _ => Err(format!(
            "Invalid standard library source path: {stdlib_path}"
        )),
    }?;

    let config = CompileConfig::default();
    let resolver = FileSystemResolver::with_stdlib_root(stdlib_path);
    log::debug!("Compile");

    let program = code.to_string();
    match compile::<T, _>(
        program,
        program_path.to_path_buf(),
        Some(&resolver),
        config,
        arena,
    ) {
        Ok(artifacts) => Ok(artifacts),
        Err(e) => Err(format!(
            "Compilation failed:\n\n{}",
            e.0.iter()
                .map(|e| format!("{}", e.value()))
                .collect::<Vec<_>>()
                .join("\n\n")
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;
    use zokrates_field::Bn128Field;

    #[test]
    fn test_sucessful_compilation() {
        let code = r#"
            def main(field N) -> bool{
                return N == 1;
            }
        "#;
        let code_path = PathBuf::from("/test");
        let arena = Arena::new();

        let compilation = api_compile::<Bn128Field>(code, &code_path, &arena);
        println!("{}", compilation.is_ok());
        // assert!(compilation.is_ok());

        let (compiled_program, _abi) = compilation.unwrap().into_inner();
        let mut buffer = Cursor::new(Vec::new());
        let constrain_count = compiled_program.serialize(&mut buffer).unwrap();
        assert_eq!(constrain_count, 3);

        //TODO: assert that abi is equal to:
        //   {
        //     "inputs": [
        //       {
        //         "name": "N",
        //         "public": true,
        //         "type": "field"
        //       }
        //     ],
        //     "output": [
        //       {
        //         "type": "bool"
        //       }
        //     ]
        //   }
    }

    #[test]
    fn test_wrong_compilation() {
        let code = r#"
            def main(field N):
                return N == 1
        "#;
        let code_path = PathBuf::from("/test");
        let arena = Arena::new();

        let compilation = api_compile::<Bn128Field>(code, &code_path, &arena);
        assert!(compilation.is_err());

        //TODO: assert that error types are the same
    }
}
