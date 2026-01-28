"""
Invariant checking for mathematical objects.
"""

from mathlib.core.matrix import Matrix
from mathlib.errors.validation_errors import InvariantError
import math


def validate_invariants(obj, invariant_type: str):
    """
    Validate mathematical invariants for object.
    """
    if invariant_type == "orthogonal_matrix":
        if not isinstance(obj, Matrix):
            raise TypeError("Object must be Matrix for orthogonal check")
        
        # Check if M^T * M = I
        Mt = obj.transpose()
        product = Mt @ obj
        identity = Matrix.identity(obj.rows)
        
        for i in range(obj.rows):
            for j in range(obj.cols):
                expected = 1.0 if i == j else 0.0
                if not math.isclose(product[i, j], expected, abs_tol=1e-9):
                    raise InvariantError(
                        "orthogonal_matrix",
                        f"Matrix is not orthogonal: M^T * M != I",
                        {"position": (i, j), "expected": expected, "got": product[i, j]}
                    )
    
    elif invariant_type == "unit_vector":
        from mathlib.core.vector import Vector
        if not isinstance(obj, Vector):
            raise TypeError("Object must be Vector for unit check")
        
        norm = obj.norm().value
        if not math.isclose(norm, 1.0, abs_tol=1e-9):
            raise InvariantError(
                "unit_vector",
                f"Vector is not unit length: ||v|| = {norm} != 1.0"
            )