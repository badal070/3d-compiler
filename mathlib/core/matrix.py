"""
Matrix - Linear transformation only.

No silent inverses. No guessing shapes.
"""

from dataclasses import dataclass
from typing import List, Tuple
import math

from mathlib.core.vector import Vector
from mathlib.core.scalar import Scalar
from mathlib.core.units import Unit, UNITLESS, Dimensionless, RADIAN
from mathlib.errors.math_errors import (
    DimensionError,
    SingularMatrixError,
    InvalidOperationError
)


@dataclass(frozen=True)
class Matrix:
    """Immutable matrix for linear transformations."""
    
    elements: tuple  # Stored as row-major tuple of tuples
    rows: int
    cols: int
    
    def __init__(self, elements: List[List[float]]):
        """
        Create a matrix.
        
        Args:
            elements: 2D list of numeric values (row-major)
        """
        if not elements:
            raise ValueError("Matrix must have at least one row")
        if not elements[0]:
            raise ValueError("Matrix must have at least one column")
        
        rows = len(elements)
        cols = len(elements[0])
        
        # Validate all rows have same length
        for i, row in enumerate(elements):
            if len(row) != cols:
                raise ValueError(f"Row {i} has {len(row)} columns, expected {cols}")
        
        # Convert to immutable tuple of tuples
        elem_tuple = tuple(tuple(float(x) for x in row) for row in elements)
        
        object.__setattr__(self, 'elements', elem_tuple)
        object.__setattr__(self, 'rows', rows)
        object.__setattr__(self, 'cols', cols)
    
    @property
    def shape(self) -> Tuple[int, int]:
        """Get matrix shape (rows, cols)."""
        return (self.rows, self.cols)
    
    def __str__(self) -> str:
        lines = []
        for row in self.elements:
            row_str = " ".join(f"{x:8.4f}" for x in row)
            lines.append(f"[{row_str}]")
        return "\n".join(lines)
    
    def __repr__(self) -> str:
        return f"Matrix({self.rows}x{self.cols})"
    
    def __getitem__(self, index: Tuple[int, int]) -> float:
        """Access element at (row, col)."""
        row, col = index
        return self.elements[row][col]
    
    def __eq__(self, other) -> bool:
        """Check equality with tolerance."""
        if not isinstance(other, Matrix):
            return False
        if self.shape != other.shape:
            return False
        
        for i in range(self.rows):
            for j in range(self.cols):
                if not math.isclose(self[i, j], other[i, j], rel_tol=1e-9):
                    return False
        return True
    
    # Matrix operations
    def __add__(self, other: 'Matrix') -> 'Matrix':
        """Add two matrices."""
        if not isinstance(other, Matrix):
            raise TypeError(f"Cannot add Matrix and {type(other)}")
        
        if self.shape != other.shape:
            raise DimensionError(self.shape, other.shape, "matrix addition")
        
        result = []
        for i in range(self.rows):
            row = [self[i, j] + other[i, j] for j in range(self.cols)]
            result.append(row)
        
        return Matrix(result)
    
    def __sub__(self, other: 'Matrix') -> 'Matrix':
        """Subtract two matrices."""
        if not isinstance(other, Matrix):
            raise TypeError(f"Cannot subtract {type(other)} from Matrix")
        
        if self.shape != other.shape:
            raise DimensionError(self.shape, other.shape, "matrix subtraction")
        
        result = []
        for i in range(self.rows):
            row = [self[i, j] - other[i, j] for j in range(self.cols)]
            result.append(row)
        
        return Matrix(result)
    
    def __matmul__(self, other: 'Matrix') -> 'Matrix':
        """Matrix multiplication using @ operator."""
        if not isinstance(other, Matrix):
            raise TypeError(f"Cannot multiply Matrix and {type(other)}")
        
        if self.cols != other.rows:
            raise DimensionError(
                f"{self.rows}x{self.cols}",
                f"{other.rows}x{other.cols}",
                "matrix multiplication (inner dimensions must match)"
            )
        
        result = []
        for i in range(self.rows):
            row = []
            for j in range(other.cols):
                val = sum(self[i, k] * other[k, j] for k in range(self.cols))
                row.append(val)
            result.append(row)
        
        return Matrix(result)
    
    def __mul__(self, scalar: float) -> 'Matrix':
        """Scalar multiplication."""
        if not isinstance(scalar, (int, float)):
            raise TypeError(f"Cannot multiply Matrix by {type(scalar)}")
        
        result = []
        for i in range(self.rows):
            row = [self[i, j] * scalar for j in range(self.cols)]
            result.append(row)
        
        return Matrix(result)
    
    def __rmul__(self, scalar: float) -> 'Matrix':
        """Right scalar multiplication."""
        return self.__mul__(scalar)
    
    def transpose(self) -> 'Matrix':
        """Return transpose of matrix."""
        result = []
        for j in range(self.cols):
            row = [self[i, j] for i in range(self.rows)]
            result.append(row)
        
        return Matrix(result)
    
    def determinant(self) -> float:
        """Compute determinant (square matrices only)."""
        if self.rows != self.cols:
            raise InvalidOperationError(
                "determinant",
                "only defined for square matrices"
            )
        
        return self._determinant_recursive(self.elements)
    
    def _determinant_recursive(self, matrix: tuple) -> float:
        """Recursive determinant calculation."""
        n = len(matrix)
        
        if n == 1:
            return matrix[0][0]
        
        if n == 2:
            return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
        
        det = 0.0
        for j in range(n):
            # Create minor matrix
            minor = []
            for i in range(1, n):
                row = [matrix[i][k] for k in range(n) if k != j]
                minor.append(tuple(row))
            
            cofactor = ((-1) ** j) * matrix[0][j] * self._determinant_recursive(tuple(minor))
            det += cofactor
        
        return det
    
    def inverse(self) -> 'Matrix':
        """Compute matrix inverse (explicit, not silent)."""
        if self.rows != self.cols:
            raise InvalidOperationError(
                "matrix inversion",
                "only defined for square matrices"
            )
        
        det = self.determinant()
        if abs(det) < 1e-10:
            raise SingularMatrixError(det)
        
        # Use Gauss-Jordan elimination
        n = self.rows
        
        # Create augmented matrix [A | I]
        aug = []
        for i in range(n):
            row = list(self.elements[i]) + [1.0 if i == j else 0.0 for j in range(n)]
            aug.append(row)
        
        # Forward elimination
        for i in range(n):
            # Find pivot
            max_row = i
            for k in range(i + 1, n):
                if abs(aug[k][i]) > abs(aug[max_row][i]):
                    max_row = k
            
            aug[i], aug[max_row] = aug[max_row], aug[i]
            
            if abs(aug[i][i]) < 1e-10:
                raise SingularMatrixError()
            
            # Scale pivot row
            pivot = aug[i][i]
            aug[i] = [x / pivot for x in aug[i]]
            
            # Eliminate column
            for k in range(n):
                if k != i:
                    factor = aug[k][i]
                    aug[k] = [aug[k][j] - factor * aug[i][j] for j in range(2 * n)]
        
        # Extract inverse from right half
        result = []
        for i in range(n):
            row = aug[i][n:]
            result.append(row)
        
        return Matrix(result)
    
    def apply(self, vector: Vector) -> Vector:
        """Apply matrix transformation to vector."""
        if not isinstance(vector, Vector):
            raise TypeError(f"Cannot apply matrix to {type(vector)}")
        
        if self.cols != vector.dimension:
            raise DimensionError(
                self.cols,
                vector.dimension,
                "matrix-vector multiplication"
            )
        
        result = []
        for i in range(self.rows):
            val = sum(self[i, j] * vector[j] for j in range(self.cols))
            result.append(val)
        
        return Vector(result, f"R{self.rows}", vector.unit)
    
    # Factory methods for common matrices
    @staticmethod
    def identity(n: int) -> 'Matrix':
        """Create n×n identity matrix."""
        elements = []
        for i in range(n):
            row = [1.0 if i == j else 0.0 for j in range(n)]
            elements.append(row)
        return Matrix(elements)
    
    @staticmethod
    def zero(rows: int, cols: int) -> 'Matrix':
        """Create zero matrix."""
        elements = [[0.0] * cols for _ in range(rows)]
        return Matrix(elements)
    
    @staticmethod
    def rotation_x(theta: Scalar) -> 'Matrix':
        """Create 3D rotation matrix around X axis."""
        if not isinstance(theta.unit, type(RADIAN)):
            from mathlib.errors.math_errors import AngleUnitError
            raise AngleUnitError("radians", str(theta.unit))
        
        c = math.cos(theta.value)
        s = math.sin(theta.value)
        
        return Matrix([
            [1.0, 0.0, 0.0],
            [0.0, c, -s],
            [0.0, s, c]
        ])
    
    @staticmethod
    def rotation_y(theta: Scalar) -> 'Matrix':
        """Create 3D rotation matrix around Y axis."""
        if not isinstance(theta.unit, type(RADIAN)):
            from mathlib.errors.math_errors import AngleUnitError
            raise AngleUnitError("radians", str(theta.unit))
        
        c = math.cos(theta.value)
        s = math.sin(theta.value)
        
        return Matrix([
            [c, 0.0, s],
            [0.0, 1.0, 0.0],
            [-s, 0.0, c]
        ])
    
    @staticmethod
    def rotation_z(theta: Scalar) -> 'Matrix':
        """Create 3D rotation matrix around Z axis."""
        if not isinstance(theta.unit, type(RADIAN)):
            from mathlib.errors.math_errors import AngleUnitError
            raise AngleUnitError("radians", str(theta.unit))
        
        c = math.cos(theta.value)
        s = math.sin(theta.value)
        
        return Matrix([
            [c, -s, 0.0],
            [s, c, 0.0],
            [0.0, 0.0, 1.0]
        ])