export const SAMPLE_DSL = `scene {
  name: "Simple Visualization"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

entity cube1 {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]
      scale: [2, 2, 2]
    }
    geometry {
      primitive: cube
    }
  }
}

entity sphere1 {
  kind: solid
  components {
    transform {
      position: [4, 0, 0]
      rotation: [0, 0, 0]
      scale: [1.5, 1.5, 1.5]
    }
    geometry {
      primitive: sphere
    }
  }
}

entity cylinder1 {
  kind: solid
  components {
    transform {
      position: [-3, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 2, 1]
    }
    geometry {
      primitive: cylinder
    }
  }
}`;

export const SAMPLE_DSL_SIMPLE = `entity box {
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
  }
}`;