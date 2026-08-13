use nova_ast::*;

#[derive(Debug, Clone)]
pub struct CompiledBundle {
    pub html: String,
    pub css: String,
    pub js: String,
}

pub struct CodeGenerator {
    html: String,
    css: String,
    js: String,
    states: std::collections::HashSet<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            html: String::new(),
            css: String::new(),
            js: String::new(),
            states: std::collections::HashSet::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> CompiledBundle {
        self.html.clear();
        self.css.clear();
        self.js.clear();
        self.states.clear();

        // Standard CSS Base
        self.css.push_str(
            r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0a0a0f; color: #ffffff; overflow-x: hidden; }
.nova-app-container { relative; width: 100vw; min-height: 100vh; }
.nova-canvas-container { position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 1; pointer-events: auto; }
.nova-ui-layer { position: relative; z-index: 10; pointer-events: none; width: 100%; min-height: 100vh; }
.nova-ui-layer * { pointer-events: auto; }
.nova-hero-section { min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 2rem; }
.nova-heading { font-size: 4rem; font-weight: 800; background: linear-gradient(135deg, #ffffff 0%, #a5b4fc 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin-bottom: 1.5rem; }
.nova-text { font-size: 1.25rem; color: #94a3b8; max-width: 600px; margin-bottom: 2rem; line-height: 1.6; }
.nova-button { padding: 1rem 2.5rem; font-size: 1.1rem; font-weight: 600; color: #ffffff; background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%); border: none; border-radius: 9999px; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 10px 25px -5px rgba(99, 102, 241, 0.4); }
.nova-button:hover { transform: translateY(-2px) scale(1.03); box-shadow: 0 15px 30px -5px rgba(99, 102, 241, 0.6); }
.nova-card { background: rgba(255, 255, 255, 0.05); backdrop-filter: blur(16px); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 1.5rem; padding: 2rem; margin: 1rem; }
"#,
        );

        // JS Module Header
        self.js.push_str(
            r#"
import { NovaEngine } from './nova-runtime.js';

document.addEventListener('DOMContentLoaded', async () => {
    const engine = new NovaEngine({
        canvasContainer: document.getElementById('nova-canvas-container')
    });
    window.__NOVA_ENGINE__ = engine;
"#,
        );

        // Generate declarations
        for decl in &program.declarations {
            match decl {
                Declaration::State(state) => self.generate_state(state),
                Declaration::Scene(scene) => self.generate_scene(scene),
                Declaration::Page(page) => self.generate_page(page),
                Declaration::Material(mat) => self.generate_material(mat),
                Declaration::Component(_) => {}
            }
        }

        self.js.push_str("\n    engine.start();\n});\n");

        // Assemble Final HTML document
        let html_output = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NOVA 3D Experience</title>
    <style>{}</style>
</head>
<body>
    <div id="nova-canvas-container" class="nova-canvas-container"></div>
    <div id="nova-ui-layer" class="nova-ui-layer">
        {}
    </div>
    <script type="module" src="./main.js"></script>
</body>
</html>"#,
            self.css, self.html
        );

        CompiledBundle {
            html: html_output,
            css: self.css.clone(),
            js: self.js.clone(),
        }
    }

    fn generate_state(&mut self, state: &StateDecl) {
        self.states.insert(state.name.clone());
        let val_str = self.compile_expr(&state.initial_value);
        self.js.push_str(&format!(
            "    engine.createState('{}', {});\n",
            state.name, val_str
        ));
    }

    fn generate_material(&mut self, mat: &MaterialDecl) {
        self.js.push_str(&format!(
            "    engine.registerMaterial('{}', {{ type: '{}', ",
            mat.name, mat.material_type
        ));
        for (k, v) in &mat.properties {
            self.js
                .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
        }
        self.js.push_str("});\n");
    }

    fn generate_scene(&mut self, scene: &SceneDecl) {
        self.js.push_str(&format!(
            "    const scene_{} = engine.createScene('{}');\n",
            scene.name, scene.name
        ));

        if let Some(ref env) = scene.environment {
            self.js.push_str(&format!("    scene_{}.setEnvironment({{ ", scene.name));
            for (k, v) in &env.properties {
                self.js
                    .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
            }
            self.js.push_str("});\n");
        }

        for cam in &scene.cameras {
            self.js.push_str(&format!(
                "    scene_{}.addCamera('{}', {{ ",
                scene.name, cam.name
            ));
            for (k, v) in &cam.properties {
                self.js
                    .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
            }
            self.js.push_str("});\n");
        }

        for light in &scene.lights {
            self.js.push_str(&format!(
                "    scene_{}.addLight('{}', {{ ",
                scene.name, light.name
            ));
            for (k, v) in &light.properties {
                self.js
                    .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
            }
            self.js.push_str("});\n");
        }

        for model in &scene.models {
            let src = model.source.as_deref().unwrap_or("./assets/model.glb");
            self.js.push_str(&format!(
                "    const model_{} = scene_{}.addModel('{}', '{}', {{ ",
                model.name, scene.name, model.name, src
            ));
            for (k, v) in &model.properties {
                self.js
                    .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
            }
            self.js.push_str("});\n");

            if let Some(ref stmts) = model.animate {
                self.js.push_str(&format!(
                    "    engine.onUpdate((delta, time) => {{\n"
                ));
                for stmt in stmts {
                    if let Stmt::Assignment { target, op, value } = stmt {
                        let t_str = target.join(".");
                        let val_str = self.compile_expr(value);
                        self.js.push_str(&format!(
                            "        model_{}.{} {} {};\n",
                            model.name, t_str, op, val_str
                        ));
                    }
                }
                self.js.push_str("    });\n");
            }

            for interaction in &model.interactions {
                self.js.push_str(&format!(
                    "    model_{}.on('{}', () => {{\n",
                    model.name, interaction.event
                ));
                for stmt in &interaction.statements {
                    match stmt {
                        Stmt::Assignment { target, op, value } => {
                            let t_str = target.join(".");
                            let val_str = self.compile_expr(value);
                            self.js.push_str(&format!(
                                "        model_{}.{} {} {};\n",
                                model.name, t_str, op, val_str
                            ));
                        }
                        Stmt::Action { name, args } => {
                            if name == "open" || name == "navigate" {
                                if let Some(arg) = args.first() {
                                    self.js.push_str(&format!(
                                        "        window.location.href = {};\n",
                                        self.compile_expr(arg)
                                    ));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                self.js.push_str("    });\n");
            }
        }

        for mesh in &scene.meshes {
            self.js.push_str(&format!(
                "    scene_{}.addMesh('{}', '{}', '{}', {{ ",
                scene.name, mesh.name, mesh.geometry, mesh.material
            ));
            for (k, v) in &mesh.properties {
                self.js
                    .push_str(&format!("{}: {}, ", k, self.compile_expr(v)));
            }
            self.js.push_str("});\n");
        }

        for scroll_anim in &scene.scroll_animations {
            self.js.push_str("    engine.onScroll((progress) => {\n");
            for keyframe in &scroll_anim.keyframes {
                self.js.push_str(&format!(
                    "        if (progress >= {}) {{\n",
                    keyframe.percentage / 100.0
                ));
                for stmt in &keyframe.statements {
                    if let Stmt::Assignment { target, op: _op, value } = stmt {
                        let t_str = target.join(".");
                        let val_str = self.compile_expr(value);
                        self.js.push_str(&format!(
                            "            engine.setTargetProperty('{}', {});\n",
                            t_str, val_str
                        ));
                    }
                }
                self.js.push_str("        }\n");
            }
            self.js.push_str("    });\n");
        }
    }

    fn generate_page(&mut self, page: &PageDecl) {
        self.html
            .push_str(&format!("<div class=\"nova-page\" id=\"page-{}\">\n", page.name));
        for item in &page.items {
            match item {
                PageItem::UIComponent(comp) => self.generate_ui_component(comp),
                PageItem::SceneRef(scene_name) => {
                    self.js
                        .push_str(&format!("    engine.setActiveScene('{}');\n", scene_name));
                }
            }
        }
        self.html.push_str("</div>\n");
    }

    fn generate_ui_component(&mut self, comp: &UIComponentItem) {
        let tag = match comp.kind.as_str() {
            "section" => "section",
            "heading" => "h1",
            "text" => "p",
            "button" => "button",
            "card" => "div",
            "nav" => "nav",
            "link" => "a",
            _ => "div",
        };

        let class_name = format!("nova-{}", comp.kind);
        self.html.push_str(&format!("<{} class=\"{}\">", tag, class_name));

        if let Some(ref text) = comp.content {
            self.html.push_str(text);
        }

        for child in &comp.children {
            match child {
                PageItem::UIComponent(c) => self.generate_ui_component(c),
                PageItem::SceneRef(s) => {
                    self.js.push_str(&format!("    engine.setActiveScene('{}');\n", s));
                }
            }
        }

        self.html.push_str(&format!("</{}>\n", tag));
    }

    fn compile_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => match lit {
                LiteralValue::Number(val, unit) => match unit {
                    Unit::Deg => format!("({} * Math.PI / 180)", val),
                    Unit::Rad => format!("{}", val),
                    Unit::Sec => format!("{}", val * 1000.0),
                    Unit::Ms => format!("{}", val),
                    _ => format!("{}", val),
                },
                LiteralValue::String(s) => format!("\"{}\"", s),
                LiteralValue::Boolean(b) => format!("{}", b),
                LiteralValue::Color(c) => format!("\"{}\"", c),
            },
            Expr::Vector(x, y, z) => format!(
                "{{ x: {}, y: {}, z: {} }}",
                self.compile_expr(x),
                self.compile_expr(y),
                self.compile_expr(z)
            ),
            Expr::Identifier(id) => {
                if self.states.contains(id) {
                    format!("engine.getState('{}')", id)
                } else {
                    match id.as_str() {
                        "directional" | "ambient" | "point" | "spot" | "perspective" | "orthographic" | "studio" | "physical" | "standard" => {
                            format!("\"{}\"", id)
                        }
                        _ => id.clone(),
                    }
                }
            },
            Expr::MemberAccess(parent, member) => {
                format!("{}.{}", self.compile_expr(parent), member)
            }
            Expr::BinaryOp(left, op, right) => format!(
                "({} {} {})",
                self.compile_expr(left),
                op,
                self.compile_expr(right)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_basic() {
        let program = Program {
            declarations: vec![
                Declaration::State(StateDecl {
                    name: "rotation".to_string(),
                    initial_value: Expr::Literal(LiteralValue::Number(0.0, Unit::Deg)),
                    span: Span::new(1, 1),
                }),
                Declaration::Page(PageDecl {
                    name: "Home".to_string(),
                    items: vec![PageItem::UIComponent(UIComponentItem {
                        kind: "heading".to_string(),
                        name: "title".to_string(),
                        content: Some("Hello 3D NOVA".to_string()),
                        properties: vec![],
                        children: vec![],
                        events: vec![],
                        span: Span::new(2, 1),
                    })],
                    span: Span::new(2, 1),
                }),
            ],
        };

        let mut codegen = CodeGenerator::new();
        let bundle = codegen.generate(&program);
        assert!(bundle.html.contains("Hello 3D NOVA"));
        assert!(bundle.js.contains("engine.createState('rotation'"));
    }
}
