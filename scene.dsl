scene {
name: "Cube Demo"
version: 1
ir_version: "0.1.0"
unit_system: "SI"
}
library_imports {
math: "core_mechanics"
geometry: "basic_solids"
}
entity cube {
kind: solid
components {
transform {
position: [0, 0, 0]
rotation: [0, 0, 0]
scale: [1, 1, 1]
}
geometry {
primitive: cube
}
physical {
mass: 1.0
rigid: true
}
}
}
motion rotate_cube {
target: cube
type: rotation
axis: [0, 1, 0]
speed: 1.5708
}
timeline main {
event {
motion: rotate_cube
start: 0.0
duration: 10.0
}
}
