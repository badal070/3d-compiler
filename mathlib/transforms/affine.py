"""
Affine - General affine transformation.

Combines linear transformation and translation.
"""

from dataclasses import dataclass
from mathlib.core.matrix import Matrix
from mathlib.core.vector import Vector
from mathlib.geometry.point import Point
from mathlib.transforms.translation import Translation
from mathlib.errors.math_errors import DimensionError


@dataclass(frozen=True)
class AffineTransform:
    """Immutable affine transformation: x' = Ax + b"""
    
    linear: Matrix  # A
    translation: Vector  # b
    
    def __init__(self, linear: Matrix, translation: Vector = None):
        if not isinstance(linear, Matrix):
            raise TypeError("linear must be Matrix")
        
        if translation is None:
            translation = Vector.zero(linear.cols)
        
        if not isinstance(translation, Vector):
            raise TypeError("translation must be Vector")
        
        if linear.cols != translation.dimension:
            raise DimensionError(
                linear.cols,
                translation.dimension,
                "affine transform initialization"
            )
        
        object.__setattr__(self, 'linear', linear)
        object.__setattr__(self, 'translation', translation)
    
    @property
    def dimension(self) -> int:
        return self.linear.cols
    
    def apply_to_point(self, point: Point) -> Point:
        """Apply affine transform to point."""
        # x' = Ax + b
        transformed = self.linear.apply(point.position) + self.translation
        return Point(list(transformed.components), transformed.unit)
    
    def apply_to_vector(self, vector: Vector) -> Vector:
        """Apply only linear part to vector (no translation)."""
        return self.linear.apply(vector)
    
    def __matmul__(self, other: 'AffineTransform') -> 'AffineTransform':
        """Compose affine transforms."""
        if not isinstance(other, AffineTransform):
            raise TypeError("Can only compose with AffineTransform")
        
        # (A1, b1) @ (A2, b2) = (A1*A2, A1*b2 + b1)
        new_linear = self.linear @ other.linear
        new_translation = self.linear.apply(other.translation) + self.translation
        
        return AffineTransform(new_linear, new_translation)