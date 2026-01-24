"""
Tensor - Multi-dimensional array with strict dimension tracking.

For advanced geometric and physics computations.
"""

from dataclasses import dataclass
from typing import List, Tuple
import math

from mathlib.errors.math_errors import DimensionError, InvalidOperationError


@dataclass(frozen=True)
class Tensor:
    """Immutable n-dimensional tensor."""
    
    elements: tuple
    shape: Tuple[int, ...]
    rank: int
    
    def __init__(self, elements: list, shape: Tuple[int, ...] = None):
        """
        Create a tensor.
        
        Args:
            elements: Nested list structure
            shape: Explicit shape tuple, auto-detected if None
        """
        # Flatten and convert to tuple
        flat, detected_shape = self._flatten_and_detect_shape(elements)
        
        if shape is None:
            shape = detected_shape
        
        # Validate shape
        expected_size = 1
        for dim in shape:
            expected_size *= dim
        
        if len(flat) != expected_size:
            raise ValueError(f"Shape {shape} requires {expected_size} elements, got {len(flat)}")
        
        object.__setattr__(self, 'elements', tuple(flat))
        object.__setattr__(self, 'shape', shape)
        object.__setattr__(self, 'rank', len(shape))
    
    def _flatten_and_detect_shape(self, elements: list) -> Tuple[list, tuple]:
        """Flatten nested list and detect shape."""
        if not isinstance(elements, list):
            return [float(elements)], ()
        
        if not elements:
            return [], (0,)
        
        # Check if all elements are numeric
        if all(not isinstance(e, list) for e in elements):
            return [float(e) for e in elements], (len(elements),)
        
        # Recursively flatten
        flattened = []
        shapes = []
        
        for elem in elements:
            flat, shape = self._flatten_and_detect_shape(elem)
            flattened.extend(flat)
            shapes.append(shape)
        
        # Verify all sub-shapes are the same
        if not all(s == shapes[0] for s in shapes):
            raise ValueError("Inconsistent tensor dimensions")
        
        combined_shape = (len(elements),) + shapes[0]
        return flattened, combined_shape
    
    def __str__(self) -> str:
        return f"Tensor{self.shape}"
    
    def __repr__(self) -> str:
        return f"Tensor(shape={self.shape}, rank={self.rank})"
    
    def __getitem__(self, indices: Tuple[int, ...]) -> float:
        """Access element at multi-dimensional index."""
        if len(indices) != self.rank:
            raise IndexError(f"Expected {self.rank} indices, got {len(indices)}")
        
        # Convert multi-dimensional index to flat index
        flat_idx = 0
        stride = 1
        
        for i in range(self.rank - 1, -1, -1):
            if indices[i] < 0 or indices[i] >= self.shape[i]:
                raise IndexError(f"Index {indices[i]} out of range for dimension {i}")
            flat_idx += indices[i] * stride
            stride *= self.shape[i]
        
        return self.elements[flat_idx]
    
    @staticmethod
    def zeros(shape: Tuple[int, ...]) -> 'Tensor':
        """Create tensor of zeros."""
        size = 1
        for dim in shape:
            size *= dim
        
        flat = [0.0] * size
        return Tensor(flat, shape)
    
    @staticmethod
    def ones(shape: Tuple[int, ...]) -> 'Tensor':
        """Create tensor of ones."""
        size = 1
        for dim in shape:
            size *= dim
        
        flat = [1.0] * size
        return Tensor(flat, shape)