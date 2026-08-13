use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    None,
    Deg,
    Rad,
    Sec,
    Ms,
    Px,
    Percent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Number(f64, Unit),
    String(String),
    Boolean(bool),
    Color(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Literal(LiteralValue),
    Vector(Box<Expr>, Box<Expr>, Box<Expr>),
    Identifier(String),
    MemberAccess(Box<Expr>, String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Assignment {
        target: Vec<String>,
        op: String,
        value: Expr,
    },
    Expression(Expr),
    Action {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraDecl {
    pub name: String,
    pub properties: Vec<(String, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentDecl {
    pub properties: Vec<(String, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightDecl {
    pub name: String,
    pub properties: Vec<(String, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionBlock {
    pub event: String, // click, hover, scroll
    pub statements: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelDecl {
    pub name: String,
    pub source: Option<String>,
    pub properties: Vec<(String, Expr)>,
    pub animate: Option<Vec<Stmt>>,
    pub interactions: Vec<InteractionBlock>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshDecl {
    pub name: String,
    pub geometry: String,
    pub material: String,
    pub properties: Vec<(String, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialDecl {
    pub name: String,
    pub material_type: String,
    pub properties: Vec<(String, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollKeyframe {
    pub percentage: f64,
    pub statements: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollAnimationDecl {
    pub keyframes: Vec<ScrollKeyframe>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneDecl {
    pub name: String,
    pub cameras: Vec<CameraDecl>,
    pub environment: Option<EnvironmentDecl>,
    pub lights: Vec<LightDecl>,
    pub models: Vec<ModelDecl>,
    pub meshes: Vec<MeshDecl>,
    pub scroll_animations: Vec<ScrollAnimationDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComponentItem {
    pub kind: String, // section, heading, text, button, container, card, nav, link
    pub name: String,
    pub content: Option<String>,
    pub properties: Vec<(String, Expr)>,
    pub children: Vec<PageItem>,
    pub events: Vec<InteractionBlock>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PageItem {
    UIComponent(UIComponentItem),
    SceneRef(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageDecl {
    pub name: String,
    pub items: Vec<PageItem>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDecl {
    pub name: String,
    pub params: Vec<String>,
    pub items: Vec<PageItem>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDecl {
    pub name: String,
    pub initial_value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Declaration {
    Scene(SceneDecl),
    Page(PageDecl),
    Component(ComponentDecl),
    State(StateDecl),
    Material(MaterialDecl),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
