use nova_ast::*;
use nova_lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub code: String,
    pub message: String,
    pub expected: Vec<String>,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] Line {}, Column {}: {}\nExpected: [{}]",
            self.code,
            self.line,
            self.column,
            self.message,
            self.expected.join(", ")
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind, expected_name: &str) -> Result<Token, ParseError> {
        let tok = self.peek().clone();
        if tok.kind == kind {
            self.advance();
            Ok(tok)
        } else {
            Err(ParseError {
                code: "N204".to_string(),
                message: format!("Unexpected token {:?} found.", tok.kind),
                expected: vec![expected_name.to_string()],
                line: tok.span.line,
                column: tok.span.column,
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();
        while !self.check(TokenKind::Eof) {
            let decl = self.parse_declaration()?;
            declarations.push(decl);
        }
        Ok(Program { declarations })
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Scene => Ok(Declaration::Scene(self.parse_scene()?)),
            TokenKind::Page => Ok(Declaration::Page(self.parse_page()?)),
            TokenKind::Component => Ok(Declaration::Component(self.parse_component()?)),
            TokenKind::State => Ok(Declaration::State(self.parse_state()?)),
            TokenKind::Material => Ok(Declaration::Material(self.parse_material()?)),
            _ => Err(ParseError {
                code: "N201".to_string(),
                message: format!("Unexpected top-level declaration starting with {:?}", tok.kind),
                expected: vec!["scene".into(), "page".into(), "component".into(), "state".into(), "material".into()],
                line: tok.span.line,
                column: tok.span.column,
            }),
        }
    }

    fn parse_state(&mut self) -> Result<StateDecl, ParseError> {
        let state_tok = self.expect(TokenKind::State, "state")?;
        let name_tok = self.expect_identifier("state name")?;
        self.expect(TokenKind::Equal, "=")?;
        let initial_value = self.parse_expression()?;
        Ok(StateDecl {
            name: name_tok,
            initial_value,
            span: state_tok.span,
        })
    }

    fn parse_material(&mut self) -> Result<MaterialDecl, ParseError> {
        let mat_tok = self.expect(TokenKind::Material, "material")?;
        let name = self.expect_identifier("material name")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut material_type = "standard".to_string();
        let mut properties = Vec::new();

        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let prop_name = self.expect_identifier("property name")?;
            self.expect(TokenKind::Colon, ":")?;
            let expr = self.parse_expression()?;
            if prop_name == "type" {
                if let Expr::Identifier(ref t) = expr {
                    material_type = t.clone();
                }
            }
            properties.push((prop_name, expr));
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(MaterialDecl {
            name,
            material_type,
            properties,
            span: mat_tok.span,
        })
    }

    fn parse_scene(&mut self) -> Result<SceneDecl, ParseError> {
        let scene_tok = self.expect(TokenKind::Scene, "scene")?;
        let name = self.expect_identifier("scene name")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut cameras = Vec::new();
        let mut environment = None;
        let mut lights = Vec::new();
        let mut models = Vec::new();
        let mut meshes = Vec::new();
        let mut scroll_animations = Vec::new();

        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let tok = self.peek().clone();
            match tok.kind {
                TokenKind::Camera => cameras.push(self.parse_camera()?),
                TokenKind::Environment => environment = Some(self.parse_environment()?),
                TokenKind::Light => lights.push(self.parse_light()?),
                TokenKind::Model => models.push(self.parse_model()?),
                TokenKind::Mesh => meshes.push(self.parse_mesh()?),
                TokenKind::Scroll => scroll_animations.push(self.parse_scroll_animation()?),
                _ => {
                    return Err(ParseError {
                        code: "N204".to_string(),
                        message: format!("Unexpected token inside scene: {:?}", tok.kind),
                        expected: vec![
                            "camera".into(),
                            "environment".into(),
                            "light".into(),
                            "model".into(),
                            "mesh".into(),
                            "scroll".into(),
                        ],
                        line: tok.span.line,
                        column: tok.span.column,
                    });
                }
            }
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(SceneDecl {
            name,
            cameras,
            environment,
            lights,
            models,
            meshes,
            scroll_animations,
            span: scene_tok.span,
        })
    }

    fn parse_camera(&mut self) -> Result<CameraDecl, ParseError> {
        let cam_tok = self.expect(TokenKind::Camera, "camera")?;
        let name = self.expect_identifier("camera name")?;
        self.expect(TokenKind::OpenBrace, "{")?;
        let properties = self.parse_property_list()?;
        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(CameraDecl {
            name,
            properties,
            span: cam_tok.span,
        })
    }

    fn parse_environment(&mut self) -> Result<EnvironmentDecl, ParseError> {
        let env_tok = self.expect(TokenKind::Environment, "environment")?;
        self.expect(TokenKind::OpenBrace, "{")?;
        let properties = self.parse_property_list()?;
        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(EnvironmentDecl {
            properties,
            span: env_tok.span,
        })
    }

    fn parse_light(&mut self) -> Result<LightDecl, ParseError> {
        let light_tok = self.expect(TokenKind::Light, "light")?;
        let name = self.expect_identifier("light name")?;
        self.expect(TokenKind::OpenBrace, "{")?;
        let properties = self.parse_property_list()?;
        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(LightDecl {
            name,
            properties,
            span: light_tok.span,
        })
    }

    fn parse_model(&mut self) -> Result<ModelDecl, ParseError> {
        let model_tok = self.expect(TokenKind::Model, "model")?;
        let name = self.expect_identifier("model name")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut source = None;
        let mut properties = Vec::new();
        let mut animate = None;
        let mut interactions = Vec::new();

        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let tok = self.peek().clone();
            match tok.kind {
                TokenKind::Animate => {
                    self.advance();
                    self.expect(TokenKind::OpenBrace, "{")?;
                    let mut stmts = Vec::new();
                    while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
                        stmts.push(self.parse_statement()?);
                    }
                    self.expect(TokenKind::CloseBrace, "}")?;
                    animate = Some(stmts);
                }
                TokenKind::Interaction => {
                    self.advance();
                    self.expect(TokenKind::OpenBrace, "{")?;
                    while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
                        interactions.push(self.parse_interaction_block()?);
                    }
                    self.expect(TokenKind::CloseBrace, "}")?;
                }
                TokenKind::Hover | TokenKind::Click | TokenKind::Scroll => {
                    interactions.push(self.parse_interaction_block()?);
                }
                TokenKind::Identifier(ref prop_name) => {
                    let p_name = prop_name.clone();
                    self.advance();
                    if self.check(TokenKind::Colon) {
                        self.advance();
                        let expr = self.parse_expression()?;
                        if p_name == "source" {
                            if let Expr::Literal(LiteralValue::String(ref src)) = expr {
                                source = Some(src.clone());
                            }
                        }
                        properties.push((p_name, expr));
                    } else if self.check(TokenKind::Equal) || self.check(TokenKind::PlusEqual) {
                        // Direct statement like rotation.y += 1deg inside model
                        let op_tok = self.advance().clone();
                        let op = match op_tok.kind {
                            TokenKind::PlusEqual => "+=".to_string(),
                            TokenKind::MinusEqual => "-=".to_string(),
                            TokenKind::StarEqual => "*=".to_string(),
                            _ => "=".to_string(),
                        };
                        let val = self.parse_expression()?;
                        let stmt = Stmt::Assignment {
                            target: vec![p_name],
                            op,
                            value: val,
                        };
                        if animate.is_none() {
                            animate = Some(Vec::new());
                        }
                        if let Some(ref mut stmts) = animate {
                            stmts.push(stmt);
                        }
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(ModelDecl {
            name,
            source,
            properties,
            animate,
            interactions,
            span: model_tok.span,
        })
    }

    fn parse_mesh(&mut self) -> Result<MeshDecl, ParseError> {
        let mesh_tok = self.expect(TokenKind::Mesh, "mesh")?;
        let name = self.expect_identifier("mesh name")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut geometry = "box".to_string();
        let mut material = "standard".to_string();
        let mut properties = Vec::new();

        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let prop_name = self.expect_identifier("property name")?;
            self.expect(TokenKind::Colon, ":")?;
            let expr = self.parse_expression()?;
            if prop_name == "geometry" {
                if let Expr::Identifier(ref g) = expr {
                    geometry = g.clone();
                }
            } else if prop_name == "material" {
                if let Expr::Identifier(ref m) = expr {
                    material = m.clone();
                }
            }
            properties.push((prop_name, expr));
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(MeshDecl {
            name,
            geometry,
            material,
            properties,
            span: mesh_tok.span,
        })
    }

    fn parse_interaction_block(&mut self) -> Result<InteractionBlock, ParseError> {
        let tok = self.peek().clone();
        let event = match tok.kind {
            TokenKind::Hover => "hover".to_string(),
            TokenKind::Click => "click".to_string(),
            TokenKind::Scroll => "scroll".to_string(),
            _ => "click".to_string(),
        };
        self.advance();

        let mut statements = Vec::new();
        if self.match_token(TokenKind::OpenBrace) {
            while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
                statements.push(self.parse_statement()?);
            }
            self.expect(TokenKind::CloseBrace, "}")?;
        } else if self.match_token(TokenKind::FatArrow) {
            statements.push(self.parse_statement()?);
        }

        Ok(InteractionBlock {
            event,
            statements,
            span: tok.span,
        })
    }

    fn parse_scroll_animation(&mut self) -> Result<ScrollAnimationDecl, ParseError> {
        let scroll_tok = self.expect(TokenKind::Scroll, "scroll")?;
        self.expect(TokenKind::Animation, "animation")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut keyframes = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let pct_tok = self.peek().clone();
            let percentage = if let TokenKind::UnitNumber(val, Unit::Percent) = pct_tok.kind {
                self.advance();
                val
            } else {
                0.0
            };

            self.expect(TokenKind::OpenBrace, "{")?;
            let mut statements = Vec::new();
            while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
                statements.push(self.parse_statement()?);
            }
            self.expect(TokenKind::CloseBrace, "}")?;

            keyframes.push(ScrollKeyframe {
                percentage,
                statements,
                span: pct_tok.span,
            });
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(ScrollAnimationDecl {
            keyframes,
            span: scroll_tok.span,
        })
    }

    fn parse_page(&mut self) -> Result<PageDecl, ParseError> {
        let page_tok = self.expect(TokenKind::Page, "page")?;
        let name = self.expect_identifier("page name")?;
        self.expect(TokenKind::OpenBrace, "{")?;

        let mut items = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            items.push(self.parse_page_item()?);
        }

        self.expect(TokenKind::CloseBrace, "}")?;
        Ok(PageDecl {
            name,
            items,
            span: page_tok.span,
        })
    }

    fn parse_component(&mut self) -> Result<ComponentDecl, ParseError> {
        let comp_tok = self.expect(TokenKind::Component, "component")?;
        let name = self.expect_identifier("component name")?;

        let mut params = Vec::new();
        if self.match_token(TokenKind::OpenParen) {
            while !self.check(TokenKind::CloseParen) && !self.check(TokenKind::Eof) {
                let p = self.expect_identifier("parameter name")?;
                params.push(p);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::CloseParen, ")")?;
        }

        self.expect(TokenKind::OpenBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            items.push(self.parse_page_item()?);
        }
        self.expect(TokenKind::CloseBrace, "}")?;

        Ok(ComponentDecl {
            name,
            params,
            items,
            span: comp_tok.span,
        })
    }

    fn parse_page_item(&mut self) -> Result<PageItem, ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Scene => {
                self.advance();
                let scene_name = self.expect_identifier("scene reference name")?;
                Ok(PageItem::SceneRef(scene_name))
            }
            TokenKind::Section
            | TokenKind::Heading
            | TokenKind::Text
            | TokenKind::Button
            | TokenKind::Container
            | TokenKind::Card
            | TokenKind::Nav
            | TokenKind::Link => {
                let ui_item = self.parse_ui_component()?;
                Ok(PageItem::UIComponent(ui_item))
            }
            TokenKind::Identifier(ref name) => {
                let item_name = name.clone();
                self.advance();
                if self.check(TokenKind::OpenBrace) {
                    let ui_item = self.parse_ui_component_with_kind(&item_name, item_name.clone(), tok.span)?;
                    Ok(PageItem::UIComponent(ui_item))
                } else {
                    // Ref
                    Ok(PageItem::SceneRef(item_name))
                }
            }
            _ => Err(ParseError {
                code: "N205".to_string(),
                message: format!("Unexpected page item: {:?}", tok.kind),
                expected: vec!["section".into(), "heading".into(), "button".into(), "scene".into()],
                line: tok.span.line,
                column: tok.span.column,
            }),
        }
    }

    fn parse_ui_component(&mut self) -> Result<UIComponentItem, ParseError> {
        let tok = self.advance().clone();
        let kind = match tok.kind {
            TokenKind::Section => "section",
            TokenKind::Heading => "heading",
            TokenKind::Text => "text",
            TokenKind::Button => "button",
            TokenKind::Container => "container",
            TokenKind::Card => "card",
            TokenKind::Nav => "nav",
            TokenKind::Link => "link",
            TokenKind::Identifier(ref s) => s.as_str(),
            _ => "container",
        }
        .to_string();

        let mut name = kind.clone();
        if let TokenKind::Identifier(ref id) = self.peek().kind {
            name = id.clone();
            self.advance();
        }

        self.parse_ui_component_with_kind(&kind, name, tok.span)
    }

    fn parse_ui_component_with_kind(
        &mut self,
        kind: &str,
        name: String,
        span: Span,
    ) -> Result<UIComponentItem, ParseError> {
        let mut content = None;

        if let TokenKind::StringLiteral(ref text) = self.peek().kind {
            content = Some(text.clone());
            self.advance();
        }

        let mut properties = Vec::new();
        let mut children = Vec::new();
        let mut events = Vec::new();

        if self.match_token(TokenKind::OpenBrace) {
            while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
                let tok = self.peek().clone();
                match tok.kind {
                    TokenKind::Click | TokenKind::Hover => {
                        events.push(self.parse_interaction_block()?);
                    }
                    TokenKind::Section
                    | TokenKind::Heading
                    | TokenKind::Text
                    | TokenKind::Button
                    | TokenKind::Container
                    | TokenKind::Card
                    | TokenKind::Nav
                    | TokenKind::Link => {
                        children.push(self.parse_page_item()?);
                    }
                    TokenKind::Identifier(ref p_name) => {
                        let id_str = p_name.clone();
                        self.advance();
                        if self.check(TokenKind::Colon) {
                            self.advance();
                            let val = self.parse_expression()?;
                            properties.push((id_str, val));
                        } else if self.check(TokenKind::OpenBrace) {
                            let child = self.parse_ui_component_with_kind(&id_str, id_str.clone(), tok.span)?;
                            children.push(PageItem::UIComponent(child));
                        } else {
                            children.push(PageItem::SceneRef(id_str));
                        }
                    }
                    _ => {
                        self.advance();
                    }
                }
            }
            self.expect(TokenKind::CloseBrace, "}")?;
        }

        Ok(UIComponentItem {
            kind: kind.to_string(),
            name,
            content,
            properties,
            children,
            events,
            span,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Open => {
                self.advance();
                if let TokenKind::StringLiteral(url) = self.peek().kind.clone() {
                    self.advance();
                    Ok(Stmt::Action {
                        name: "open".into(),
                        args: vec![Expr::Literal(LiteralValue::String(url))],
                    })
                } else {
                    Ok(Stmt::Action {
                        name: "open".into(),
                        args: vec![],
                    })
                }
            }
            TokenKind::Navigate => {
                self.advance();
                self.expect(TokenKind::OpenParen, "(")?;
                let expr = self.parse_expression()?;
                self.expect(TokenKind::CloseParen, ")")?;
                Ok(Stmt::Action {
                    name: "navigate".into(),
                    args: vec![expr],
                })
            }
            _ => {
                let saved_pos = self.pos;
                if let Ok(target_path) = self.parse_target_path() {
                    if self.check(TokenKind::Equal)
                        || self.check(TokenKind::PlusEqual)
                        || self.check(TokenKind::MinusEqual)
                        || self.check(TokenKind::StarEqual)
                    {
                        let op_tok = self.advance().clone();
                        let op = match op_tok.kind {
                            TokenKind::PlusEqual => "+=".to_string(),
                            TokenKind::MinusEqual => "-=".to_string(),
                            TokenKind::StarEqual => "*=".to_string(),
                            _ => "=".to_string(),
                        };
                        let value = self.parse_expression()?;
                        return Ok(Stmt::Assignment {
                            target: target_path,
                            op,
                            value,
                        });
                    }
                }
                self.pos = saved_pos;
                let expr = self.parse_expression()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn parse_target_path(&mut self) -> Result<Vec<String>, ParseError> {
        let mut path = Vec::new();
        path.push(self.expect_identifier("target path identifier")?);
        while self.match_token(TokenKind::Dot) {
            path.push(self.expect_identifier("target path member")?);
        }
        Ok(path)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary_expression()?;

        while self.check(TokenKind::Plus)
            || self.check(TokenKind::Minus)
            || self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
        {
            let op_tok = self.advance().clone();
            let op_str = match op_tok.kind {
                TokenKind::Plus => "+",
                TokenKind::Minus => "-",
                TokenKind::Star => "*",
                TokenKind::Slash => "/",
                _ => "+",
            }
            .to_string();
            let right = self.parse_primary_expression()?;
            left = Expr::BinaryOp(Box::new(left), op_str, Box::new(right));
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Number(val) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(val, Unit::None)))
            }
            TokenKind::UnitNumber(val, unit) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(val, unit)))
            }
            TokenKind::StringLiteral(ref s) => {
                let str_val = s.clone();
                self.advance();
                Ok(Expr::Literal(LiteralValue::String(str_val)))
            }
            TokenKind::ColorLiteral(ref c) => {
                let color_val = c.clone();
                self.advance();
                Ok(Expr::Literal(LiteralValue::Color(color_val)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(false)))
            }
            TokenKind::OpenParen => {
                self.advance();
                let e1 = self.parse_expression()?;
                if self.match_token(TokenKind::Comma) {
                    let e2 = self.parse_expression()?;
                    self.expect(TokenKind::Comma, ",")?;
                    let e3 = self.parse_expression()?;
                    self.expect(TokenKind::CloseParen, ")")?;
                    Ok(Expr::Vector(Box::new(e1), Box::new(e2), Box::new(e3)))
                } else {
                    self.expect(TokenKind::CloseParen, ")")?;
                    Ok(e1)
                }
            }
            _ => {
                if let Ok(id_name) = self.expect_identifier("expression identifier") {
                    let mut expr = Expr::Identifier(id_name);
                    while self.match_token(TokenKind::Dot) {
                        let member = self.expect_identifier("member identifier")?;
                        expr = Expr::MemberAccess(Box::new(expr), member);
                    }
                    Ok(expr)
                } else {
                    Err(ParseError {
                        code: "N202".to_string(),
                        message: format!("Invalid expression starting with {:?}", tok.kind),
                        expected: vec!["number".into(), "string".into(), "identifier".into(), "vector (...)".into()],
                        line: tok.span.line,
                        column: tok.span.column,
                    })
                }
            }
        }
    }

    fn parse_property_list(&mut self) -> Result<Vec<(String, Expr)>, ParseError> {
        let mut props = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.check(TokenKind::Eof) {
            let name = self.expect_identifier("property name")?;
            self.expect(TokenKind::Colon, ":")?;
            let expr = self.parse_expression()?;
            props.push((name, expr));
        }
        Ok(props)
    }

    fn expect_identifier(&mut self, context: &str) -> Result<String, ParseError> {
        let tok = self.peek().clone();
        let name_opt = match tok.kind {
            TokenKind::Identifier(ref name) => Some(name.clone()),
            TokenKind::Camera => Some("camera".to_string()),
            TokenKind::Model => Some("model".to_string()),
            TokenKind::Light => Some("light".to_string()),
            TokenKind::Scene => Some("scene".to_string()),
            TokenKind::Page => Some("page".to_string()),
            TokenKind::Section => Some("section".to_string()),
            TokenKind::Heading => Some("heading".to_string()),
            TokenKind::Text => Some("text".to_string()),
            TokenKind::Button => Some("button".to_string()),
            TokenKind::Container => Some("container".to_string()),
            TokenKind::Card => Some("card".to_string()),
            TokenKind::Nav => Some("nav".to_string()),
            TokenKind::Link => Some("link".to_string()),
            _ => None,
        };

        if let Some(id) = name_opt {
            self.advance();
            Ok(id)
        } else {
            Err(ParseError {
                code: "N203".to_string(),
                message: format!("Expected identifier for {}, found {:?}", context, tok.kind),
                expected: vec!["identifier".into()],
                line: tok.span.line,
                column: tok.span.column,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_lexer::Lexer;

    #[test]
    fn test_parse_simple_scene() {
        let code = r#"
            scene Demo {
                camera main {
                    position: (0, 1, 6)
                }
                model object {
                    source: "./assets/model.glb"
                    animate {
                        rotation.y += 1deg
                    }
                    click {
                        scale = 1.2
                    }
                }
            }
            page Home {
                heading "My 3D Website"
                scene Demo
            }
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(program.declarations.len(), 2);
    }
}
