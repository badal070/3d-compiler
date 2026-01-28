"""
Rotation - First-class rotation transformation.

Transforms are first-class citizens, not matrices with delusions.
"""

from dataclasses import dataclass
from typing import Union
import math

from mathlib.core.scalar import Scalar
from mathlib.core.vector import Vector
from mathlib.core.matrix import Matrix
from mathlib.core.units import RADIAN, Angle
from mathlib.geometry.point import Point
from mathlib.errors.math_errors import AngleUnitError, InvalidOperationError, ZeroVectorError


@dataclass(frozen=True)
class Rotation:
    """Immutable rotation transformation."""
    
    axis: Union[str, Vector]
    angle: Scalar
    _matrix: Matrix = None
    
    def __init__(self, axis: Union[str, Vector], angle: Scalar):
        """
        Create a rotation.
        
        Args:
            axis: Rotation axis ("x", "y", "z") or arbitrary Vector
            angle: Rotation angle (must be in radians)
        """
        if not isinstance(angle, Scalar):
            raise TypeError(f"angle must be Scalar, got {type(angle)}")
        
        if not isinstance(angle.unit, Angle):
            raise AngleUnitError("radians", str(angle.unit))
        
        if angle.unit.angle_type != "radians":
            raise AngleUnitError("radians", angle.unit.angle_type)
        
        # Validate and normalize axis
        if isinstance(axis, str):
            if axis not in ("x", "y", "z"):
                raise ValueError(f"axis must be 'x', 'y', or 'z', got '{axis}'")
            object.__setattr__(self, 'axis', axis)
        elif isinstance(axis, Vector):
            if axis.dimension != 3:
                raise InvalidOperationError(
                    "rotation",
                    "arbitrary axis rotation only defined in 3D"
                )
            if axis.is_zero():
                raise ZeroVectorError("Rotation axis cannot be zero vector")
            # Normalize axis
            normalized = axis.normalize()
            object.__setattr__(self, 'axis', normalized)
        else:
            raise TypeError(f"axis must be str or Vector, got {type(axis)}")
        
        object.__setattr__(self, 'angle', angle)
        
        # Precompute matrix
        matrix = self._compute_matrix()
        object.__setattr__(self, '_matrix', matrix)
    
    def _compute_matrix(self) -> Matrix:
        """Compute rotation matrix."""
        c = math.cos(self.angle.value)
        s = math.sin(self.angle.value)
        
        if isinstance(self.axis, str):
            # Standard axis rotations
            if self.axis == "x":
                return Matrix([
                    [1.0, 0.0, 0.0],
                    [0.0, c, -s],
                    [0.0, s, c]
                ])
            elif self.axis == "y":
                return Matrix([
                    [c, 0.0, s],
                    [0.0, 1.0, 0.0],
                    [-s, 0.0, c]
                ])
            else:  # z
                return Matrix([
                    [c, -s, 0.0],
                    [s, c, 0.0],
                    [0.0, 0.0, 1.0]
                ])
        else:
            # Rodrigues' rotation formula for arbitrary axis
            # R = I + sin(θ)K + (1-cos(θ))K²
            # where K is the skew-symmetric matrix of the axis
            
            ux, uy, uz = self.axis.components
            
            # K (skew-symmetric matrix)
            K = Matrix([
                [0.0, -uz, uy],
                [uz, 0.0, -ux],
                [-uy, ux, 0.0]
            ])
            
            # K²
            K2 = K @ K
            
            # I + sin(θ)K + (1-cos(θ))K²
            I = Matrix.identity(3)
            R = I + (K * s) + (K2 * (1.0 - c))
            
            return R
    
    def __str__(self) -> str:
        if isinstance(self.axis, str):
            return f"Rotation(axis={self.axis}, angle={self.angle})"
        return f"Rotation(axis=custom, angle={self.angle})"
    
    def __repr__(self) -> str:
        return self.__str__()
    
    def as_matrix(self) -> Matrix:
        """Get rotation as matrix."""
        return self._matrix
    
    def apply_to_vector(self, vector: Vector) -> Vector:
        """Apply rotation to a vector."""
        if not isinstance(vector, Vector):
            raise TypeError(f"Expected Vector, got {type(vector)}")
        
        if vector.dimension != 3:
            raise InvalidOperationError(
                "rotation application",
                "rotation only defined for 3D vectors"
            )
        
        return self._matrix.apply(vector)
    
    def apply_to_point(self, point: Point) -> Point:
        """Apply rotation to a point (rotates around origin)."""
        if not isinstance(point, Point):
            raise TypeError(f"Expected Point, got {type(point)}")
        
        if point.dimension != 3:
            raise InvalidOperationError(
                "rotation application",
                "rotation only defined for 3D points"
            )
        
        rotated_pos = self._matrix.apply(point.position)
        return Point(list(rotated_pos.components), rotated_pos.unit)
    
    def inverse(self) -> 'Rotation':
        """Get inverse rotation (opposite angle)."""
        return Rotation(self.axis, -self.angle)
    
    def __matmul__(self, other: 'Rotation') -> 'Rotation':
        """
        Compose rotations using @ operator.
        
        Note: This creates a new arbitrary-axis rotation.
        """
        if not isinstance(other, Rotation):
            raise TypeError(f"Cannot compose Rotation with {type(other)}")
        
        # Compose matrices
        composed_matrix = self._matrix @ other._matrix
        
        # For now, return as a matrix-based rotation
        # Full decomposition to axis-angle would require more complex math
        # This is a simplified version that maintains the interface
        
        # Extract axis and angle from composed matrix (simplified)
        # In practice, you'd use proper axis-angle extraction
        # For educational purposes, we'll create a matrix wrapper
        
        # Store composition info
        # This is a placeholder - full implementation would extract axis-angle
        raise NotImplementedError(
            "Rotation composition via @ operator requires axis-angle extraction. "
            "Use .as_matrix() and compose matrices instead."
        )
    
    @staticmethod
    def from_matrix(matrix: Matrix) -> 'Rotation':
        """Create rotation from rotation matrix (extracts axis and angle)."""
        # This would implement axis-angle extraction from matrix
        # Complex algorithm - placeholder for now
        raise NotImplementedError(
            "Extracting axis-angle from matrix not yet implemented"
        )