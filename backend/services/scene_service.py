class SceneService:
    def __init__(self):
        self.current_scene = None
    
    def get_current_scene(self):
        """Get the current scene context"""
        return self.current_scene
    
    def set_scene(self, scene_data):
        """Set the current scene"""
        self.current_scene = scene_data
    
    def reset_scene(self):
        """Reset the current scene"""
        self.current_scene = None