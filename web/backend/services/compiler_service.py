import subprocess
import json
import tempfile
import os

class CompilerService:
    def __init__(self):
        # Path to compiled Rust binary or WASM module
        self.compiler_path = os.getenv(
            "COMPILER_PATH", 
            "../target/release/dsl_compiler"
        )
    
    def compile(self, dsl_source: str, optimize: bool = True) -> dict:
        """Compile DSL to IR using Rust compiler"""
        
        # Write DSL to temp file
        with tempfile.NamedTemporaryFile(
            mode='w', 
            suffix='.dsl', 
            delete=False
        ) as f:
            f.write(dsl_source)
            temp_path = f.name
        
        try:
            # Call Rust compiler
            result = subprocess.run(
                [self.compiler_path, temp_path, '--json'],
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode != 0:
                raise Exception(result.stderr)
            
            # Parse IR JSON
            ir_scene = json.loads(result.stdout)
            return ir_scene
            
        finally:
            os.unlink(temp_path)
    
    def validate_only(self, dsl_source: str) -> list[str]:
        """Validate DSL without compilation"""
        
        with tempfile.NamedTemporaryFile(
            mode='w',
            suffix='.dsl',
            delete=False
        ) as f:
            f.write(dsl_source)
            temp_path = f.name
        
        try:
            result = subprocess.run(
                [self.compiler_path, temp_path, '--validate'],
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode != 0:
                return [result.stderr]
            
            return []
            
        finally:
            os.unlink(temp_path)
