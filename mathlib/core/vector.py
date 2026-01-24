"""
Vector - Strict dimensioned vector.

Rules:
- No auto-broadcasting
- Dot, cross, norm explicitly named
- Bad math habits die here
"""

from dataclasses import dataclass
from typing import List, Union
import math

from mathlib.core.scalar import Scalar
from mathlib.core.units import Unit, UNITLESS, Dimensionless
from mathlib.errors.math_errors import (
    DimensionError, 
    UnitError, 
    ZeroVectorError,
    InvalidOperationError
)


@dataclass(frozen=True)
class Vector:
    """Immutable vector with explicit dimension and units."""
    
    components: tuple
    space: str  # e.g., "R2", "R3"
    unit: Unit = UNITLESS
    
    def __init__(self, components: List[float], space: str = None, unit: Unit = UNITLESS):
        """
        Create a vector.
        
        Args:
            components: List of numeric values
            space: Vector space (e.g., "R2", "R3"), auto-detected if None
            unit: Unit for all components
        """
        # Validate components
        if not components:
            raise ValueError("Vector must have at least one component")
        
        # Convert to tuple of floats for immutability
        comp_tuple = tuple(float(c) for c in components)
        object.__setattr__(self, 'components', comp_tuple)
        
        # Determine space
        if space is None:
            space = f"R{len(comp_tuple)}"
        object.__setattr__(self, 'space', space)
        
        # Set unit
        if not isinstance(unit, Unit):
            raise TypeError(f"unit must be a Unit instance, got {type(unit)}")
        object.__setattr__(self, 'unit', unit)
        
        # Validate space matches dimension
        expected_dim = len(comp_tuple)
        if space.startswith("R") and space[1:].isdigit():
            space_dim = int(space[1:])
            if space_dim != expected_dim:
                raise DimensionError(space_dim, expected_dim, "vector initialization")
    
    @property
    def dimension(self) -> int:
        """Get vector dimension."""
        return len(self.components)
    
    def __str__(self) -> str:
        comp_str = ", ".join(f"{c:.6g}" for c in self.components)
        if isinstance(self.unit, Dimensionless):
            return f"[{comp_str}]"
        return f"[{comp_str}] {self.unit}"
    
    def __repr__(self) -> str:
        return f"Vector({list(self.components)}, {self.space}, {self.unit.symbol})"
    
    def __getitem__(self, index: int) -> float:
        """Access component by index."""
        return self.components[index]
    
    def __len__(self) -> int:
        """Get vector dimension."""
        return len(self.components)
    
    # Vector operations
    def __add__(self, other: 'Vector') -> 'Vector':
        """Add two vectors."""
        if not isinstance(other, Vector):
            raise TypeError(f"Cannot add Vector and {type(other)}")
        
        if self.dimension != other.dimension:
            raise DimensionError(self.dimension, other.dimension, "vector addition")
        
        self.unit.assert_compatible(other.unit, "vector addition")
        
        new_components = [a + b for a, b in zip(self.components, other.components)]
        return Vector(new_components, self.space, self.unit)
    
    def __sub__(self, other: 'Vector') -> 'Vector':
        """Subtract two vectors."""
        if not isinstance(other, Vector):
            raise TypeError(f"Cannot subtract {type(other)} from Vector")
        
        if self.dimension != other.dimension:
            raise DimensionError(self.dimension, other.dimension, "vector subtraction")
        
        self.unit.assert_compatible(other.unit, "vector subtraction")
        
        new_components = [a - b for a, b in zip(self.components, other.components)]
        return Vector(new_components, self.space, self.unit)
    
    def __mul__(self, scalar: Union[Scalar, float, int]) -> 'Vector':
        """Scalar multiplication."""
        if isinstance(scalar, (int, float)):
            scalar_val = scalar
        elif isinstance(scalar, Scalar):
            if not isinstance(scalar.unit, Dimensionless):
                raise UnitError(
                    self.unit,
                    scalar.unit,
                    "scalar multiplication (multiplier must be dimensionless)"
                )
            scalar_val = scalar.value
        else:
            raise TypeError(f"Cannot multiply Vector by {type(scalar)}")
        
        new_components = [c * scalar_val for c in self.components]
        return Vector(new_components, self.space, self.unit)
    
    def __rmul__(self, scalar: Union[float, int]) -> 'Vector':
        """Right scalar multiplication."""
        return self.__mul__(scalar)
    
    def __truediv__(self, scalar: Union[Scalar, float, int]) -> 'Vector':
        """Scalar division."""
        if isinstance(scalar, (int, float)):
            if scalar == 0:
                raise ZeroDivisionError("Division by zero")
            scalar_val = scalar
        elif isinstance(scalar, Scalar):
            if scalar.value == 0:
                raise ZeroDivisionError("Division by zero")
            if not isinstance(scalar.unit, Dimensionless):
                raise UnitError(
                    self.unit,
                    scalar.unit,
                    "scalar division (divisor must be dimensionless)"
                )
            scalar_val = scalar.value
        else:
            raise TypeError(f"Cannot divide Vector by {type(scalar)}")
        
        new_components = [c / scalar_val for c in self.components]
        return Vector(new_components, self.space, self.unit)
    
    def __neg__(self) -> 'Vector':
        """Negate vector."""
        return Vector([-c for c in self.components], self.space, self.unit)
    
    def __eq__(self, other) -> bool:
        """Check equality with tolerance."""
        if not isinstance(other, Vector):
            return False
        if self.dimension != other.dimension or self.unit != other.unit:
            return False
        return all(math.isclose(a, b, rel_tol=1e-9) 
                   for a, b in zip(self.components, other.components))
    
    # Explicit vector operations (no implicit broadcasting)
    def dot(self, other: 'Vector') -> Scalar:
        """Dot product (explicit, not operator overload)."""
        if not isinstance(other, Vector):
            raise TypeError(f"Cannot dot Vector with {type(other)}")
        
        if self.dimension != other.dimension:
            raise DimensionError(self.dimension, other.dimension, "dot product")
        
        self.unit.assert_compatible(other.unit, "dot product")
        
        result = sum(a * b for a, b in zip(self.components, other.components))
        
        # Result unit is unit^2 (simplified to same unit for now)
        return Scalar(result, self.unit)
    
    def cross(self, other: 'Vector') -> 'Vector':
        """Cross product (3D only, explicit)."""
        if not isinstance(other, Vector):
            raise TypeError(f"Cannot cross Vector with {type(other)}")
        
        if self.dimension != 3 or other.dimension != 3:
            raise InvalidOperationError(
                "cross product",
                "only defined for 3D vectors"
            )
        
        self.unit.assert_compatible(other.unit, "cross product")
        
        a, b, c = self.components
        d, e, f = other.components
        
        result = [
            b * f - c * e,
            c * d - a * f,
            a * e - b * d
        ]
        
        return Vector(result, "R3", self.unit)
    
    def norm(self) -> Scalar:
        """Euclidean norm (explicit)."""
        magnitude = math.sqrt(sum(c * c for c in self.components))
        return Scalar(magnitude, self.unit)
    
    def normalize(self) -> 'Vector':
        """Return unit vector in same direction."""
        n = self.norm()
        if n.is_zero():
            raise ZeroVectorError("Cannot normalize zero vector")
        
        # Normalized vector is dimensionless
        return Vector(
            [c / n.value for c in self.components],
            self.space,
            UNITLESS
        )
    
    def is_zero(self, tolerance: float = 1e-10) -> bool:
        """Check if vector is effectively zero."""
        return all(abs(c) < tolerance for c in self.components)
    
    @staticmethod
    def zero(dimension: int, unit: Unit = UNITLESS) -> 'Vector':
        """Create zero vector of given dimension."""
        return Vector([0.0] * dimension, f"R{dimension}", unit)
    
    @staticmethod
    def basis(dimension: int, index: int, unit: Unit = UNITLESS) -> 'Vector':
        """Create basis vector (1 at index, 0 elsewhere)."""
        if index < 0 or index >= dimension:
            raise ValueError(f"Index {index} out of range for dimension {dimension}")
        
        components = [0.0] * dimension
        components[index] = 1.0
        return Vector(components, f"R{dimension}", unit)