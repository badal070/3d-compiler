from anthropic import Anthropic
import os

class LLMService:
    def __init__(self):
        self.client = Anthropic(api_key=os.getenv("ANTHROPIC_API_KEY"))
        self.system_prompt = self._load_system_prompt()
    
    def _load_system_prompt(self):
        return """You are an expert in creating 3D visualizations for educational purposes.
        
Your task is to:
1. Understand the concept the user wants to visualize
2. Generate DSL code that represents the concept
3. Provide a clear explanation of what's happening in the scene

DSL Syntax Rules:
- Mandatory order: scene → library_imports → entities → constraints → motions → timelines
- All rotations in RADIANS
- All masses positive
- All vectors exactly 3 components [x, y, z]

Example DSL structure:
```
scene {
  name: "Example"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

library_imports {
  math: "core_mechanics"
  geometry: "basic_solids"
}

entity cube1 {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: cube
    }
  }
}
```

Always respond with:
1. Explanation in natural language
2. DSL code block
3. Key points about the visualization"""

    async def process_message(self, user_message: str, scene_context: dict = None):
        """Process user message and generate DSL + explanation"""
        
        messages = []
        
        # Add scene context if available
        if scene_context:
            messages.append({
                "role": "user",
                "content": f"Current scene context: {json.dumps(scene_context)}"
            })
        
        messages.append({
            "role": "user",
            "content": user_message
        })
        
        response = self.client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=2000,
            system=self.system_prompt,
            messages=messages
        )
        
        # Parse response
        content = response.content[0].text
        
        # Extract DSL code block
        dsl_code = self._extract_dsl(content)
        explanation = self._extract_explanation(content)
        
        return {
            "text": content,
            "dsl": dsl_code,
            "explanation": explanation
        }
    
    def _extract_dsl(self, content: str) -> str | None:
        """Extract DSL code from markdown code blocks"""
        import re
        match = re.search(r'```(?:dsl)?\n(.*?)\n```', content, re.DOTALL)
        return match.group(1) if match else None
    
    def _extract_explanation(self, content: str) -> str:
        """Extract explanation text (everything except code blocks)"""
        import re
        return re.sub(r'```.*?```', '', content, flags=re.DOTALL).strip()