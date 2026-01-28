"""
Point - Geometric point in space.

Represents position, not direction.
"""

from dataclasses import dataclass
from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.core.units import Unit, UNITLESS
from mathlib.errors.math_errors import DimensionError
import math


@dataclass(frozen=True)
class Point:
    """Immutable geometric point."""
    
    position: Vector
    
    def __init__(self, coordinates: list, unit: Unit = UNITLESS):
        """
        Create a point.
        
        Args:
            coordinates: List of coordinate values
            unit: Unit for coordinates
        """
        pos = Vector(coordinates, unit=unit)
        object.__setattr__(self, 'position', pos)
    
    @property
    def dimension(self) -> int:
        """Get point dimension."""
        return self.position.dimension
    
    @property
    def x(self) -> float:
        """Get x coordinate."""
        return self.position[0]
    
    @property
    def y(self) -> float:
        """Get y coordinate if 2D or 3D."""
        if self.dimension < 2:
            raise AttributeError("Point does not have y coordinate")
        return self.position[1]
    
    @property
    def z(self) -> float:
        """Get z coordinate if 3D."""
        if self.dimension < 3:
            raise AttributeError("Point does not have z coordinate")
        return self.position[2]
    
    def __str__(self) -> str:
        coords = ", ".join(f"{c:.6g}" for c in self.position.components)
        return f"Point({coords})"
    
    def __repr__(self) -> str:
        return f"Point({list(self.position.components)})"
    
    def __eq__(self, other) -> bool:
        if not isinstance(other, Point):
            return False
        return self.position == other.position
    
    def distance_to(self, other: 'Point') -> Scalar:
        """Compute distance to another point."""
        if not isinstance(other, Point):
            raise TypeError(f"Cannot compute distance to {type(other)}")
        
        if self.dimension != other.dimension:
            raise DimensionError(
                self.dimension,
                other.dimension,
                "distance computation"
            )
        
        diff = other.position - self.position
        return diff.norm()
    
    def midpoint(self, other: 'Point') -> 'Point':
        """Compute midpoint between two points."""
        if not isinstance(other, Point):
            raise TypeError(f"Cannot compute midpoint with {type(other)}")
        
        if self.dimension != other.dimension:
            raise DimensionError(
                self.dimension,
                other.dimension,
                "midpoint computation"
            )
        
        mid_pos = (self.position + other.position) / 2.0
        return Point(list(mid_pos.components), mid_pos.unit)
    
    def translate(self, vector: Vector) -> 'Point':
        """Translate point by vector."""
        if not isinstance(vector, Vector):
            raise TypeError(f"Cannot translate by {type(vector)}")
        
        new_pos = self.position + vector
        return Point(list(new_pos.components), new_pos.unit)
    
    @staticmethod
    def origin(dimension: int, unit: Unit = UNITLESS) -> 'Point':
        """Create origin point."""
        return Point([0.0] * dimension, unit)