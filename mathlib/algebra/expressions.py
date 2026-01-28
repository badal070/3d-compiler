"""
Expressions - Symbolic expression trees.

No eval(). Ever. That's how demons enter.
"""

from dataclasses import dataclass
from abc import ABC, abstractmethod
from typing import Dict


class Expression(ABC):
    """Abstract expression node."""
    
    @abstractmethod
    def evaluate(self, context: Dict[str, float]) -> float:
        """Evaluate expression with given variable values."""
        pass
    
    @abstractmethod
    def __str__(self) -> str:
        pass


@dataclass(frozen=True)
class Variable(Expression):
    """Symbolic variable."""
    
    name: str
    
    def evaluate(self, context: Dict[str, float]) -> float:
        if self.name not in context:
            raise ValueError(f"Variable '{self.name}' not in context")
        return context[self.name]
    
    def __str__(self) -> str:
        return self.name


@dataclass(frozen=True)
class Constant(Expression):
    """Numeric constant."""
    
    value: float
    
    def evaluate(self, context: Dict[str, float]) -> float:
        return self.value
    
    def __str__(self) -> str:
        return str(self.value)


@dataclass(frozen=True)
class BinaryOp(Expression):
    """Binary operation."""
    
    left: Expression
    op: str
    right: Expression
    
    def evaluate(self, context: Dict[str, float]) -> float:
        l = self.left.evaluate(context)
        r = self.right.evaluate(context)
        
        if self.op == '+':
            return l + r
        elif self.op == '-':
            return l - r
        elif self.op == '*':
            return l * r
        elif self.op == '/':
            if r == 0:
                raise ZeroDivisionError()
            return l / r
        else:
            raise ValueError(f"Unknown operator: {self.op}")
    
    def __str__(self) -> str:
        return f"({self.left} {self.op} {self.right})"