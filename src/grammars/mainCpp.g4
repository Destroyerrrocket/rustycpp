parser grammar mainCpp;
@tokenfactory {
pub type LocalTokenFactory<'input> = crate::utils::antlrlexerwrapper::AntlrLexerWrapperFactory<'input, crate::lexer::token::Token>;
}
tokens {
	Identifier,
	Alignas,
	Alignof,
	Asm,
	Auto,
	Bool,
	Break,
	Case,
	Catch,
	Char,
	Char8_t,
	Char16_t,
	Char32_t,
	Class,
	Concept,
	Const,
	Consteval,
	Constexpr,
	Constinit,
	Const_cast,
	Continue,
	Co_await,
	Co_return,
	Co_yield,
	Decltype,
	Default,
	Delete,
	Do,
	Double,
	Dynamic_cast,
	Else,
	Enum,
	Explicit,
	Export,
	Extern,
	Float,
	For,
	Friend,
	Goto,
	If,
	Inline,
	Int,
	Long,
	Mutable,
	Namespace,
	New,
	Noexcept,
	Operator,
	Private,
	Protected,
	Public,
	Register,
	Reinterpret_cast,
	Requires,
	Return,
	Short,
	Signed,
	Sizeof,
	Static,
	Static_assert,
	Static_cast,
	Struct,
	Switch,
	Template,
	This,
	Thread_local,
	Throw,
	Try,
	Typedef,
	Typeid,
	Typename,
	Union,
	Unsigned,
	Using,
	Virtual,
	Void,
	Volatile,
	Wchar_t,
	While,
	LBrace,
	RBrace,
	LBracket,
	RBracket,
	LParen,
	RParen,
	Semicolon,
	Colon,
	ThreeDots,
	Question,
	DoubleColon,
	Dot,
	DotStar,
	Arrow,
	ArrowStar,
	Tilde,
	Exclamation,
	Plus,
	Minus,
	Star,
	Slash,
	Percent,
	Caret,
	Ampersand,
	Pipe,
	Equal,
	PlusEqual,
	MinusEqual,
	StarEqual,
	SlashEqual,
	PercentEqual,
	CaretEqual,
	AmpersandEqual,
	PipeEqual,
	DoubleEqual,
	ExclamationEqual,
	Less,
	Greater,
	LessEqual,
	GreaterEqual,
	Spaceship,
	DoubleAmpersand,
	DoublePipe,
	DoubleLess,
	DoubleGreater,
	DoubleLessEqual,
	DoubleGreaterEqual,
	DoublePlus,
	DoubleMinus,
	Comma,
	Import,
	Module,
	IntegerLiteral,
	FloatingPointLiteral,
	CharacterLiteral,
	StringLiteral,
	BoolLiteral,
	PointerLiteral,
	UdIntegerLiteral,
	UdFloatingPointLiteral,
	UdCharacterLiteral,
	UdStringLiteral
}

//////////////////////////////////////////////////
// / Parser Rules ////////////////////////////////////////////////
translation_unit:
	declaration_seq?
	| global_module_fragment? module_declaration declaration_seq? private_module_fragment?;

global_module_fragment: Module Semicolon declaration_seq?;

private_module_fragment:
	Module Colon Private Semicolon declaration_seq?;

module_declaration:
	Export? Module module_name module_partition? attribute_specifier_seq?;

module_partition: Colon (Identifier Dot)* Identifier;

module_name: (Identifier Dot)* Identifier;

attribute_specifier_seq: attribute_specifier+;

attribute_specifier:
	LBracket LBracket attribute_using_prefix? attribute_list RBracket RBracket
//	| alignment_specifier
	;

attribute_list:
	attribute?
	| attribute_list Comma attribute?
	| attribute ThreeDots
	| attribute_list Comma attribute ThreeDots;

attribute:
	attribute_token attribute_argument_clause?;

attribute_token:
	Identifier
	| attribute_scoped_token;

attribute_scoped_token:
	attribute_namespace DoubleColon Identifier;

attribute_argument_clause:
	LParen balanced_token_seq? RParen;

balanced_token_seq:
	balanced_token+;

balanced_token:
	LParen balanced_token_seq? RParen
	| LBracket balanced_token_seq? RBracket
	| LBrace balanced_token_seq? RBrace
	| ~( LParen | RParen | LBracket | RBracket | LBrace | RBrace );

/*
alignment_specifier:
	Alignas LParen type_id ThreeDots? RParen
	| Alignas LParen constant_expression ThreeDots? RParen;
*/
attribute_using_prefix:
	Using attribute_namespace Colon;

attribute_namespace:
	Identifier;

declaration_seq: declaration+;

declaration:
LBrace RBrace
/*	block_declaration
	| nodeclspec_function_declaration
	| function_definition
	| template_declaration
	| deduction_guide
	| explicit_instantiation
	| explicit_specialization
	| export_declaration
	| linkage_specification
	| namespace_definition
	| empty_declaration
	| attribute_declaration
	| module_import_declaration
;

block_declaration:
	simple_declaration
	| asm_declaration
	| namespace_alias_definition
	| using_declaration
	| using_enum_declaration
	| using_directive
	| static_assert_declaration
	| alias_declaration
	| opaque_enum_declaration
	;

nodeclspec_function_declaration:
	attribute_specifier_seq? declarator ;
*/

//expr : LParen expr RParen # Paren | expr op=(DoubleMinus|DoublePlus) # SinglePostIncrement |
// <assoc=right> op=(DoubleMinus|DoublePlus) expr # SinglePreIncrement | <assoc=right>
// op=(Minus|Plus) expr # UnarySign | <assoc=right> op=(Exclamation|Tilde) expr #
// LogicalOrBitNot | expr op=(Star|Slash|Percent) expr # MulDivMod | expr op=(Plus|Minus) expr #
// AddSub | expr op=(DoubleLess|DoubleGreater) expr # BitShift | expr Spaceship expr # Spaceship
// | expr op=(Less|LessEqual|Greater|GreaterEqual) expr # Compare | expr
// op=(DoubleEqual|ExclamationEqual) expr # Equalities | expr (Ampersand) expr # BitAnd | expr
// (Caret) expr # BitXor | expr (Pipe) expr # BitOr | expr (DoubleAmpersand) expr # LogAnd |
// expr (DoublePipe) expr # LogOr | <assoc=right> expr Question expr Colon expr # Ternary | expr
// Comma expr # Comma | IntegerLiteral # Number ;

