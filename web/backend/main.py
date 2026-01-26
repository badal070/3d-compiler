from fastapi import FastAPI, WebSocket
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
import uvicorn

app = FastAPI(title="3D Compiler API")

# CORS for local development
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Mount static files (frontend)
app.mount("/static", StaticFiles(directory="frontend"), name="static")

# Include routers
from routes import compile, chat, scene
app.include_router(compile.router, prefix="/api/compile", tags=["compile"])
app.include_router(chat.router, prefix="/api/chat", tags=["chat"])
app.include_router(scene.router, prefix="/api/scene", tags=["scene"])

@app.get("/")
async def root():
    return {"message": "3D Compiler API - Ready"}

if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)