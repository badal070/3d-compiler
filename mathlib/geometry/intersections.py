"""
Intersections - Geometric intersection operations.

Returns: Point, Line, or EmptySet. Never None.
"""

from dataclasses import dataclass
from typing import Union
from mathlib.geometry.point import Point
from mathlib.geometry.line import Line
from mathlib.geometry.plane import Plane
from mathlib.core.vector import Vector
from mathlib.errors.math_errors import (
    ParallelLinesError,
    CoincidentLinesError,
    InvalidOperationError
)


@dataclass(frozen=True)
class EmptySet:
    """Represents no intersection."""
    
    reason: str = "No intersection exists"
    
    def __str__(self) -> str:
        return f"∅ ({self.reason})"
    
    def __bool__(self) -> bool:
        return False


@dataclass(frozen=True)
class IntersectionResult:
    """Result of an intersection operation."""
    
    result: Union[Point, Line, EmptySet]
    
    def __init__(self, result: Union[Point, Line, EmptySet]):
        object.__setattr__(self, 'result', result)
    
    def __str__(self) -> str:
        return str(self.result)
    
    def __bool__(self) -> bool:
        return not isinstance(self.result, EmptySet)
    
    @property
    def exists(self) -> bool:
        """Check if intersection exists."""
        return not isinstance(self.result, EmptySet)
    
    @property
    def is_point(self) -> bool:
        """Check if result is a point."""
        return isinstance(self.result, Point)
    
    @property
    def is_line(self) -> bool:
        """Check if result is a line."""
        return isinstance(self.result, Line)
    
    @property
    def is_empty(self) -> bool:
        """Check if result is empty."""
        return isinstance(self.result, EmptySet)


def intersect_line_line(line1: Line, line2: Line, tolerance: float = 1e-9) -> IntersectionResult:
    """
    Compute intersection of two lines.
    
    Returns:
        - Point: if lines intersect at a point
        - Line: if lines are coincident
        - EmptySet: if lines are parallel but not coincident
    """
    if not isinstance(line1, Line) or not isinstance(line2, Line):
        raise TypeError("Both arguments must be Lines")
    
    # Check if coincident
    if line1.is_coincident(line2, tolerance):
        return IntersectionResult(line1)
    
    # Check if parallel
    if line1.is_parallel(line2, tolerance):
        return IntersectionResult(EmptySet("Lines are parallel"))
    
    # For 2D lines: solve parametric equations
    if line1.dimension == 2:
        # line1: P1 + t * d1
        # line2: P2 + s * d2
        # P1 + t * d1 = P2 + s * d2
        
        p1 = line1.point.position
        p2 = line2.point.position
        d1 = line1.direction
        d2 = line2.direction
        
        # [d1.x, -d2.x] [t]   [p2.x - p1.x]
        # [d1.y, -d2.y] [s] = [p2.y - p1.y]
        
        det = d1[0] * (-d2[1]) - d1[1] * (-d2[0])
        
        if abs(det) < tolerance:
            return IntersectionResult(EmptySet("Lines are parallel"))
        
        dx = p2[0] - p1[0]
        dy = p2[1] - p1[1]
        
        t = (dx * (-d2[1]) - dy * (-d2[0])) / det
        
        intersection_point = line1.point_at(t)
        return IntersectionResult(intersection_point)
    
    # For 3D lines: check if skew or intersecting
    elif line1.dimension == 3:
        # Lines in 3D may be skew (non-parallel, non-intersecting)
        p1 = line1.point.position
        p2 = line2.point.position
        d1 = line1.direction
        d2 = line2.direction
        
        # Check if coplanar
        w = p2 - p1
        cross_d = d1.cross(d2)
        
        # If (p2 - p1) · (d1 × d2) ≠ 0, lines are skew
        coplanar_test = w.dot(cross_d).value
        
        if abs(coplanar_test) > tolerance:
            return IntersectionResult(EmptySet("Lines are skew (not coplanar)"))
        
        # Lines are coplanar, find intersection
        # Solve: p1 + t*d1 = p2 + s*d2
        
        # Use cross product method
        w_cross_d2 = w.cross(d2)
        cross_norm_sq = cross_d.dot(cross_d).value
        
        if abs(cross_norm_sq) < tolerance:
            return IntersectionResult(EmptySet("Lines are parallel"))
        
        t = w_cross_d2.dot(cross_d).value / cross_norm_sq
        
        intersection_point = line1.point_at(t)
        return IntersectionResult(intersection_point)
    
    else:
        raise InvalidOperationError(
            "line-line intersection",
            f"not implemented for dimension {line1.dimension}"
        )


