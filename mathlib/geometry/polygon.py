"""
Polygon - Planar polygon in 2D or 3D space.
"""

from dataclasses import dataclass
from typing import List
from mathlib.geometry.point import Point
from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.errors.math_errors import InvalidOperationError


@dataclass(frozen=True)
class Polygon:
    """Immutable polygon defined by vertices."""
    
    vertices: tuple
    
    def __init__(self, vertices: List[Point]):
        if len(vertices) < 3:
            raise InvalidOperationError(
                "polygon creation",
                "polygon must have at least 3 vertices"
            )
        
        # Verify all points have same dimension
        dim = vertices[0].dimension
        if not all(v.dimension == dim for v in vertices):
            raise InvalidOperationError(
                "polygon creation",
                "all vertices must have same dimension"
            )
        
        object.__setattr__(self, 'vertices', tuple(vertices))
    
    @property
    def num_vertices(self) -> int:
        return len(self.vertices)
    
    @property
    def dimension(self) -> int:
        return self.vertices[0].dimension
    
    def edge_vector(self, index: int) -> Vector:
        """Get edge vector from vertex[i] to vertex[i+1]."""
        next_idx = (index + 1) % self.num_vertices
        return self.vertices[next_idx].position - self.vertices[index].position
    
    def perimeter(self) -> Scalar:
        """Calculate polygon perimeter."""
        total = 0.0
        unit = self.vertices[0].position.unit
        
        for i in range(self.num_vertices):
            edge = self.edge_vector(i)
            total += edge.norm().value
        
        return Scalar(total, unit)
    
    def centroid(self) -> Point:
        """Calculate polygon centroid (center of mass)."""
        sum_vec = Vector.zero(self.dimension, self.vertices[0].position.unit)
        
        for vertex in self.vertices:
            sum_vec = sum_vec + vertex.position
        
        avg_vec = sum_vec / float(self.num_vertices)
        return Point(list(avg_vec.components), avg_vec.unit)