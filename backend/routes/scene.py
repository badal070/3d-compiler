from fastapi import APIRouter

router = APIRouter()

@router.get("/")
async def get_scene():
    """Get current scene info"""
    return {"status": "Scene API ready"}