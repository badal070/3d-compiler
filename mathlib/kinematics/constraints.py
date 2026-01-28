"""
Constraints - Kinematic constraints.
"""

from dataclasses import dataclass
from abc import ABC, abstractmethod
from mathlib.geometry.point import Point


class Constraint(ABC):
    """Abstract constraint on kinematic system."""
    
    @abstractmethod
    def is_satisfied(self, point: Point) -> bool:
        """Check if constraint is satisfied."""
        pass
    
    @abstractmethod
    def error(self, point: Point) -> float:
        """Compute constraint violation magnitude."""
        pass


@dataclass(frozen=True)
class PositionConstraint(Constraint):
    """Constrain position to specific point."""
    
    target: Point
    tolerance: float = 1e-6
    
    def is_satisfied(self, point: Point) -> bool:
        dist = point.distance_to(self.target)
        return dist.value < self.tolerance
    
    def error(self, point: Point) -> float:
        return point.distance_to(self.target).value