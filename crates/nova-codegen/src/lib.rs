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
    elem_counter: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            html: String::new(),
            css: String::new(),
            js: String::new(),
            states: std::collections::HashSet::new(),
            elem_counter: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> CompiledBundle {
        self.html.clear();
        self.css.clear();
        self.js.clear();
        self.states.clear();
        self.elem_counter = 0;

        // Modern Glassmorphic CSS Engine
        self.css.push_str(
            r#"
@import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap');
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: 'Plus Jakarta Sans', system-ui, -apple-system, sans-serif; background: #06060c; color: #f8fafc; overflow-x: hidden; }
.nova-canvas-container { position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 1; pointer-events: auto; }
.nova-ui-layer { position: relative; z-index: 10; pointer-events: none; width: 100%; min-height: 100vh; }
.nova-ui-layer * { pointer-events: auto; }
.nova-nav { display: flex; justify-content: space-between; align-items: center; padding: 1.5rem 3rem; background: rgba(15, 23, 42, 0.6); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255, 255, 255, 0.08); position: fixed; top: 0; left: 0; width: 100%; z-index: 50; }
.nova-nav-links { display: flex; gap: 2rem; align-items: center; }
.nova-link { color: #94a3b8; text-decoration: none; font-weight: 500; transition: color 0.2s ease; cursor: pointer; }
.nova-link:hover { color: #38bdf8; }
.nova-hero-section { min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 8rem 2rem 4rem; position: relative; }
.nova-heading { font-size: 4.5rem; font-weight: 800; tracking: -0.02em; background: linear-gradient(135deg, #ffffff 0%, #a5b4fc 50%, #38bdf8 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin-bottom: 1.5rem; line-height: 1.1; }
.nova-text { font-size: 1.25rem; color: #94a3b8; max-width: 680px; margin-bottom: 2.5rem; line-height: 1.7; }
.nova-button { padding: 1rem 2.5rem; font-size: 1.1rem; font-weight: 600; color: #ffffff; background: linear-gradient(135deg, #6366f1 0%, #06b6d4 100%); border: none; border-radius: 9999px; cursor: pointer; transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); box-shadow: 0 10px 30px -5px rgba(99, 102, 241, 0.5); display: inline-flex; align-items: center; justify-content: center; gap: 0.75rem; }
.nova-button:hover { transform: translateY(-3px) scale(1.03); box-shadow: 0 20px 40px -5px rgba(99, 102, 241, 0.7); }
.nova-button-secondary { background: rgba(255, 255, 255, 0.08); border: 1px solid rgba(255, 255, 255, 0.15); box-shadow: none; }
.nova-button-secondary:hover { background: rgba(255, 255, 255, 0.15); box-shadow: 0 10px 25px -5px rgba(255, 255, 255, 0.1); }
.nova-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 2rem; max-width: 1200px; margin: 4rem auto; padding: 0 2rem; }
.nova-card { background: rgba(15, 23, 42, 0.55); backdrop-filter: blur(24px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 1.5rem; padding: 2.5rem; transition: transform 0.3s ease, border-color 0.3s ease; }
.nova-card:hover { transform: translateY(-6px); border-color: rgba(99, 102, 241, 0.4); }
.nova-card .nova-heading { font-size: 1.75rem; margin-bottom: 1rem; }
.nova-badge { display: inline-block; padding: 0.35rem 1rem; border-radius: 9999px; background: rgba(99, 102, 241, 0.15); color: #818cf8; font-size: 0.875rem; font-weight: 600; margin-bottom: 1.5rem; border: 1px solid rgba(99, 102, 241, 0.3); }
.nova-configurator-panel { position: fixed; bottom: 2.5rem; left: 50%; transform: translateX(-50%); z-index: 40; display: flex; gap: 1rem; padding: 0.85rem 1.5rem; background: rgba(15, 23, 42, 0.75); backdrop-filter: blur(20px); border: 1px solid rgba(255, 255, 255, 0.12); border-radius: 9999px; box-shadow: 0 20px 50px rgba(0,0,0,0.5); }
@media (max-width: 768px) {
    .nova-heading { font-size: 2.75rem; }
    .nova-nav { padding: 1rem 1.5rem; }
    .nova-configurator-panel { width: 90%; justify-content: center; flex-wrap: wrap; }
}
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
        self.elem_counter += 1;
        let elem_id = format!("nova-elem-{}", self.elem_counter);

        let tag = match comp.kind.as_str() {
            "section" => "section",
            "heading" => "h1",
            "text" => "p",
            "button" => "button",
            "card" => "div",
            "nav" => "nav",
            "link" => "a",
            "container" => "div",
            _ => "div",
        };

        let class_name = format!("nova-{}", comp.kind);
        self.html.push_str(&format!("<{} id=\"{}\" class=\"{}\">", tag, elem_id, class_name));

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

        for interaction in &comp.events {
            self.js.push_str(&format!(
                "    document.getElementById('{}')?.addEventListener('{}', () => {{\n",
                elem_id, interaction.event
            ));
            for stmt in &interaction.statements {
                match stmt {
                    Stmt::Assignment { target, op: _, value } => {
                        let t_str = target.join(".");
                        let val_str = self.compile_expr(value);
                        if self.states.contains(&t_str) {
                            self.js.push_str(&format!(
                                "        engine.setState('{}', {});\n",
                                t_str, val_str
                            ));
                        }
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
