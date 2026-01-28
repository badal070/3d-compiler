"""
Translation - Pure translation transformation.

@ means compose, not multiply randomly.
"""

from dataclasses import dataclass
from mathlib.core.vector import Vector
from mathlib.geometry.point import Point
from mathlib.errors.math_errors import DimensionError


@dataclass(frozen=True)
class Translation:
    """Immutable translation transformation."""
    
    offset: Vector
    
    def __init__(self, offset: Vector):
        """
        Create a translation.
        
        Args:
            offset: Translation vector
        """
        if not isinstance(offset, Vector):
            raise TypeError(f"offset must be Vector, got {type(offset)}")
        
        object.__setattr__(self, 'offset', offset)
    
    @property
    def dimension(self) -> int:
        """Get translation dimension."""
        return self.offset.dimension
    
    def __str__(self) -> str:
        return f"Translation({self.offset})"
    
    def __repr__(self) -> str:
        return self.__str__()
    
    def apply_to_point(self, point: Point) -> Point:
        """Apply translation to a point."""
        if not isinstance(point, Point):
            raise TypeError(f"Expected Point, got {type(point)}")
        
        if point.dimension != self.dimension:
            raise DimensionError(
                self.dimension,
                point.dimension,
                "translation application"
            )
        
        return point.translate(self.offset)
    
    def apply_to_vector(self, vector: Vector) -> Vector:
        """
        Apply translation to a vector.
        
        Note: Translations don't affect free vectors (they represent direction),
        but this is included for API completeness. Returns vector unchanged.
        """
        if not isinstance(vector, Vector):
            raise TypeError(f"Expected Vector, got {type(vector)}")
        
        # Vectors represent direction, not position
        # Translation doesn't affect them
        return vector
    
    def inverse(self) -> 'Translation':
        """Get inverse translation."""
        return Translation(-self.offset)
    
    def __matmul__(self, other) -> 'Translation':
        """
        Compose translations using @ operator.
        
        Translation @ Translation = Combined translation
        """
        if isinstance(other, Translation):
            if self.dimension != other.dimension:
                raise DimensionError(
                    self.dimension,
                    other.dimension,
                    "translation composition"
                )
            # T1 @ T2 means apply T2 first, then T1
            combined_offset = self.offset + other.offset
            return Translation(combined_offset)
        
        raise TypeError(f"Cannot compose Translation with {type(other)}")
    
    @staticmethod
    def identity(dimension: int) -> 'Translation':
        """Create identity translation (zero offset)."""
        return Translation(Vector.zero(dimension))