# NOVA EBNF Grammar Specification

This document defines the formal Extended Backus-Naur Form (EBNF) grammar for the **NOVA** programming language.

```ebnf
Program         ::= ( Declaration )* ;

Declaration     ::= SceneDecl
                  | PageDecl
                  | ComponentDecl
                  | StateDecl
                  | MaterialDecl
                  | AnimationDecl
                  | ImportDecl ;

SceneDecl       ::= "scene" Identifier "{" ( SceneItem )* "}" ;

SceneItem       ::= CameraDecl
                  | LightDecl
                  | ModelDecl
                  | MeshDecl
                  | EnvironmentDecl
                  | ScrollDecl ;

CameraDecl      ::= "camera" Identifier "{" ( PropertyAssignment )* "}" ;

EnvironmentDecl ::= "environment" "{" ( PropertyAssignment )* "}" ;

LightDecl       ::= "light" Identifier "{" ( PropertyAssignment )* "}" ;

ModelDecl       ::= "model" Identifier "{" ( ModelProperty )* "}" ;

ModelProperty   ::= PropertyAssignment
                  | AnimateBlock
                  | InteractionBlock ;

MeshDecl        ::= "mesh" Identifier "{" ( PropertyAssignment )* "}" ;

MaterialDecl    ::= "material" Identifier "{" ( PropertyAssignment )* "}" ;

AnimateBlock    ::= "animate" "{" ( ExpressionStmt )* "}" ;

InteractionBlock::= "interaction" "{" ( EventBlock )* "}" ;

EventBlock      ::= EventType "{" ( Statement )* "}" ;

EventType       ::= "hover" | "click" | "scroll" | "drag" ;

PageDecl        ::= "page" Identifier "{" ( PageItem )* "}" ;

PageItem        ::= UIComponentItem | "scene" Identifier ;

UIComponentItem ::= ComponentType Identifier ( StringLiteral )? "{" ( Statement | UIProperty )* "}" ;

ComponentType   ::= "section" | "heading" | "text" | "button" | "container" | "card" | "nav" | "link" ;

ComponentDecl   ::= "component" Identifier ( "(" ParameterList ")" )? "{" ( PageItem )* "}" ;

StateDecl       ::= "state" Identifier "=" Expression ;

ScrollDecl      ::= "scroll" "animation" "{" ( ScrollKeyframe )* "}" ;

ScrollKeyframe  ::= Percentage "{" ( Statement )* "}" ;

PropertyAssignment ::= Identifier ":" Expression ;

UIProperty      ::= Identifier ":" Expression ;

Statement       ::= ExpressionStmt
                  | AssignmentStmt
                  | ActionStmt ;

ExpressionStmt  ::= Expression ;

AssignmentStmt  ::= Identifier ( "." Identifier )* TargetOperator Expression ;

TargetOperator  ::= "=" | "+=" | "-=" | "*=" ;

ActionStmt      ::= Identifier "(" ( ArgumentList )? ")"
                  | "open" StringLiteral
                  | "navigate" "(" StringLiteral ")" ;

Expression      ::= Literal
                  | VectorLiteral
                  | Identifier ( "." Identifier )*
                  | BinaryExpr ;

VectorLiteral   ::= "(" Expression "," Expression "," Expression ")" ;

Literal         ::= NumberLiteral
                  | UnitLiteral
                  | ColorLiteral
                  | StringLiteral
                  | BooleanLiteral ;

UnitLiteral     ::= NumberLiteral ( "deg" | "rad" | "s" | "ms" | "px" | "%" ) ;

ColorLiteral    ::= "#" HexDigit HexDigit HexDigit HexDigit HexDigit HexDigit ;

Identifier      ::= [a-zA-Z_][a-zA-Z0-9_]* ;
NumberLiteral   ::= [0-9]+ ( "." [0-9]+ )? ;
StringLiteral   ::= '"' [^"]* '"' ;
BooleanLiteral  ::= "true" | "false" ;
Percentage      ::= [0-9]+ "%" ;
HexDigit        ::= [0-9a-fA-F] ;
```
