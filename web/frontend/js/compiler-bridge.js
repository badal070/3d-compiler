export class CompilerBridge {
    constructor(wsUrl) {
        this.wsUrl = wsUrl;
        this.ws = null;
        this.messageHandlers = [];
        this.errorHandlers = [];
    }
    
    async connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.wsUrl);
            
            this.ws.onopen = () => {
                console.log('WebSocket connected');
                resolve();
            };
            
            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.errorHandlers.forEach(handler => handler(error));
                reject(error);
            };
            
            this.ws.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    this.messageHandlers.forEach(handler => handler(data));
                } catch (error) {
                    console.error('Message parse error:', error);
                }
            };
            
            this.ws.onclose = () => {
                console.log('WebSocket disconnected');
                // Attempt reconnect after 3 seconds
                setTimeout(() => this.connect(), 3000);
            };
        });
    }
    
    async sendMessage(data) {
        if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
            throw new Error('WebSocket not connected');
        }
        
        this.ws.send(JSON.stringify(data));
    }
    
    onMessage(handler) {
        this.messageHandlers.push(handler);
    }
    
    onError(handler) {
        this.errorHandlers.push(handler);
    }
    
    disconnect() {
        if (this.ws) {
            this.ws.close();
        }
    }
}