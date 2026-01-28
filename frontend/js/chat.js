export class ChatInterface {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
    }
    
    addMessage(role, content, dsl = null) {
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${role}`;
        
        const bubble = document.createElement('div');
        bubble.className = 'message-bubble';
        
        const text = document.createElement('div');
        text.className = 'message-text';
        text.textContent = content;
        bubble.appendChild(text);
        
        // Add DSL code block if provided
        if (dsl) {
            const codeBlock = this.createCodeBlock(dsl);
            bubble.appendChild(codeBlock);
        }
        
        // Add timestamp
        const timestamp = document.createElement('div');
        timestamp.className = 'message-timestamp';
        timestamp.textContent = new Date().toLocaleTimeString();
        bubble.appendChild(timestamp);
        
        messageDiv.appendChild(bubble);
        this.container.appendChild(messageDiv);
        
        // Scroll to bottom
        this.container.scrollTop = this.container.scrollHeight;
    }
    
    createCodeBlock(dsl) {
        const codeBlock = document.createElement('div');
        codeBlock.className = 'dsl-code-block';
        
        const header = document.createElement('div');
        header.className = 'code-header';
        header.innerHTML = `
            <span>DSL Code</span>
            <button class="copy-btn">Copy</button>
        `;
        
        const content = document.createElement('pre');
        content.className = 'code-content';
        content.textContent = dsl;
        
        codeBlock.appendChild(header);
        codeBlock.appendChild(content);
        
        // Copy functionality
        header.querySelector('.copy-btn').addEventListener('click', () => {
            navigator.clipboard.writeText(dsl);
            const btn = header.querySelector('.copy-btn');
            btn.textContent = 'Copied!';
            setTimeout(() => btn.textContent = 'Copy', 2000);
        });
        
        return codeBlock;
    }
    
    clear() {
        this.container.innerHTML = '';
    }
}