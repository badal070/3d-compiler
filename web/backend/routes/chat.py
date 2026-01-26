from fastapi import APIRouter, WebSocket, WebSocketDisconnect
from services.llm_service import LLMService
from services.scene_service import SceneService
import json

router = APIRouter()
llm_service = LLMService()
scene_service = SceneService()

class ChatMessage(BaseModel):
    message: str
    scene_context: dict | None = None

@router.websocket("/ws")
async def chat_websocket(websocket: WebSocket):
    """WebSocket endpoint for real-time chat"""
    await websocket.accept()
    
    try:
        while True:
            # Receive user message
            data = await websocket.receive_text()
            user_input = json.loads(data)
            
            # Get current scene context
            scene_context = scene_service.get_current_scene()
            
            # Process with LLM
            response = await llm_service.process_message(
                user_input["message"],
                scene_context
            )
            
            # Send response back
            await websocket.send_json({
                "type": "response",
                "content": response["text"],
                "dsl": response.get("dsl"),
                "explanation": response.get("explanation")
            })
            
            # If DSL was generated, compile it
            if response.get("dsl"):
                compilation = compiler.compile(response["dsl"])
                await websocket.send_json({
                    "type": "scene_update",
                    "ir_scene": compilation
                })
                
    except WebSocketDisconnect:
        print("Client disconnected")

@router.post("/message")
async def chat_message(request: ChatMessage):
    """HTTP endpoint for chat (alternative to WebSocket)"""
    response = await llm_service.process_message(
        request.message,
        request.scene_context
    )
    return response