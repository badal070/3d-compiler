from fastapi import APIRouter
from pydantic import BaseModel
from backend.services.compiler_service import CompilerService

router = APIRouter()
compiler = CompilerService()

class CompileRequest(BaseModel):
    dsl_source: str
    optimize: bool = True

class CompileResponse(BaseModel):
    success: bool
    ir_scene: dict | None = None
    errors: list[str] = []

@router.post("/", response_model=CompileResponse)
async def compile_dsl(request: CompileRequest):
    """Compile DSL source to IR"""
    try:
        if not request.dsl_source.strip():
            return CompileResponse(
                success=False,
                errors=["DSL source is empty"]
            )
        
        result = compiler.compile(request.dsl_source, request.optimize)
        return CompileResponse(
            success=True,
            ir_scene=result
        )
    except Exception as e:
        error_msg = str(e)
        print(f"❌ Compilation error: {error_msg}")
        return CompileResponse(
            success=False,
            errors=[error_msg]
        )

@router.post("/validate", response_model=dict)
async def validate_dsl(request: CompileRequest):
    """Validate DSL without full compilation"""
    try:
        errors = compiler.validate_only(request.dsl_source)
        return {
            "valid": len(errors) == 0,
            "errors": errors
        }
    except Exception as e:
        return {
            "valid": False,
            "errors": [str(e)]
        }

@router.get("/health", response_model=dict)
async def compiler_health():
    """Check compiler status"""
    status = {
        "status": "ok",
        "compiler_path": str(compiler.compiler_path)
    }
    
    return status