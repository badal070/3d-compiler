"""
Joints - Kinematic joint constraints.
"""

from dataclasses import dataclass
from abc import ABC, abstractmethod
from mathlib.core.scalar import Scalar
from mathlib.transforms.rotation import Rotation
from mathlib.transforms.translation import Translation
from mathlib.core.vector import Vector


class Joint(ABC):
    """Abstract base class for joints."""
    
    @abstractmethod
    def get_transform(self, parameter: Scalar):
        """Get transform for given joint parameter."""
        pass


@dataclass(frozen=True)
class RevoluteJoint(Joint):
    """Revolute (rotational) joint."""
    
    axis: str  # "x", "y", or "z"
    
    def get_transform(self, angle: Scalar):
        """Get rotation transform for given angle."""
        return Rotation(self.axis, angle)


@dataclass(frozen=True)
class PrismaticJoint(Joint):
    """Prismatic (sliding) joint."""
    
    axis: Vector  # Direction of motion
    
    def get_transform(self, distance: Scalar):
        """Get translation transform for given distance."""
        offset = self.axis * distance.value
        return Translation(offset)