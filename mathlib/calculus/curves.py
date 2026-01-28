"""
Curves - Parametric curves for animation.
"""

from dataclasses import dataclass
from typing import Callable
from mathlib.geometry.point import Point
from mathlib.core.vector import Vector


@dataclass(frozen=True)
class ParametricCurve:
    """Parametric curve in space."""
    
    func: Callable[[float], Point]
    t_min: float
    t_max: float
    
    def evaluate(self, t: float) -> Point:
        """Evaluate curve at parameter t."""
        if t < self.t_min or t > self.t_max:
            raise ValueError(f"Parameter t={t} outside range [{self.t_min}, {self.t_max}]")
        return self.func(t)
    
    def sample(self, n: int) -> list:
        """Sample n points along curve."""
        samples = []
        for i in range(n):
            t = self.t_min + (self.t_max - self.t_min) * i / (n - 1)
            samples.append(self.evaluate(t))
        return samples


# Alias for symbolic expression-based curves
Curve = ParametricCurve