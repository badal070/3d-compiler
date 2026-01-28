"""
Equations - Equation representation.
"""

from dataclasses import dataclass
from mathlib.algebra.expressions import Expression


@dataclass(frozen=True)
class Equation:
    """Equation: lhs = rhs"""
    
    lhs: Expression
    rhs: Expression
    
    def __str__(self) -> str:
        return f"{self.lhs} = {self.rhs}"
    
    def evaluate_error(self, context: dict) -> float:
        """Compute |lhs - rhs| for given variable values."""
        return abs(self.lhs.evaluate(context) - self.rhs.evaluate(context))