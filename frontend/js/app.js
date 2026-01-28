import { ThreeRenderer } from './renderer.js';
import { AppState } from './state.js';

class App {
    constructor() {
        this.state = new AppState();
        this.renderer = new ThreeRenderer('renderCanvas');
        this.dslInput = document.getElementById('userInput');
        this.sendBtn = document.getElementById('sendBtn');
        this.compilerStatus = null;
        
        this.checkCompilerHealth();
        this.setupEventListeners();
        this.prefillSample();
    }

    async prefillSample() {
        try {
            const res = await fetch('/scene.dsl');
            if (!res.ok) return;
            const text = await res.text();
            if (this.dslInput && !this.dslInput.value.trim()) {
                this.dslInput.value = text;
            }
        } catch (e) {
            // ignore
        }
    }
    
    async checkCompilerHealth() {
        try {
            const response = await fetch('/api/compile/health');
            const data = await response.json();
            this.compilerStatus = data;
            
            this.updateStatus('Compiler Ready', 'success');
            console.log('Compiler status:', data);
        } catch (error) {
            console.error('Failed to check compiler:', error);
            this.updateStatus('Compiler connection failed', 'error');
        }
    }
    
    setupEventListeners() {
        // Compile button
        this.sendBtn.addEventListener('click', () => {
            this.handleCompile();
        });
        
        // Ctrl+Enter to compile
        this.dslInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && e.ctrlKey) {
                e.preventDefault();
                this.handleCompile();
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
    }
    
    async handleCompile() {
        const dslSource = this.dslInput.value.trim();
        
        if (!dslSource) {
            this.updateStatus('Please enter DSL code', 'error');
            return;
        }
        
        this.updateStatus('Compiling...', 'loading');
        this.showLoading(true);
        
        try {
            // First validate
            const validateResponse = await fetch('/api/compile/validate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ dsl_source: dslSource })
            });
            
            const validateResult = await validateResponse.json();
            
            if (!validateResult.valid) {
                const errors = validateResult.errors.join('; ');
                this.updateStatus(`Validation failed: ${errors}`, 'error');
                this.showLoading(false);
                return;
            }
            
            // Then compile
            const compileResponse = await fetch('/api/compile/', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    dsl_source: dslSource,
                    optimize: true
                })
            });
            
            const compileResult = await compileResponse.json();
            
            if (compileResult.success && compileResult.ir_scene) {
                console.log('✓ Compilation successful');
                console.log('IR Scene:', compileResult.ir_scene);
                
                // Load the compiled scene into renderer
                await this.renderer.loadScene(compileResult.ir_scene);
                this.state.setScene(compileResult.ir_scene);
                // start renderer playback to apply motions
                if (this.renderer && typeof this.renderer.setPlaying === 'function') {
                    this.renderer.setPlaying(this.state.isPlaying);
                }
                this.enableControls(true);
                
                const entityCount = Array.isArray(compileResult.ir_scene.entities)
                    ? compileResult.ir_scene.entities.length
                    : Object.keys(compileResult.ir_scene.entities || {}).length;
                
                this.updateStatus(
                    `Scene loaded: ${entityCount} objects`,
                    'success'
                );
            } else {
                const errors = compileResult.errors?.join('; ') || 'Unknown error';
                this.updateStatus(`Compilation failed: ${errors}`, 'error');
                console.error('Compilation errors:', compileResult.errors);
            }
            
        } catch (error) {
            this.updateStatus(`Error: ${error.message}`, 'error');
            console.error('Request error:', error);
        } finally {
            this.showLoading(false);
        }
    }
    
    togglePlayback() {
        const isPlaying = this.state.togglePlayback();
        const btn = document.getElementById('playBtn');
        btn.innerHTML = isPlaying ?
            '<span class="icon">⏸</span> Pause' :
            '<span class="icon">▶</span> Play';

        // Inform renderer about playback state
        if (this.renderer && typeof this.renderer.setPlaying === 'function') {
            this.renderer.setPlaying(isPlaying);
        }
    }
    
    resetScene() {
        this.renderer.reset();
        this.state.reset();
        this.enableControls(false);
        this.updateStatus('Scene reset', 'success');
    }
    
    updateStatus(text, type = 'info') {
        const statusText = document.getElementById('status');
        const statusIcon = document.getElementById('statusIcon');
        
        statusText.textContent = text;
        statusIcon.className = `status-icon status-${type}`;
    }
    
    showLoading(show) {
        document.getElementById('loadingOverlay').classList.toggle('hidden', !show);
    }
    
    enableControls(enable) {
        const playBtn = document.getElementById('playBtn');
        const resetBtn = document.getElementById('resetBtn');
        playBtn.disabled = !enable;
        resetBtn.disabled = !enable;
        if (enable) {
            // ensure play button label matches state
            playBtn.innerHTML = this.state.isPlaying ? '<span class="icon">⏸</span> Pause' : '<span class="icon">▶</span> Play';
        } else {
            playBtn.innerHTML = '<span class="icon">⏸</span> Pause';
        }
    }
}

// Initialize app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new App();
});