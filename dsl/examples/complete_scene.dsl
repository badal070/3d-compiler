// Complete Scene Example
// This demonstrates all major DSL features

// 1. SCENE HEADER (required, always first)
scene {
  name: "Mechanical System Demo"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

// 2. LIBRARY IMPORTS (required, always second)
library_imports {
  math: "core_mechanics"
  geometry: "basic_solids"
  gears: "gear_systems"
}

// 3. ENTITIES (optional, define physical objects)

// Main gear - drives the system
entity main_gear {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]        // radians
      scale: [2, 0.5, 2]         // meters (SI)
    }
    
    geometry {
      primitive: cylinder
    }
    
    physical {
      mass: 5.0                  // kilograms (SI)
      rigid: true
    }
  }
}

// Secondary gear - driven by main gear
entity secondary_gear {
  kind: solid
  components {
    transform {
      position: [6, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 0.5, 1]
    }
    
    geometry {
      primitive: cylinder
    }
    
    physical {
      mass: 2.5
      rigid: true
    }
  }
}

// Output shaft
entity output_shaft {
  kind: solid
  components {
    transform {
      position: [6, 0, 2]
      rotation: [1.5708, 0, 0]   // 90 degrees in radians
      scale: [0.2, 3, 0.2]
    }
    
    geometry {
      primitive: cylinder
    }
    
    physical {
      mass: 1.0
      rigid: true
    }
  }
}

// Fixed base
entity base_platform {
  kind: solid
  components {
    transform {
      position: [0, -1, 0]
      rotation: [0, 0, 0]
      scale: [10, 0.2, 10]
    }
    
    geometry {
      primitive: cube
    }
    
    physical {
      mass: 50.0
      rigid: true
    }
  }
}

// 4. CONSTRAINTS (optional, define relationships)

// Main gear fixed to base
constraint main_gear_anchor {
  type: fixed_joint
  parent: base_platform
  child: main_gear
}

// Gear coupling
constraint gear_coupling {
  type: gear_relation
  driver: main_gear
  driven: secondary_gear
  ratio: 2.0                     // secondary rotates 2x faster
}

// Output shaft attached to secondary gear
constraint shaft_connection {
  type: fixed_joint
  parent: secondary_gear
  child: output_shaft
}

// 5. MOTIONS (optional, define behaviors)

// Drive the main gear
motion drive_main_gear {
  target: main_gear
  type: rotation
  axis: [0, 1, 0]                // Y-axis (normalized)
  speed: 3.14159                 // radians per second (π rad/s)
}

// Alternative motion example (not used in timeline)
motion oscillate_platform {
  target: base_platform
  type: translation
  direction: [0, 1, 0]
  speed: 0.5
}

// 6. TIMELINES (optional, orchestrate when things happen)

timeline main_sequence {
  // Start driving the main gear immediately
  event {
    motion: drive_main_gear
    start: 0.0                   // seconds
    duration: 10.0               // seconds
  }
}

// Alternative timeline (demonstrates multiple timelines)
timeline test_sequence {
  event {
    motion: drive_main_gear
    start: 0.0
    duration: 5.0
  }
  
  event {
    motion: oscillate_platform
    start: 5.0
    duration: 3.0
  }
}

// END OF FILE
// 
// Notes:
// - Comments use // syntax
// - All rotation angles in RADIANS (not degrees!)
// - All positions/scales in meters for SI
// - All masses in kilograms for SI
// - All times in seconds
// - Vectors must be exactly 3 components [x, y, z]
// - Rotation axes must be normalized (magnitude = 1.0)
// - No expressions allowed (3.14159, not 3*PI or PI)
// - No forward references
// - Strict ordering: scene → imports → entities → constraints → motions → timelines