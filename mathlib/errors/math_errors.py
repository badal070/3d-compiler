"""
Math-specific errors that fail loudly and educate.

Philosophy: Errors are descriptive, non-recovering, and educational.
"""


class MathLibError(Exception):
    """Base exception for all MathLib errors."""
    
    def __init__(self, message: str, context: dict = None):
        self.message = message
        self.context = context or {}
        super().__init__(self._format_message())
    
    def _format_message(self) -> str:
        """Format error with educational context."""
        parts = [self.message]
        if self.context:
            parts.append("\nContext:")
            for key, value in self.context.items():
                parts.append(f"  {key}: {value}")
        return "\n".join(parts)


class DimensionError(MathLibError):
    """Raised when vector/matrix dimensions don't match for operation."""
    
    def __init__(self, expected, got, operation: str = None):
        context = {
            "expected_dimension": expected,
            "got_dimension": got,
        }
        if operation:
            context["operation"] = operation
        
        message = f"Dimension mismatch: expected {expected}, got {got}"
        if operation:
            message += f" for operation '{operation}'"
        
        super().__init__(message, context)


class UnitError(MathLibError):
    """Raised when units are incompatible for operation."""
    
    def __init__(self, unit1, unit2, operation: str = None):
        context = {
            "unit1": str(unit1),
            "unit2": str(unit2),
        }
        if operation:
            context["operation"] = operation
        
        message = f"Incompatible units: {unit1} and {unit2}"
        if operation:
            message += f" for operation '{operation}'"
        
        super().__init__(message, context)


class AngleUnitError(UnitError):
    """Raised when angle units are wrong (degrees vs radians)."""
    
    def __init__(self, expected: str, got: str):
        message = f"Expected {expected}, got {got}.\n" \
                  f"Hint: Use Angle.from_degrees() or Angle.from_radians() explicitly."
        context = {
            "expected": expected,
            "got": got,
        }
        MathLibError.__init__(self, message, context)


class SingularMatrixError(MathLibError):
    """Raised when attempting to invert a singular matrix."""
    
    def __init__(self, determinant=None):
        context = {}
        if determinant is not None:
            context["determinant"] = determinant
        
        message = "Cannot invert singular matrix (determinant = 0)"
        super().__init__(message, context)


class InvalidOperationError(MathLibError):
    """Raised when an operation is mathematically invalid."""
    
    def __init__(self, operation: str, reason: str):
        message = f"Invalid operation '{operation}': {reason}"
        context = {"operation": operation, "reason": reason}
        super().__init__(message, context)


class GeometryError(MathLibError):
    """Raised for impossible geometric configurations."""
    
    def __init__(self, message: str, geometric_object=None):
        context = {}
        if geometric_object is not None:
            context["object"] = str(geometric_object)
        super().__init__(message, context)


class ParallelLinesError(GeometryError):
    """Raised when lines are parallel but shouldn't be."""
    
    def __init__(self):
        super().__init__("Lines are parallel and do not intersect")


class CoincidentLinesError(GeometryError):
    """Raised when lines are coincident."""
    
    def __init__(self):
        super().__init__("Lines are coincident (infinite intersections)")


class ZeroVectorError(MathLibError):
    """Raised when a zero vector is used where it's invalid."""
    
    def __init__(self, context_msg: str = None):
        message = "Zero vector is invalid in this context"
        if context_msg:
            message += f": {context_msg}"
        super().__init__(message)