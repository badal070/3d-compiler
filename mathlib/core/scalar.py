"""
Scalar - Single numeric value with metadata.

Prevents unit crimes. Makes DSL validation sane.
"""

from dataclasses import dataclass
from typing import Union
import math

from mathlib.core.units import Unit, UNITLESS, Dimensionless
from mathlib.errors.math_errors import UnitError, DimensionError


@dataclass(frozen=True)
class Scalar:
    """Immutable scalar value with explicit units."""
    
    value: float
    unit: Unit = UNITLESS
    
    def __post_init__(self):
        """Validate scalar on creation."""
        if not isinstance(self.value, (int, float)):
            raise TypeError(f"Scalar value must be numeric, got {type(self.value)}")
        if not isinstance(self.unit, Unit):
            raise TypeError(f"unit must be a Unit instance, got {type(self.unit)}")
        
        # Convert int to float for consistency
        if isinstance(self.value, int):
            object.__setattr__(self, 'value', float(self.value))
    
    def __str__(self) -> str:
        if isinstance(self.unit, Dimensionless):
            return f"{self.value}"
        return f"{self.value} {self.unit}"
    
    def __repr__(self) -> str:
        return f"Scalar({self.value}, {self.unit.symbol})"
    
    # Arithmetic operations
    def __add__(self, other: 'Scalar') -> 'Scalar':
        """Add two scalars with unit checking."""
        if not isinstance(other, Scalar):
            raise TypeError(f"Cannot add Scalar and {type(other)}")
        
        self.unit.assert_compatible(other.unit, "addition")
        return Scalar(self.value + other.value, self.unit)
    
    def __sub__(self, other: 'Scalar') -> 'Scalar':
        """Subtract two scalars with unit checking."""
        if not isinstance(other, Scalar):
            raise TypeError(f"Cannot subtract {type(other)} from Scalar")
        
        self.unit.assert_compatible(other.unit, "subtraction")
        return Scalar(self.value - other.value, self.unit)
    
    def __mul__(self, other: Union['Scalar', float, int]) -> 'Scalar':
        """Multiply scalar (dimensionless multipliers only)."""
        if isinstance(other, (int, float)):
            return Scalar(self.value * other, self.unit)
        
        if isinstance(other, Scalar):
            # Only allow multiplication by dimensionless scalars
            if not isinstance(other.unit, Dimensionless):
                raise UnitError(
                    self.unit, 
                    other.unit,
                    "multiplication (only dimensionless multipliers allowed)"
                )
            return Scalar(self.value * other.value, self.unit)
        
        raise TypeError(f"Cannot multiply Scalar and {type(other)}")
    
    def __rmul__(self, other: Union[float, int]) -> 'Scalar':
        """Right multiplication (for numeric * Scalar)."""
        return self.__mul__(other)
    
    def __truediv__(self, other: Union['Scalar', float, int]) -> 'Scalar':
        """Divide scalar (dimensionless divisors only)."""
        if isinstance(other, (int, float)):
            if other == 0:
                raise ZeroDivisionError("Division by zero")
            return Scalar(self.value / other, self.unit)
        
        if isinstance(other, Scalar):
            if other.value == 0:
                raise ZeroDivisionError("Division by zero")
            
            # Only allow division by dimensionless scalars
            if not isinstance(other.unit, Dimensionless):
                raise UnitError(
                    self.unit,
                    other.unit,
                    "division (only dimensionless divisors allowed)"
                )
            return Scalar(self.value / other.value, self.unit)
        
        raise TypeError(f"Cannot divide Scalar by {type(other)}")
    
    def __neg__(self) -> 'Scalar':
        """Negate scalar."""
        return Scalar(-self.value, self.unit)
    
    def __abs__(self) -> 'Scalar':
        """Absolute value."""
        return Scalar(abs(self.value), self.unit)
    
    # Comparison operations
    def __eq__(self, other) -> bool:
        if not isinstance(other, Scalar):
            return False
        return (math.isclose(self.value, other.value, rel_tol=1e-9) and 
                self.unit == other.unit)
    
    def __lt__(self, other: 'Scalar') -> bool:
        if not isinstance(other, Scalar):
            raise TypeError(f"Cannot compare Scalar and {type(other)}")
        self.unit.assert_compatible(other.unit, "comparison")
        return self.value < other.value
    
    def __le__(self, other: 'Scalar') -> bool:
        if not isinstance(other, Scalar):
            raise TypeError(f"Cannot compare Scalar and {type(other)}")
        self.unit.assert_compatible(other.unit, "comparison")
        return self.value <= other.value
    
    def __gt__(self, other: 'Scalar') -> bool:
        return not self.__le__(other)
    
    def __ge__(self, other: 'Scalar') -> bool:
        return not self.__lt__(other)
    
    # Utility methods
    def is_zero(self, tolerance: float = 1e-10) -> bool:
        """Check if scalar is effectively zero."""
        return abs(self.value) < tolerance
    
    def to_unit(self, target_unit: Unit) -> 'Scalar':
        """Convert to different unit (must implement conversion logic)."""
        # For now, just check compatibility
        self.unit.assert_compatible(target_unit, "unit conversion")
        # Actual conversion would require conversion factors
        # This is a placeholder for future implementation
        return Scalar(self.value, target_unit)