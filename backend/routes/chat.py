from fastapi import APIRouter, WebSocket, WebSocketDisconnect
from pydantic import BaseModel
from backend.services.compiler_service import CompilerService
import json

router = APIRouter()
compiler_service = CompilerService()

@router.websocket("/ws")
async def scene_websocket(websocket: WebSocket):
    """WebSocket endpoint for scene updates"""
    await websocket.accept()
    
    try:
        while True:
            data = await websocket.receive_text()
            message = json.loads(data)
            
            # Handle scene requests without LLM
            if message.get("type") == "compile":
                dsl = message.get("dsl")
                result = compiler_service.compile(dsl)
                await websocket.send_json({
                    "type": "scene_update",
                    "ir_scene": result
                })
                
    except WebSocketDisconnect:
        print("Client disconnected")