use nova_ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

pub struct SemanticAnalyzer {
    scenes: HashSet<String>,
    states: HashSet<String>,
    materials: HashSet<String>,
    components: HashSet<String>,
    diagnostics: Vec<Diagnostic>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scenes: HashSet::new(),
            states: HashSet::new(),
            materials: HashSet::new(),
            components: HashSet::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Vec<Diagnostic> {
        self.diagnostics.clear();

        // Pass 1: Register top-level symbols
        for decl in &program.declarations {
            match decl {
                Declaration::Scene(scene) => {
                    if !self.scenes.insert(scene.name.clone()) {
                        self.diagnostics.push(Diagnostic {
                            level: DiagnosticLevel::Error,
                            code: "N301".to_string(),
                            message: format!("Duplicate scene declaration '{}'.", scene.name),
                            line: scene.span.line,
                            column: scene.span.column,
                        });
                    }
                }
                Declaration::State(state) => {
                    if !self.states.insert(state.name.clone()) {
                        self.diagnostics.push(Diagnostic {
                            level: DiagnosticLevel::Error,
                            code: "N302".to_string(),
                            message: format!("Duplicate state declaration '{}'.", state.name),
                            line: state.span.line,
                            column: state.span.column,
                        });
                    }
                }
                Declaration::Material(mat) => {
                    self.materials.insert(mat.name.clone());
                }
                Declaration::Component(comp) => {
                    self.components.insert(comp.name.clone());
                }
                Declaration::Page(_) => {}
            }
        }

        // Pass 2: Validate references
        for decl in &program.declarations {
            if let Declaration::Page(page) = decl {
                for item in &page.items {
                    if let PageItem::SceneRef(scene_name) = item {
                        if !self.scenes.contains(scene_name) && !self.components.contains(scene_name) {
                            self.diagnostics.push(Diagnostic {
                                level: DiagnosticLevel::Warning,
                                code: "N303".to_string(),
                                message: format!("Unresolved scene or component reference '{}' in page '{}'.", scene_name, page.name),
                                line: page.span.line,
                                column: page.span.column,
                            });
                        }
                    }
                }
            }
        }

        self.diagnostics.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_scene_detection() {
        let program = Program {
            declarations: vec![
                Declaration::Scene(SceneDecl {
                    name: "Demo".to_string(),
                    cameras: vec![],
                    environment: None,
                    lights: vec![],
                    models: vec![],
                    meshes: vec![],
                    scroll_animations: vec![],
                    span: Span::new(1, 1),
                }),
                Declaration::Scene(SceneDecl {
                    name: "Demo".to_string(),
                    cameras: vec![],
                    environment: None,
                    lights: vec![],
                    models: vec![],
                    meshes: vec![],
                    scroll_animations: vec![],
                    span: Span::new(10, 1),
                }),
            ],
        };

        let mut analyzer = SemanticAnalyzer::new();
        let diagnostics = analyzer.analyze(&program);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "N301");
    }
}
