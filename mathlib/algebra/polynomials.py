"""
Polynomials - Polynomial representation and operations.
"""

from dataclasses import dataclass
from typing import List


@dataclass(frozen=True)
class Polynomial:
    """Polynomial with coefficients."""
    
    coefficients: tuple  # [a0, a1, a2, ...] for a0 + a1*x + a2*x^2 + ...
    
    def __init__(self, coefficients: List[float]):
        object.__setattr__(self, 'coefficients', tuple(coefficients))
    
    @property
    def degree(self) -> int:
        """Get polynomial degree."""
        return len(self.coefficients) - 1
    
    def evaluate(self, x: float) -> float:
        """Evaluate polynomial at x using Horner's method."""
        result = 0.0
        for coeff in reversed(self.coefficients):
            result = result * x + coeff
        return result
    
    def __str__(self) -> str:
        terms = []
        for i, coeff in enumerate(self.coefficients):
            if abs(coeff) > 1e-10:
                if i == 0:
                    terms.append(f"{coeff}")
                elif i == 1:
                    terms.append(f"{coeff}x")
                else:
                    terms.append(f"{coeff}x^{i}")
        return " + ".join(terms) if terms else "0"