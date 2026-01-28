"""
Scale - Non-uniform scaling transformation.
"""

from dataclasses import dataclass
from typing import Union, List
from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.core.matrix import Matrix
from mathlib.geometry.point import Point
from mathlib.errors.math_errors import DimensionError, InvalidOperationError


@dataclass(frozen=True)
class Scale:
    """Immutable scale transformation."""
    
    factors: Vector
    
    def __init__(self, factors: Union[Vector, List[float], float]):
        if isinstance(factors, (int, float)):
            # Uniform scaling in 3D
            factors = Vector([factors, factors, factors])
        elif isinstance(factors, list):
            factors = Vector(factors)
        elif not isinstance(factors, Vector):
            raise TypeError(f"factors must be Vector, list, or scalar")
        
        object.__setattr__(self, 'factors', factors)
    
    @property
    def dimension(self) -> int:
        return self.factors.dimension
    
    def __str__(self) -> str:
        return f"Scale({self.factors})"
    
    def as_matrix(self) -> Matrix:
        """Convert to diagonal matrix."""
        n = self.dimension
        elements = []
        for i in range(n):
            row = [self.factors[i] if i == j else 0.0 for j in range(n)]
            elements.append(row)
        return Matrix(elements)
    
    def apply_to_vector(self, vector: Vector) -> Vector:
        """Apply scaling to vector."""
        if vector.dimension != self.dimension:
            raise DimensionError(
                self.dimension,
                vector.dimension,
                "scale application"
            )
        
        scaled = [v * s for v, s in zip(vector.components, self.factors.components)]
        return Vector(scaled, vector.space, vector.unit)
    
    def apply_to_point(self, point: Point) -> Point:
        """Apply scaling to point (scales from origin)."""
        if point.dimension != self.dimension:
            raise DimensionError(
                self.dimension,
                point.dimension,
                "scale application"
            )
        
        scaled_pos = self.apply_to_vector(point.position)
        return Point(list(scaled_pos.components), scaled_pos.unit)
    
    @staticmethod
    def uniform(factor: float, dimension: int = 3) -> 'Scale':
        """Create uniform scaling."""
        return Scale([factor] * dimension)