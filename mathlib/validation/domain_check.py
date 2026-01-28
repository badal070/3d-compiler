"""
Domain validation for mathematical operations.
"""

from mathlib.errors.validation_errors import DomainError


def check_domain(value: float, domain_min: float = None, domain_max: float = None,
                 parameter: str = None):
    """
    Check if value is within valid domain.
    """
    if domain_min is not None and value < domain_min:
        domain_str = f"[{domain_min}, " + (f"{domain_max}]" if domain_max else "∞)")
        raise DomainError(value, domain_str, parameter)
    
    if domain_max is not None and value > domain_max:
        domain_str = ("(-∞, " if domain_min is None else f"[{domain_min}, ") + f"{domain_max}]"
        raise DomainError(value, domain_str, parameter)


def check_positive(value: float, parameter: str = None):
    """Check if value is positive."""
    check_domain(value, 0.0, None, parameter)


def check_unit_interval(value: float, parameter: str = None):
    """Check if value is in [0, 1]."""
    check_domain(value, 0.0, 1.0, parameter)