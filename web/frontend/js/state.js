export class AppState {
    constructor() {
        this.currentScene = null;
        this.isPlaying = true;
        this.currentTime = 0;
    }
    
    setScene(sceneData) {
        this.currentScene = sceneData;
        this.currentTime = 0;
    }
    
    getCurrentScene() {
        return this.currentScene;
    }
    
    togglePlayback() {
        this.isPlaying = !this.isPlaying;
        return this.isPlaying;
    }
    
    reset() {
        this.currentTime = 0;
        this.isPlaying = true;
    }
    
    updateTime(delta) {
        if (this.isPlaying) {
            this.currentTime += delta;
        }
    }
}
```