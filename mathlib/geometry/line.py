"""
Line - Infinite line in space.

Defined by point and direction. No "almost parallel" nonsense.
"""

from dataclasses import dataclass
from mathlib.geometry.point import Point
from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.errors.math_errors import ZeroVectorError, DimensionError


@dataclass(frozen=True)
class Line:
    """Immutable infinite line."""
    
    point: Point
    direction: Vector
    
    def __init__(self, point: Point, direction: Vector):
        """
        Create a line.
        
        Args:
            point: A point on the line
            direction: Direction vector (will be normalized)
        """
        if not isinstance(point, Point):
            raise TypeError(f"point must be Point, got {type(point)}")
        if not isinstance(direction, Vector):
            raise TypeError(f"direction must be Vector, got {type(direction)}")
        
        if direction.is_zero():
            raise ZeroVectorError("Line direction cannot be zero vector")
        
        if point.dimension != direction.dimension:
            raise DimensionError(
                point.dimension,
                direction.dimension,
                "line initialization"
            )
        
        # Store normalized direction
        normalized_dir = direction.normalize()
        
        object.__setattr__(self, 'point', point)
        object.__setattr__(self, 'direction', normalized_dir)
    
    @property
    def dimension(self) -> int:
        """Get line dimension."""
        return self.point.dimension
    
    def __str__(self) -> str:
        return f"Line(point={self.point}, dir={self.direction})"
    
    def __repr__(self) -> str:
        return self.__str__()
    
    def point_at(self, t: float) -> Point:
        """Get point on line at parameter t."""
        # P(t) = point + t * direction
        offset = self.direction * t
        new_pos = self.point.position + offset
        return Point(list(new_pos.components), new_pos.unit)
    
    def is_parallel(self, other: 'Line', tolerance: float = 1e-9) -> bool:
        """Check if lines are parallel."""
        if not isinstance(other, Line):
            raise TypeError(f"Cannot check parallelism with {type(other)}")
        
        if self.dimension != other.dimension:
            raise DimensionError(
                self.dimension,
                other.dimension,
                "parallel check"
            )
        
        # Lines are parallel if directions are parallel
        # Check if cross product is zero (3D) or determinant is zero (2D)
        if self.dimension == 3:
            cross = self.direction.cross(other.direction)
            return cross.is_zero(tolerance)
        elif self.dimension == 2:
            # For 2D: det([d1, d2]) = d1.x * d2.y - d1.y * d2.x
            det = (self.direction[0] * other.direction[1] - 
                   self.direction[1] * other.direction[0])
            return abs(det) < tolerance
        else:
            # General case: directions are scalar multiples
            # Find first non-zero component
            ratio = None
            for i in range(self.dimension):
                if abs(self.direction[i]) > tolerance:
                    if abs(other.direction[i]) < tolerance:
                        return False
                    current_ratio = other.direction[i] / self.direction[i]
                    if ratio is None:
                        ratio = current_ratio
                    elif abs(ratio - current_ratio) > tolerance:
                        return False
            return True
    
    def is_coincident(self, other: 'Line', tolerance: float = 1e-9) -> bool:
        """Check if lines are coincident (same line)."""
        if not self.is_parallel(other, tolerance):
            return False
        
        # Check if other.point is on this line
        # Point is on line if (point - self.point) × direction = 0
        diff = other.point.position - self.point.position
        
        if self.dimension == 3:
            cross = diff.cross(self.direction)
            return cross.is_zero(tolerance)
        else:
            # General case: check if diff is parallel to direction
            if diff.is_zero(tolerance):
                return True
            
            # Find first non-zero component in direction
            for i in range(self.dimension):
                if abs(self.direction[i]) > tolerance:
                    if abs(diff[i]) < tolerance:
                        return False
                    ratio = diff[i] / self.direction[i]
                    # Check if all components have same ratio
                    for j in range(self.dimension):
                        if abs(self.direction[j]) > tolerance:
                            expected = ratio * self.direction[j]
                            if abs(diff[j] - expected) > tolerance:
                                return False
                    return True
            return False
    
    def distance_to_point(self, point: Point) -> Scalar:
        """Compute shortest distance from line to point."""
        if not isinstance(point, Point):
            raise TypeError(f"Expected Point, got {type(point)}")
        
        if self.dimension != point.dimension:
            raise DimensionError(
                self.dimension,
                point.dimension,
                "distance computation"
            )
        
        # Distance = ||(point - line.point) - projection||
        diff = point.position - self.point.position
        projection_length = diff.dot(self.direction).value
        projection = self.direction * projection_length
        perpendicular = diff - projection
        
        return perpendicular.norm()
    
    @staticmethod
    def from_two_points(p1: Point, p2: Point) -> 'Line':
        """Create line passing through two points."""
        if not isinstance(p1, Point) or not isinstance(p2, Point):
            raise TypeError("Both arguments must be Points")
        
        direction = p2.position - p1.position
        return Line(p1, direction)