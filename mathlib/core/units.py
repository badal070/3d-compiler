"""
Explicit unit system to prevent unit crimes.

Hard separation: length, angle, time, mass.
Adding meters to radians? We throw hands (errors).
"""

from dataclasses import dataclass
from typing import Any
from mathlib.errors.math_errors import UnitError, AngleUnitError


@dataclass(frozen=True)
class Unit:
    """Base unit with dimension tracking."""
    
    dimension: str
    symbol: str
    
    def __str__(self) -> str:
        return self.symbol
    
    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Unit):
            return False
        return self.dimension == other.dimension and self.symbol == other.symbol
    
    def is_compatible(self, other: 'Unit') -> bool:
        """Check if units can be used together."""
        return self.dimension == other.dimension
    
    def assert_compatible(self, other: 'Unit', operation: str = None):
        """Raise error if units are incompatible."""
        if not self.is_compatible(other):
            raise UnitError(self, other, operation)


# Length units
@dataclass(frozen=True)
class Length(Unit):
    """Length/distance unit."""
    
    def __init__(self, symbol: str = "m"):
        object.__setattr__(self, 'dimension', 'length')
        object.__setattr__(self, 'symbol', symbol)


# Angle units (the crime scene)
@dataclass(frozen=True)
class Angle(Unit):
    """Angle unit - strictly separated radians vs degrees."""
    
    angle_type: str  # 'radians' or 'degrees'
    
    def __init__(self, angle_type: str):
        if angle_type not in ('radians', 'degrees'):
            raise ValueError(f"angle_type must be 'radians' or 'degrees', got '{angle_type}'")
        
        object.__setattr__(self, 'dimension', 'angle')
        object.__setattr__(self, 'angle_type', angle_type)
        object.__setattr__(self, 'symbol', 'rad' if angle_type == 'radians' else 'deg')
    
    @staticmethod
    def radians() -> 'Angle':
        """Create radians unit."""
        return Angle('radians')
    
    @staticmethod
    def degrees() -> 'Angle':
        """Create degrees unit."""
        return Angle('degrees')
    
    def is_compatible(self, other: 'Unit') -> bool:
        """Angles are only compatible with same angle type."""
        if not isinstance(other, Angle):
            return False
        return self.angle_type == other.angle_type
    
    def assert_compatible(self, other: 'Unit', operation: str = None):
        """Raise angle-specific error if incompatible."""
        if not isinstance(other, Angle):
            raise UnitError(self, other, operation)
        if self.angle_type != other.angle_type:
            raise AngleUnitError(self.angle_type, other.angle_type)


# Time units
@dataclass(frozen=True)
class Time(Unit):
    """Time unit."""
    
    def __init__(self, symbol: str = "s"):
        object.__setattr__(self, 'dimension', 'time')
        object.__setattr__(self, 'symbol', symbol)


# Mass units
@dataclass(frozen=True)
class Mass(Unit):
    """Mass unit."""
    
    def __init__(self, symbol: str = "kg"):
        object.__setattr__(self, 'dimension', 'mass')
        object.__setattr__(self, 'symbol', symbol)


# Dimensionless (for pure numbers)
@dataclass(frozen=True)
class Dimensionless(Unit):
    """Dimensionless quantity (pure number)."""
    
    def __init__(self):
        object.__setattr__(self, 'dimension', 'dimensionless')
        object.__setattr__(self, 'symbol', '')
    
    def is_compatible(self, other: 'Unit') -> bool:
        """Dimensionless is only compatible with dimensionless."""
        return isinstance(other, Dimensionless)


# Predefined common units
METER = Length("m")
CENTIMETER = Length("cm")
MILLIMETER = Length("mm")

RADIAN = Angle.radians()
DEGREE = Angle.degrees()

SECOND = Time("s")
MILLISECOND = Time("ms")

KILOGRAM = Mass("kg")
GRAM = Mass("g")

UNITLESS = Dimensionless()