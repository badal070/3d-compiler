"""
Integrals - Numeric integration.
"""

from typing import Callable
from mathlib.core.scalar import Scalar


def integrate(f: Callable[[float], float], a: float, b: float, n: int = 1000) -> Scalar:
    """
    Numeric integration using Simpson's rule.
    """
    if n % 2 == 1:
        n += 1  # Simpson's rule needs even n
    
    h = (b - a) / n
    x = a
    
    total = f(a) + f(b)
    
    for i in range(1, n):
        x = a + i * h
        if i % 2 == 0:
            total += 2 * f(x)
        else:
            total += 4 * f(x)
    
    result = total * h / 3
    return Scalar(result)