class CompilerService:
    def __init__(self, allow_mock: bool = False):
        self.compiler_path = RUST_COMPILER
        self.use_mock = False

        if not self.compiler_path.exists():
            if allow_mock:
                self.use_mock = True
                print("⚠️  Rust compiler binary not found")
                print("    Using mock compiler for testing")
            else:
                raise RuntimeError(
                    "Rust compiler binary not found.\n"
                    "Build it with:\n"
                    "  cargo build --release -p dsl-compiler"
                )
        else:
            print(f"✓ Rust compiler found: {self.compiler_path}")
