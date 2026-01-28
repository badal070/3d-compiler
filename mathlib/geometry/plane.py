"""
Plane - Infinite plane in 3D space.

Defined by point and normal vector.
"""

from dataclasses import dataclass
from mathlib.geometry.point import Point
from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.errors.math_errors import ZeroVectorError, DimensionError, InvalidOperationError


@dataclass(frozen=True)
class Plane:
    """Immutable plane in 3D."""
    
    point: Point
    normal: Vector
    
    def __init__(self, point: Point, normal: Vector):
        """
        Create a plane.
        
        Args:
            point: A point on the plane
            normal: Normal vector (perpendicular to plane)
        """
        if not isinstance(point, Point):
            raise TypeError(f"point must be Point, got {type(point)}")
        if not isinstance(normal, Vector):
            raise TypeError(f"normal must be Vector, got {type(normal)}")
        
        if point.dimension != 3:
            raise InvalidOperationError(
                "plane creation",
                "planes are only defined in 3D space"
            )
        
        if normal.dimension != 3:
            raise DimensionError(3, normal.dimension, "plane normal")
        
        if normal.is_zero():
            raise ZeroVectorError("Plane normal cannot be zero vector")
        
        # Store normalized normal
        normalized_normal = normal.normalize()
        
        object.__setattr__(self, 'point', point)
        object.__setattr__(self, 'normal', normalized_normal)
    
    def __str__(self) -> str:
        return f"Plane(point={self.point}, normal={self.normal})"
    
    def __repr__(self) -> str:
        return self.__str__()
    
    def signed_distance_to_point(self, point: Point) -> Scalar:
        """
        Compute signed distance from plane to point.
        
        Positive if point is on the side of the normal,
        negative otherwise.
        """
        if not isinstance(point, Point):
            raise TypeError(f"Expected Point, got {type(point)}")
        
        if point.dimension != 3:
            raise DimensionError(3, point.dimension, "plane distance")
        
        # Distance = (point - plane.point) · normal
        diff = point.position - self.point.position
        return diff.dot(self.normal)
    
    def distance_to_point(self, point: Point) -> Scalar:
        """Compute absolute distance from plane to point."""
        signed_dist = self.signed_distance_to_point(point)
        return Scalar(abs(signed_dist.value), signed_dist.unit)
    
    def contains_point(self, point: Point, tolerance: float = 1e-9) -> bool:
        """Check if point lies on the plane."""
        dist = self.signed_distance_to_point(point)
        return abs(dist.value) < tolerance
    
    def project_point(self, point: Point) -> Point:
        """Project point onto plane."""
        if not isinstance(point, Point):
            raise TypeError(f"Expected Point, got {type(point)}")
        
        signed_dist = self.signed_distance_to_point(point)
        
        # Projected point = point - distance * normal
        offset = self.normal * signed_dist.value
        projected_pos = point.position - offset
        
        return Point(list(projected_pos.components), projected_pos.unit)
    
    def is_parallel(self, other: 'Plane', tolerance: float = 1e-9) -> bool:
        """Check if planes are parallel."""
        if not isinstance(other, Plane):
            raise TypeError(f"Cannot check parallelism with {type(other)}")
        
        # Planes are parallel if normals are parallel
        cross = self.normal.cross(other.normal)
        return cross.is_zero(tolerance)
    
    def is_coincident(self, other: 'Plane', tolerance: float = 1e-9) -> bool:
        """Check if planes are coincident (same plane)."""
        if not self.is_parallel(other, tolerance):
            return False
        
        # Check if other.point is on this plane
        return self.contains_point(other.point, tolerance)
    
    @staticmethod
    def from_three_points(p1: Point, p2: Point, p3: Point) -> 'Plane':
        """Create plane passing through three non-collinear points."""
        if not all(isinstance(p, Point) for p in [p1, p2, p3]):
            raise TypeError("All arguments must be Points")
        
        if not all(p.dimension == 3 for p in [p1, p2, p3]):
            raise InvalidOperationError(
                "plane from three points",
                "all points must be in 3D space"
            )
        
        # Compute two edge vectors
        v1 = p2.position - p1.position
        v2 = p3.position - p1.position
        
        # Normal is cross product
        normal = v1.cross(v2)
        
        if normal.is_zero():
            raise InvalidOperationError(
                "plane from three points",
                "points are collinear"
            )
        
        return Plane(p1, normal)
    
    @staticmethod
    def from_point_and_vectors(point: Point, v1: Vector, v2: Vector) -> 'Plane':
        """Create plane from point and two spanning vectors."""
        if not isinstance(point, Point):
            raise TypeError(f"point must be Point, got {type(point)}")
        if not isinstance(v1, Vector) or not isinstance(v2, Vector):
            raise TypeError("v1 and v2 must be Vectors")
        
        if point.dimension != 3 or v1.dimension != 3 or v2.dimension != 3:
            raise InvalidOperationError(
                "plane from point and vectors",
                "all inputs must be in 3D space"
            )
        
        normal = v1.cross(v2)
        
        if normal.is_zero():
            raise InvalidOperationError(
                "plane from point and vectors",
                "vectors are parallel"
            )
        
        return Plane(point, normal)