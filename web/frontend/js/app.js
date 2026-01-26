import { ThreeRenderer } from './renderer.js';
import { ChatInterface } from './chat.js';
import { CompilerBridge } from './compiler-bridge.js';
import { AppState } from './state.js';

class App {
    constructor() {
        this.state = new AppState();
        this.renderer = new ThreeRenderer('renderCanvas');
        this.chat = new ChatInterface('chatContainer');
        this.bridge = new CompilerBridge('ws://localhost:8000/api/chat/ws');
        
        this.setupEventListeners();
        this.connectWebSocket();
    }
    
    setupEventListeners() {
        // Send button
        document.getElementById('sendBtn').addEventListener('click', () => {
            this.handleUserInput();
        });
        
        // Enter key in textarea
        document.getElementById('userInput').addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                this.handleUserInput();
            }
        });
        
        // Play/Pause
        document.getElementById('playBtn').addEventListener('click', () => {
            this.togglePlayback();
        });
        
        // Reset
        document.getElementById('resetBtn').addEventListener('click', () => {
            this.resetScene();
        });
        
        // Camera reset
        document.getElementById('cameraBtn').addEventListener('click', () => {
            this.renderer.resetCamera();
        });
        
        // Example suggestions
        document.querySelectorAll('.examples li').forEach(li => {
            li.addEventListener('click', () => {
                document.getElementById('userInput').value = li.textContent.trim();
                this.handleUserInput();
            });
        });
    }
    
    async connectWebSocket() {
        try {
            await this.bridge.connect();
            this.updateStatus('Connected', 'success');
            
            // Listen for messages
            this.bridge.onMessage((data) => {
                this.handleServerMessage(data);
            });
            
            this.bridge.onError((error) => {
                this.updateStatus('Connection error', 'error');
                console.error('WebSocket error:', error);
            });
            
        } catch (error) {
            this.updateStatus('Failed to connect', 'error');
            console.error('Connection error:', error);
        }
    }
    
    async handleUserInput() {
        const input = document.getElementById('userInput');
        const message = input.value.trim();
        
        if (!message) return;
        
        // Clear input
        input.value = '';
        
        // Add user message to chat
        this.chat.addMessage('user', message);
        
        // Update status
        this.updateStatus('Processing...', 'loading');
        
        // Send to server
        try {
            await this.bridge.sendMessage({
                message: message,
                scene_context: this.state.getCurrentScene()
            });
        } catch (error) {
            this.chat.addMessage('error', 'Failed to send message: ' + error.message);
            this.updateStatus('Error', 'error');
        }
    }
    
    handleServerMessage(data) {
        switch (data.type) {
            case 'response':
                // Add assistant response
                this.chat.addMessage('assistant', data.content, data.dsl);
                this.updateStatus('Ready', 'success');
                break;
                
            case 'scene_update':
                // Update 3D scene
                this.showLoading(true);
                this.renderer.loadScene(data.ir_scene)
                    .then(() => {
                        this.state.setScene(data.ir_scene);
                        this.enableControls(true);
                        this.showLoading(false);
                        this.updateStatus('Scene loaded', 'success');
                    })
                    .catch(error => {
                        console.error('Scene load error:', error);
                        this.showLoading(false);
                        this.updateStatus('Scene load failed', 'error');
                    });
                break;
                
            case 'error':
                this.chat.addMessage('error', data.message);
                this.updateStatus('Error', 'error');
                break;
        }
    }
    
    togglePlayback() {
        const isPlaying = this.state.togglePlayback();
        const btn = document.getElementById('playBtn');
        btn.innerHTML = isPlaying ? 
            '<span class="icon">⏸</span> Pause' :
            '<span class="icon">▶</span> Play';
    }
    
    resetScene() {
        this.renderer.reset();
        this.state.reset();
        this.updateStatus('Scene reset', 'success');
    }
    
    updateStatus(text, type = 'info') {
        const statusText = document.getElementById('status');
        const statusIcon = document.getElementById('statusIcon');
        
        statusText.textContent = text;
        
        const colors = {
            success: 'var(--success)',
            error: 'var(--error)',
            loading: 'var(--primary)',
            info: 'var(--text-dim)'
        };
        
        statusIcon.style.color = colors[type];
    }
    
    showLoading(show) {
        const overlay = document.getElementById('loadingOverlay');
        overlay.classList.toggle('hidden', !show);
    }
    
    enableControls(enabled) {
        document.getElementById('playBtn').disabled = !enabled;
        document.getElementById('resetBtn').disabled = !enabled;
    }
}

// Start app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.app = new App();
});