from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from services.compiler_service import CompilerService

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
        result = compiler.compile(request.dsl_source, request.optimize)
        return CompileResponse(
            success=True,
            ir_scene=result
        )
    except Exception as e:
        return CompileResponse(
            success=False,
            errors=[str(e)]
        )

@router.post("/validate")
async def validate_dsl(request: CompileRequest):
    """Validate DSL without full compilation"""
    try:
        errors = compiler.validate_only(request.dsl_source)
        return {"valid": len(errors) == 0, "errors": errors}
    except Exception as e:
        return {"valid": False, "errors": [str(e)]}