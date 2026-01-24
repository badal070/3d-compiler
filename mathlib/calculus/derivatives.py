"""
Derivatives - Symbolic and numeric differentiation.
"""

from typing import Callable, List
from mathlib.core.scalar import Scalar
from mathlib.core.vector import Vector


def derivative(f: Callable[[float], float], x: float, h: float = 1e-7) -> Scalar:
    """Compute numeric derivative using central difference."""
    df = (f(x + h) - f(x - h)) / (2 * h)
    return Scalar(df)


def gradient(f: Callable[[List[float]], float], x: List[float], h: float = 1e-7) -> Vector:
    """Compute gradient (partial derivatives) of scalar function."""
    n = len(x)
    grad_components = []
    
    for i in range(n):
        x_plus = x.copy()
        x_minus = x.copy()
        x_plus[i] += h
        x_minus[i] -= h
        
        partial = (f(x_plus) - f(x_minus)) / (2 * h)
        grad_components.append(partial)
    
    return Vector(grad_components)