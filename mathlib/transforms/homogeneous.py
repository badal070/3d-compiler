"""
Homogeneous - Homogeneous coordinates for unified transforms.
"""

from dataclasses import dataclass
from mathlib.core.matrix import Matrix
from mathlib.core.vector import Vector
from mathlib.geometry.point import Point
from mathlib.transforms.affine import AffineTransform


@dataclass(frozen=True)
class HomogeneousMatrix:
    """4x4 homogeneous transformation matrix."""
    
    matrix: Matrix
    
    def __init__(self, matrix: Matrix = None):
        if matrix is None:
            matrix = Matrix.identity(4)
        
        if matrix.shape != (4, 4):
            raise ValueError("Homogeneous matrix must be 4x4")
        
        object.__setattr__(self, 'matrix', matrix)
    
    def apply_to_point(self, point: Point) -> Point:
        """Apply transform to 3D point."""
        if point.dimension != 3:
            raise ValueError("Point must be 3D")
        
        # Convert to homogeneous coordinates
        homog = Vector([*point.position.components, 1.0])
        
        # Apply transformation
        result = self.matrix.apply(homog)
        
        # Convert back (perspective divide)
        w = result[3]
        if abs(w) < 1e-10:
            raise ValueError("Invalid homogeneous coordinate (w ≈ 0)")
        
        coords = [result[i] / w for i in range(3)]
        return Point(coords, point.position.unit)
    
    @staticmethod
    def from_affine(affine: AffineTransform) -> 'HomogeneousMatrix':
        """Convert affine transform to homogeneous matrix."""
        A = affine.linear
        b = affine.translation
        
        elements = []
        for i in range(3):
            row = [A[i, j] for j in range(3)] + [b[i]]
            elements.append(row)
        elements.append([0.0, 0.0, 0.0, 1.0])
        
        return HomogeneousMatrix(Matrix(elements))