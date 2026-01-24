"""
Frames - Reference frame graph for kinematics.

Frames are graph nodes. No hidden global frame. Ever.
"""

from dataclasses import dataclass, field
from typing import Optional, Dict
from mathlib.transforms.affine import AffineTransform
from mathlib.geometry.point import Point
from mathlib.errors.validation_errors import MissingFrameError, CircularDependencyError


@dataclass
class Frame:
    """Mutable reference frame in kinematic tree."""
    
    name: str
    parent: Optional['Frame'] = None
    transform: Optional[AffineTransform] = None
    children: Dict[str, 'Frame'] = field(default_factory=dict)
    
    def __post_init__(self):
        if self.parent is not None:
            self.parent.children[self.name] = self
    
    def transform_to_parent(self, point: Point) -> Point:
        """Transform point from this frame to parent frame."""
        if self.parent is None:
            return point
        
        if self.transform is None:
            return point
        
        return self.transform.apply_to_point(point)
    
    def transform_to_world(self, point: Point) -> Point:
        """Transform point from this frame to world frame."""
        current_point = point
        current_frame = self
        
        # Detect cycles
        visited = set()
        
        while current_frame.parent is not None:
            if current_frame.name in visited:
                raise CircularDependencyError(list(visited))
            visited.add(current_frame.name)
            
            current_point = current_frame.transform_to_parent(current_point)
            current_frame = current_frame.parent
        
        return current_point
    
    def find_frame(self, name: str) -> Optional['Frame']:
        """Find frame by name in tree."""
        if self.name == name:
            return self
        
        for child in self.children.values():
            result = child.find_frame(name)
            if result:
                return result
        
        return None