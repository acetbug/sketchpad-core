pub struct SketchpadCore {
    geometry: Vec<Geometry>,
    tools: Vec<Tool>,
    oprations: Vec<Operation>,
}

impl SketchpadCore {
    pub fn new() -> Self {
        Self {
            geometry: Vec::new(),
            tools: Vec::new(),
            oprations: Vec::new(),
        }
    }

    pub fn add_geometry(&mut self, geometry: Geometry) {
        self.geometry.push(geometry);
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn add_operation(&mut self, operation: Operation) {
        self.oprations.push(operation);
    }
}
