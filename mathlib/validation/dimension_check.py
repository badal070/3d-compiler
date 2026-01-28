"""
Dimension checking for vectors and matrices.
"""

from mathlib.core.vector import Vector
from mathlib.core.matrix import Matrix
from mathlib.errors.math_errors import DimensionError


def check_dimensions(obj1, obj2, operation: str):
    """
    Check dimension compatibility for operation.
    """
    if isinstance(obj1, Vector) and isinstance(obj2, Vector):
        if obj1.dimension != obj2.dimension:
            raise DimensionError(obj1.dimension, obj2.dimension, operation)
    
    elif isinstance(obj1, Matrix) and isinstance(obj2, Matrix):
        if operation in ('add', 'subtract'):
            if obj1.shape != obj2.shape:
                raise DimensionError(obj1.shape, obj2.shape, operation)
        elif operation == 'multiply':
            if obj1.cols != obj2.rows:
                raise DimensionError(
                    f"{obj1.rows}x{obj1.cols}",
                    f"{obj2.rows}x{obj2.cols}",
                    "matrix multiplication"
                )