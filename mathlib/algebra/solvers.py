"""
Solvers - Equation solvers.
"""

from mathlib.algebra.equations import Equation
from mathlib.algebra.expressions import Variable


def solve(equation: Equation, variable: str, initial_guess: float = 0.0, 
          tolerance: float = 1e-9, max_iterations: int = 100) -> float:
    """
    Solve equation for variable using Newton's method.
    
    Simplified numeric solver.
    """
    x = initial_guess
    h = 1e-7
    
    for _ in range(max_iterations):
        context = {variable: x}
        
        # Compute f(x) = lhs(x) - rhs(x)
        f_x = equation.evaluate_error(context)
        
        if abs(f_x) < tolerance:
            return x
        
        # Compute f'(x) numerically
        context_plus = {variable: x + h}
        f_plus = equation.evaluate_error(context_plus)
        df = (f_plus - f_x) / h
        
        if abs(df) < 1e-12:
            raise ValueError("Derivative too small, cannot converge")
        
        # Newton's method update
        x = x - f_x / df
    
    raise ValueError(f"Failed to converge after {max_iterations} iterations")