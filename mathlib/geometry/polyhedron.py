"""
Polyhedron - 3D solid bounded by polygons.
"""

from dataclasses import dataclass
from typing import List
from mathlib.geometry.polygon import Polygon
from mathlib.geometry.point import Point
from mathlib.errors.math_errors import InvalidOperationError


@dataclass(frozen=True)
class Polyhedron:
    """Immutable polyhedron defined by faces."""
    
    faces: tuple
    
    def __init__(self, faces: List[Polygon]):
        if len(faces) < 4:
            raise InvalidOperationError(
                "polyhedron creation",
                "polyhedron must have at least 4 faces"
            )
        
        # Verify all faces are 3D
        for face in faces:
            if face.dimension != 3:
                raise InvalidOperationError(
                    "polyhedron creation",
                    "all faces must be in 3D space"
                )
        
        object.__setattr__(self, 'faces', tuple(faces))
    
    @property
    def num_faces(self) -> int:
        return len(self.faces)
    
    def vertices(self) -> List[Point]:
        """Get all unique vertices."""
        vertex_set = []
        for face in self.faces:
            for v in face.vertices:
                if not any(v == existing for existing in vertex_set):
                    vertex_set.append(v)
        return vertex_set