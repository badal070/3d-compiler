"""
Chains - Kinematic chain evaluation.
"""

from dataclasses import dataclass
from typing import List
from mathlib.kinematics.joints import Joint
from mathlib.kinematics.frames import Frame
from mathlib.core.scalar import Scalar
from mathlib.geometry.point import Point


@dataclass
class KinematicChain:
    """Evaluable kinematic chain."""
    
    frames: List[Frame]
    joints: List[Joint]
    
    def __post_init__(self):
        if len(self.joints) != len(self.frames) - 1:
            raise ValueError("Must have n-1 joints for n frames")
    
    def forward_kinematics(self, parameters: List[Scalar]) -> Point:
        """Compute end effector position given joint parameters."""
        if len(parameters) != len(self.joints):
            raise ValueError(f"Expected {len(self.joints)} parameters")
        
        # Start at base frame origin
        point = Point.origin(3)
        
        # Apply each joint transform
        for joint, param, frame in zip(self.joints, parameters, self.frames[1:]):
            transform = joint.get_transform(param)
            
            if hasattr(transform, 'apply_to_point'):
                point = transform.apply_to_point(point)
        
        return point