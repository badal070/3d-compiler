"""
Validation-specific errors for rejecting bad scenes before rendering.

These errors catch problems in the DSL → Math → Renderer pipeline.
"""

from mathlib.errors.math_errors import MathLibError


class ValidationError(MathLibError):
    """Base class for validation failures."""
    
    def __init__(self, message: str, failed_check: str = None, context: dict = None):
        ctx = context or {}
        if failed_check:
            ctx["failed_check"] = failed_check
        super().__init__(message, ctx)


class DomainError(ValidationError):
    """Raised when a value is outside its valid domain."""
    
    def __init__(self, value, valid_domain: str, parameter: str = None):
        context = {
            "value": value,
            "valid_domain": valid_domain,
        }
        if parameter:
            context["parameter"] = parameter
        
        message = f"Value {value} is outside valid domain: {valid_domain}"
        if parameter:
            message = f"Parameter '{parameter}': {message}"
        
        super().__init__(message, "domain_check", context)


class InvariantError(ValidationError):
    """Raised when a mathematical invariant is violated."""
    
    def __init__(self, invariant_name: str, description: str, context: dict = None):
        message = f"Invariant '{invariant_name}' violated: {description}"
        ctx = context or {}
        ctx["invariant"] = invariant_name
        super().__init__(message, "invariant_check", ctx)


class UndefinedTransformError(ValidationError):
    """Raised when a transform is undefined or invalid."""
    
    def __init__(self, transform_name: str, reason: str):
        message = f"Transform '{transform_name}' is undefined: {reason}"
        context = {
            "transform": transform_name,
            "reason": reason,
        }
        super().__init__(message, "transform_check", context)


class ImpossibleGeometryError(ValidationError):
    """Raised when geometric constraints cannot be satisfied."""
    
    def __init__(self, description: str, constraints: list = None):
        context = {}
        if constraints:
            context["violated_constraints"] = constraints
        
        message = f"Impossible geometry: {description}"
        super().__init__(message, "geometry_check", context)


class CircularDependencyError(ValidationError):
    """Raised when kinematic chains have circular dependencies."""
    
    def __init__(self, chain: list):
        message = f"Circular dependency detected in kinematic chain: {' → '.join(map(str, chain))}"
        context = {"chain": chain}
        super().__init__(message, "dependency_check", context)


class MissingFrameError(ValidationError):
    """Raised when a required reference frame doesn't exist."""
    
    def __init__(self, frame_name: str, parent_frame: str = None):
        context = {"missing_frame": frame_name}
        if parent_frame:
            context["parent_frame"] = parent_frame
        
        message = f"Required frame '{frame_name}' does not exist"
        if parent_frame:
            message += f" in parent frame '{parent_frame}'"
        
        super().__init__(message, "frame_check", context)