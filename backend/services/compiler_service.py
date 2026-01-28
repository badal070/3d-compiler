import subprocess
import json
import tempfile
import os
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
RUST_COMPILER = ROOT / "target" / "release" / "dsl-compiler"

class CompilerService:
    def __init__(self):
        self.compiler_path = RUST_COMPILER
        if not self.compiler_path.exists():
            raise FileNotFoundError(
                f"DSL compiler not found at {self.compiler_path}. "
                f"Please build it with: cargo build --release -p dsl-compiler"
            )
    
    def compile(self, dsl_source: str, optimize: bool = True) -> dict:
        """Compile DSL to IR"""
        return self._real_compile(dsl_source, optimize)
    
    def _real_compile(self, dsl_source: str, optimize: bool = True) -> dict:
        """Compile using Rust binary"""
        
        with tempfile.NamedTemporaryFile(
            mode='w', 
            suffix='.dsl', 
            delete=False
        ) as f:
            f.write(dsl_source)
            temp_path = f.name
        
        try:
            # Build command
            cmd = [str(self.compiler_path), temp_path, '--json']
            if optimize:
                cmd.append('--optimize')
            
            print(f"Running: {' '.join(cmd)}")
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode != 0:
                error_msg = result.stderr or result.stdout
                print(f"Compiler error: {error_msg}")
                raise Exception(f"Compilation failed: {error_msg}")
            
            try:
                ir_scene = json.loads(result.stdout)
                print(f"✓ Successfully compiled scene with {len(ir_scene.get('entities', {}))} entities")
                return ir_scene
            except json.JSONDecodeError as e:
                print(f"JSON parse error from compiler output:")
                print(f"stdout: {result.stdout}")
                print(f"stderr: {result.stderr}")
                raise Exception(f"Invalid compiler output: {str(e)}")
            
        except subprocess.TimeoutExpired:
            raise Exception("Compilation timeout (30s)")
        except Exception as e:
            raise Exception(f"Compilation error: {str(e)}")
        finally:
            try:
                os.unlink(temp_path)
            except:
                pass
    

    def validate_only(self, dsl_source: str) -> list[str]:
        """Validate DSL"""
        with tempfile.NamedTemporaryFile(
            mode='w',
            suffix='.dsl',
            delete=False
        ) as f:
            f.write(dsl_source)
            temp_path = f.name
        
        try:
            result = subprocess.run(
                [str(self.compiler_path), temp_path, '--validate-only'],
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode != 0:
                return [result.stderr or "Validation failed"]
            
            return []
        except Exception as e:
            return [str(e)]
        finally:
            try:
                os.unlink(temp_path)
            except:
                pass