def intersect_line_plane(line: Line, plane: Plane, tolerance: float = 1e-9) -> IntersectionResult:
    """
    Compute intersection of line and plane.
    
    Returns:
        - Point: if line intersects plane at a point
        - Line: if line lies in the plane
        - EmptySet: if line is parallel to plane
    """
    if not isinstance(line, Line) or not isinstance(plane, Plane):
        raise TypeError("Arguments must be Line and Plane")
    
    if line.dimension != 3:
        raise InvalidOperationError(
            "line-plane intersection",
            "only defined for 3D"
        )
    
    # Check if line direction is perpendicular to plane normal
    dot_product = line.direction.dot(plane.normal).value
    
    # If dot product is zero, line is parallel to plane
    if abs(dot_product) < tolerance:
        # Check if line lies in plane
        if plane.contains_point(line.point, tolerance):
            return IntersectionResult(line)
        else:
            return IntersectionResult(EmptySet("Line is parallel to plane"))
    
    # Compute intersection point
    # (point_on_line - point_on_plane) · normal = t * direction · normal
    diff = line.point.position - plane.point.position
    t = -diff.dot(plane.normal).value / dot_product
    
    intersection_point = line.point_at(t)
    return IntersectionResult(intersection_point)


def intersect_plane_plane(plane1: Plane, plane2: Plane, tolerance: float = 1e-9) -> IntersectionResult:
    """
    Compute intersection of two planes.
    
    Returns:
        - Line: if planes intersect in a line
        - Plane: if planes are coincident
        - EmptySet: if planes are parallel
    """
    if not isinstance(plane1, Plane) or not isinstance(plane2, Plane):
        raise TypeError("Both arguments must be Planes")
    
    # Check if coincident
    if plane1.is_coincident(plane2, tolerance):
        return IntersectionResult(plane1)
    
    # Check if parallel
    if plane1.is_parallel(plane2, tolerance):
        return IntersectionResult(EmptySet("Planes are parallel"))
    
    # Intersection is a line
    # Direction of line is perpendicular to both normals
    direction = plane1.normal.cross(plane2.normal)
    
    # Find a point on the intersection line
    # Use the point closest to the origin
    
    # Set up system: find point p such that
    # (p - p1) · n1 = 0
    # (p - p2) · n2 = 0
    
    # We need one more constraint; use direction to parameterize
    # Find the component where |direction| is largest
    abs_components = [abs(c) for c in direction.components]
    max_idx = abs_components.index(max(abs_components))
    
    # Set that component to 0 for the point we're finding
    # This gives us the point on the line closest to the origin in that plane
    
    n1 = plane1.normal
    n2 = plane2.normal
    d1 = -plane1.point.position.dot(n1).value
    d2 = -plane2.point.position.dot(n2).value
    
    # Solve 2D system for the other two components
    if max_idx == 0:  # x is free, solve for y and z
        # n1.y * y + n1.z * z = -d1
        # n2.y * y + n2.z * z = -d2
        det = n1[1] * n2[2] - n1[2] * n2[1]
        if abs(det) < tolerance:
            # Fallback
            point_components = [0.0, 0.0, 0.0]
        else:
            y = (-d1 * n2[2] + d2 * n1[2]) / det
            z = (n1[1] * (-d2) - n2[1] * (-d1)) / det
            point_components = [0.0, y, z]
    elif max_idx == 1:  # y is free, solve for x and z
        det = n1[0] * n2[2] - n1[2] * n2[0]
        if abs(det) < tolerance:
            point_components = [0.0, 0.0, 0.0]
        else:
            x = (-d1 * n2[2] + d2 * n1[2]) / det
            z = (n1[0] * (-d2) - n2[0] * (-d1)) / det
            point_components = [x, 0.0, z]
    else:  # z is free, solve for x and y
        det = n1[0] * n2[1] - n1[1] * n2[0]
        if abs(det) < tolerance:
            point_components = [0.0, 0.0, 0.0]
        else:
            x = (-d1 * n2[1] + d2 * n1[1]) / det
            y = (n1[0] * (-d2) - n2[0] * (-d1)) / det
            point_components = [x, y, 0.0]
    
    point = Point(point_components, plane1.point.position.unit)
    intersection_line = Line(point, direction)
    
    return IntersectionResult(intersection_line)


def intersect(obj1, obj2, tolerance: float = 1e-9) -> IntersectionResult:
    """
    Generic intersection dispatcher.
    
    Automatically determines intersection type based on object types.
    """
    if isinstance(obj1, Line) and isinstance(obj2, Line):
        return intersect_line_line(obj1, obj2, tolerance)
    elif isinstance(obj1, Line) and isinstance(obj2, Plane):
        return intersect_line_plane(obj1, obj2, tolerance)
    elif isinstance(obj1, Plane) and isinstance(obj2, Line):
        return intersect_line_plane(obj2, obj1, tolerance)
    elif isinstance(obj1, Plane) and isinstance(obj2, Plane):
        return intersect_plane_plane(obj1, obj2, tolerance)
    else:
        raise NotImplementedError(
            f"Intersection not implemented for {type(obj1).__name__} and {type(obj2).__name__}"
        )