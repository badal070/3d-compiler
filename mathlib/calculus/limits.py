"""
Limits - Symbolic and numeric limit computation.
"""

from typing import Callable
from mathlib.core.scalar import Scalar


def limit(f: Callable[[float], float], x0: float, epsilon: float = 1e-9) -> Scalar:
    """
    Compute numeric limit of f as x approaches x0.
    
    This is a simplified numeric approximation.
    """
    h = epsilon
    left = f(x0 - h)
    right = f(x0 + h)
    
    # Check if limit exists
    if abs(left - right) > epsilon * 10:
        raise ValueError(f"Limit does not exist at x={x0}")
    
    return Scalar((left + right) / 2.0)