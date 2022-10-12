parser grammar mainCpp;
@tokenfactory {
pub type LocalTokenFactory<'input> = crate::utils::antlrlexerwrapper::AntlrLexerWrapperFactory<'input, crate::lex::token::Token>;
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
	ImportableHeaderName,
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

////////////////// Parser Rules ////////////////////////////////////////////////
translation_unit:
	declaration_seq?
	| global_module_fragment? module_declaration declaration_seq? private_module_fragment?
	;

// Start dynamic keywords
typedef_name:
	Identifier
	| simple_template_id
	;

namespace_name:
	Identifier
	| namespace_alias
	;

namespace_alias:
	Identifier
	;

class_name:
	Identifier
	| simple_template_id
	;

enum_name:
	Identifier
	;

template_name:
	Identifier
	;

// Simpler dynamic keywords
final__:
	Identifier
	;

override__:
	Identifier
	;

zero:
	IntegerLiteral
	;

// End dynamic keywords

// Start module scopes
global_module_fragment: Module Semicolon declaration_seq?
	;

private_module_fragment:
	Module Colon Private Semicolon declaration_seq?
	;

module_declaration:
	Export? Module module_name module_partition? attribute_specifier_seq?
	;

module_partition: Colon (Identifier Dot)* Identifier
	;

module_name: (Identifier Dot)* Identifier
	;
// End module scopes

// Start attributes
attribute_specifier_seq: attribute_specifier+
	;

attribute_specifier:
	LBracket LBracket attribute_using_prefix? attribute_list RBracket RBracket
	| alignment_specifier
	;

attribute_list:
	attribute?
	| attribute_list Comma attribute?
	| attribute ThreeDots
	| attribute_list Comma attribute ThreeDots
	;

attribute:
	attribute_token attribute_argument_clause?
	;

attribute_token:
	Identifier
	| attribute_scoped_token
	;

attribute_scoped_token:
	attribute_namespace DoubleColon Identifier
	;

attribute_argument_clause:
	LParen balanced_token_seq? RParen
	;

balanced_token_seq:
	balanced_token+
	;

balanced_token:
	LParen balanced_token_seq? RParen
	| LBracket balanced_token_seq? RBracket
	| LBrace balanced_token_seq? RBrace
	| ~( LParen | RParen | LBracket | RBracket | LBrace | RBrace )
	;


alignment_specifier:
	Alignas LParen type_id ThreeDots? RParen
	| Alignas LParen constant_expression ThreeDots? RParen
	;

attribute_using_prefix:
	Using attribute_namespace Colon
	;

attribute_namespace:
	Identifier
	;
// End attributes

// Start declarations
declaration_seq: declaration+
	;

declaration:
	block_declaration
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
	attribute_specifier_seq? declarator Semicolon
	;

alias_declaration:
	Using Identifier attribute_specifier_seq? Equal defining_type_id Semicolon
	;

simple_declaration:
	decl_specifier_seq init_declarator_list? Semicolon
	| attribute_specifier_seq decl_specifier_seq init_declarator_list Semicolon
	| attribute_specifier_seq? decl_specifier_seq ref_qualifier? LBrace identifier_list RBrace initializer Semicolon
	;

static_assert_declaration:
	Static_assert LParen constant_expression (Comma StringLiteral)? RParen Semicolon
	;

empty_declaration: Semicolon
	;

attribute_declaration:
	attribute_specifier_seq Semicolon
	;

decl_specifier_seq:
	decl_specifier+ attribute_specifier_seq?
	;

decl_specifier:
	storage_class_specifier
	| defining_type_specifier
	| function_specifier
	| Friend
	| Typedef
	| Constexpr
	| Consteval
	| Constinit
	| Inline
	;

storage_class_specifier:
	Static
	| Thread_local
	| Extern
	| Mutable
	;

function_specifier:
	Virtual
	| explicit_specifier
	;

explicit_specifier:
	Explicit
	| Explicit (LParen constant_expression RParen)
	;

defining_type_specifier_seq:
	defining_type_specifier+ attribute_specifier_seq?
	;

defining_type_specifier:
	type_specifier
	| class_specifier
	| enum_specifier
	;

type_specifier_seq:
	type_specifier+ attribute_specifier_seq?
	;

type_specifier:
	simple_type_specifier
	| elaborated_type_specifier
	| typename_specifier
	| cv_qualifier
	;

simple_type_specifier:
	nested_name_specifier? type_name
	| nested_name_specifier Template simple_template_id
	| decltype_specifier
	| placeholder_type_specifier
	| nested_name_specifier template_name
	| Char
	| Char8_t
	| Char16_t
	| Char32_t
	| Wchar_t
	| Bool
	| Short
	| Int
	| Long
	| Signed
	| Unsigned
	| Float
	| Double
	| Void
	;

type_name:
	class_name
	| enum_name
	| typedef_name
	;

elaborated_type_specifier:
	class_key attribute_specifier_seq? nested_name_specifier? Identifier
	| class_key simple_template_id
	| class_key nested_name_specifier Template? simple_template_id
	| elaborated_enum_specifier
	;

elaborated_enum_specifier:
	Enum nested_name_specifier? Identifier
	;

decltype_specifier:
	Decltype LParen expression RParen
	;

placeholder_type_specifier:
	type_constraint? Auto
	| type_constraint? Decltype LParen Auto RParen
	;

init_declarator_list:
	init_declarator (Comma init_declarator)*
	;

init_declarator:
	declarator initializer?
	| declarator requires_clause
	;

declarator:
	ptr_declarator
	| noptr_declarator parameters_and_qualifiers trailing_return_type
	;

ptr_declarator:
	noptr_declarator
	| ptr_operator ptr_declarator
	;

noptr_declarator:
	declarator_id attribute_specifier_seq?
	| noptr_declarator parameters_and_qualifiers
	| noptr_declarator LBracket constant_expression? RBracket attribute_specifier_seq?
	| LParen ptr_declarator RParen
	;

parameters_and_qualifiers:
	LParen parameter_declaration_clause RParen cv_qualifier_seq? ref_qualifier? noexcept_specifier? attribute_specifier_seq?
	;

trailing_return_type:
	Arrow type_id
	;

ptr_operator:
	Star attribute_specifier_seq? cv_qualifier_seq?
	| Ampersand attribute_specifier_seq?
	| DoubleAmpersand attribute_specifier_seq?
	| nested_name_specifier Star attribute_specifier_seq? cv_qualifier_seq?
	;

cv_qualifier_seq:
	cv_qualifier+
	;

cv_qualifier:
	Const
	| Volatile
	;

ref_qualifier:
	Ampersand
	| DoubleAmpersand
	;

declarator_id:
	ThreeDots? id_expression
	;

type_id:
	type_specifier_seq abstract_declarator?
	;

defining_type_id:
	defining_type_specifier_seq abstract_declarator?
	;

abstract_declarator:
	ptr_abstract_declarator
	| noptr_abstract_declarator parameters_and_qualifiers trailing_return_type
	| abstract_pack_declarator
	;

ptr_abstract_declarator:
	noptr_abstract_declarator
	| ptr_operator ptr_abstract_declarator?
	;

noptr_abstract_declarator:
	noptr_abstract_declarator (parameters_and_qualifiers | LBracket constant_expression? RBracket attribute_specifier_seq?)
	| (parameters_and_qualifiers | LBracket constant_expression? RBracket attribute_specifier_seq?)
	| LParen ptr_abstract_declarator RParen
	;

abstract_pack_declarator:
	ptr_operator* noptr_abstract_pack_declarator
	;

noptr_abstract_pack_declarator:
	noptr_abstract_pack_declarator parameters_and_qualifiers
	| noptr_abstract_pack_declarator LBracket constant_expression? RBracket attribute_specifier_seq?
	| ThreeDots
	;

parameter_declaration_clause:
	parameter_declaration_list? ThreeDots?
	| parameter_declaration_list Comma ThreeDots
	;

parameter_declaration_list:
	parameter_declaration (Comma parameter_declaration)*
	;

parameter_declaration:
	attribute_specifier_seq? decl_specifier_seq declarator
	| attribute_specifier_seq? decl_specifier_seq declarator Equal initializer_clause
	| attribute_specifier_seq? decl_specifier_seq abstract_declarator?
	| attribute_specifier_seq? decl_specifier_seq abstract_declarator? Equal initializer_clause
	;

initializer:
	brace_or_equal_initializer
	| LParen expression_list RParen
	;

brace_or_equal_initializer:
	Equal initializer_clause
	| braced_init_list
	;

initializer_clause:
	assignment_expression
	| braced_init_list
	;

braced_init_list:
	LBrace initializer_list Comma? RBrace
	| LBrace designated_initializer_list Comma? RBrace
	| LBrace RBrace
	;

initializer_list:
	initializer_clause ThreeDots? (Comma initializer_clause ThreeDots?)*
	;

designated_initializer_list:
	designated_initializer_clause (Comma designated_initializer_clause)*
	;

designated_initializer_clause:
	designator brace_or_equal_initializer
	;

designator:
	Dot Identifier
	;

expr_or_braced_init_list:
	expression
	| braced_init_list
	;

function_definition:
	attribute_specifier_seq? decl_specifier_seq? declarator virt_specifier_seq? function_body
	| attribute_specifier_seq? decl_specifier_seq? declarator requires_clause function_body
	;

function_body:
	ctor_initializer? compound_statement
	| function_try_block
	| Equal Default Semicolon
	| Equal Delete Semicolon
	;

enum_specifier:
	enum_head LBrace enumerator_list? RBrace
	| enum_head LBrace enumerator_list Comma RBrace
	;

enum_head:
	enum_key attribute_specifier_seq? enum_head_name? enum_base?
	;

enum_head_name:
	nested_name_specifier? Identifier
	;

opaque_enum_declaration:
	enum_key attribute_specifier_seq? enum_head_name enum_base? Semicolon
	;

enum_key:
	Enum
	| Enum Class
	| Enum Struct
	;

enum_base:
	Colon type_specifier_seq
	;

enumerator_list:
	enumerator_definition (Comma enumerator_definition)*
	;

enumerator_definition:
	enumerator
	| enumerator Equal constant_expression
	;

enumerator:
	Identifier attribute_specifier_seq?
	;

using_enum_declaration:
	Using elaborated_enum_specifier Semicolon
	;

namespace_definition:
	named_namespace_definition
	| unnamed_namespace_definition
	| nested_namespace_definition
	;

named_namespace_definition:
	Inline? Namespace attribute_specifier_seq? Identifier LBrace namespace_body RBrace
	;

unnamed_namespace_definition:
	Inline? Namespace attribute_specifier_seq? LBrace namespace_body RBrace
	;

nested_namespace_definition:
	Namespace enclosing_namespace_specifier DoubleColon Inline? Identifier LBrace namespace_body RBrace
	;

enclosing_namespace_specifier:
	Identifier (DoubleColon Identifier)*
	;

namespace_body:
	declaration_seq?
	;

namespace_alias_definition:
	Namespace Identifier Equal qualified_namespace_specifier Semicolon
	;

qualified_namespace_specifier:
	nested_name_specifier? namespace_name
	;

using_directive:
	attribute_specifier_seq? Using Namespace nested_name_specifier? Semicolon
	;

using_declaration:
	Using using_declarator_list Semicolon
	;

using_declarator_list:
	using_declarator ThreeDots? (Comma using_declarator ThreeDots?)*
	;

using_declarator:
	Typename nested_name_specifier unqualified_id
	;

asm_declaration:
	attribute_specifier_seq? Asm LParen StringLiteral RParen Semicolon
	;

linkage_specification:
	Extern StringLiteral LBrace declaration_seq RBrace
	| Extern StringLiteral declaration
	;
// End declarations

// Start module declarations
export_declaration:
	Export declaration
	| Export LBrace declaration_seq RBrace
	| Export module_import_declaration
	;

module_import_declaration:
	Import module_name attribute_specifier_seq? Semicolon
	| Import module_partition attribute_specifier_seq? Semicolon
	| Import ImportableHeaderName attribute_specifier_seq? Semicolon
	;
// End module declarations

// Start classes
class_specifier:
	class_head LBrace member_specification? RBrace
	;

class_head:
	class_key attribute_specifier_seq? class_head_name class_virt_specifier? base_clause?
	| class_key attribute_specifier_seq? base_clause?
	;

class_head_name:
	nested_name_specifier? class_name
	;

class_virt_specifier:
	final__
	;

class_key:
	Class
	| Struct
	| Union
	;

member_specification:
	member_declaration member_specification?
	| access_specifier Colon member_specification?
	;

member_declaration:
	attribute_specifier_seq? decl_specifier_seq? member_declarator_list? Semicolon
	| function_definition
	| using_declaration
	| using_enum_declaration
	| static_assert_declaration
	| template_declaration
	| explicit_specialization
	| deduction_guide
	| alias_declaration
	| opaque_enum_declaration
	| empty_declaration
	;

member_declarator_list:
	member_declarator (Comma member_declarator)*
	;

member_declarator:
	declarator virt_specifier_seq? pure_specifier?
	| declarator requires_clause
	| declarator brace_or_equal_initializer?
	| Identifier? attribute_specifier_seq? Colon constant_expression brace_or_equal_initializer?
	;

virt_specifier_seq:
	virt_specifier+
	;

virt_specifier:
	override__
	| final__
	;

pure_specifier:
	Equal zero
	;

conversion_function_id:
	Operator conversion_type_id
	;

conversion_type_id:
	type_specifier_seq conversion_declarator?
	;

conversion_declarator:
	ptr_operator+
	;

base_clause:
	Colon base_specifier_list
	;

base_specifier_list:
	base_specifier ThreeDots? (Comma base_specifier ThreeDots?)*
	;

base_specifier:
	attribute_specifier_seq? class_or_decltype
	| attribute_specifier_seq? Virtual access_specifier? class_or_decltype
	| attribute_specifier_seq? access_specifier Virtual class_or_decltype
	;

class_or_decltype:
	nested_name_specifier? type_name
	| nested_name_specifier Template simple_template_id
	| decltype_specifier
	;

access_specifier:
	Private
	| Protected
	| Public
	;

ctor_initializer:
	Colon mem_initializer_list
	;

mem_initializer_list:
	mem_initializer ThreeDots? (Comma mem_initializer ThreeDots?)*
	;

mem_initializer:
	mem_initializer_id LParen expression_list? RParen
	| mem_initializer_id braced_init_list
	;

mem_initializer_id:
	class_or_decltype
	| Identifier
	;
// End classes

// Start overloading
operator_function_id:
	Operator operator
	;

operator:
	New
	| Delete
	| New LBracket RBracket
	| Delete LBracket RBracket
	| Co_await
	| LParen RParen
	| LBracket RBracket
	| Arrow
	| ArrowStar
	| Tilde
	| Exclamation
	| Plus
	| Minus
	| Star
	| Slash
	| Percent
	| Caret
	| Ampersand
	| Pipe
	| Equal
	| PlusEqual
	| MinusEqual
	| StarEqual
	| SlashEqual
	| PercentEqual
	| CaretEqual
	| AmpersandEqual
	| PipeEqual
	| DoubleEqual
	| ExclamationEqual
	| Less
	| Greater
	| LessEqual
	| GreaterEqual
	| Spaceship
	| DoubleAmpersand
	| DoublePipe
	| DoubleLess
	| DoubleGreater
	| DoubleLessEqual
	| DoubleGreaterEqual
	| DoublePlus
	| DoubleMinus
	| Comma
	;

literal_operator_id:
	Operator StringLiteral Identifier
	| Operator UdStringLiteral
	;
// End overloading


// Start templates
template_declaration:
	template_head declaration
	| template_head concept_definition
	;

template_head:
	Template Less template_parameter_list Greater requires_clause?
	;

template_parameter_list:
	template_parameter (Comma template_parameter)*
	;

requires_clause:
	Requires constraint_logical_or_expression
	;

constraint_logical_or_expression:
	constraint_logical_and_expression (DoublePipe constraint_logical_and_expression)*
	;

constraint_logical_and_expression:
	primary_expression (DoubleAmpersand primary_expression)*
	;

template_parameter:
	type_parameter
	| parameter_declaration
	;

type_parameter:
	type_parameter_key ThreeDots? Identifier?
	| type_parameter_key Identifier? Equal type_id
	| type_constraint ThreeDots? Identifier?
	| type_constraint Identifier? Equal type_id
	| template_head type_parameter_key ThreeDots? Identifier?
	| template_head type_parameter_key Identifier? Equal id_expression
	;

type_parameter_key:
	Class
	| Typename
	;

type_constraint:
	nested_name_specifier? concept_name
	| nested_name_specifier? concept_name Less template_argument_list? Greater
	;

simple_template_id:
	template_name Less template_argument_list? Greater
	;

template_id:
	simple_template_id
	| operator_function_id Less template_argument_list? Greater
	| literal_operator_id Less template_argument_list? Greater
	;

template_argument_list:
	template_argument ThreeDots? (Comma template_argument ThreeDots?)*
	;

template_argument:
	constant_expression
	| type_id
	| id_expression
	;

constraint_expression:
	logical_or_expression
	;

deduction_guide:
	explicit_specifier? template_name LParen parameter_declaration_clause RParen Arrow simple_template_id;

concept_definition:
	Concept concept_name Equal constraint_expression Semicolon
	;

concept_name:
	Identifier
	;

typename_specifier:
	Typename nested_name_specifier Identifier
	| Typename nested_name_specifier Template? simple_template_id
	;

explicit_instantiation:
	Extern? Template declaration
	;

explicit_specialization:
	Template Less Greater declaration
	;
// End templates

// Start exception handling
try_block:
	Try compound_statement handler_seq
	;

function_try_block:
	Try ctor_initializer? compound_statement handler_seq
	;

handler_seq:
	handler+
	;

handler:
	Catch LParen exception_declaration? RParen compound_statement
	;

exception_declaration:
	attribute_specifier_seq? type_specifier_seq declarator
	| attribute_specifier_seq? type_specifier_seq abstract_declarator?
	| ThreeDots
	;

noexcept_specifier:
	Noexcept LParen constant_expression RParen
	| Noexcept
	;
// End exception handling

// Start statements
statement:
	labeled_statement
	| attribute_specifier_seq? expression_statement
	| attribute_specifier_seq? compound_statement
	| attribute_specifier_seq? selection_statement
	| attribute_specifier_seq? iteration_statement
	| attribute_specifier_seq? jump_statement
	| declaration_statement
	| attribute_specifier_seq? try_block
	;

init_statement:
	expression_statement
	| simple_declaration
	;

condition:
	expression
	| attribute_specifier_seq? decl_specifier_seq declarator brace_or_equal_initializer
	;

labeled_statement:
	attribute_specifier_seq? Identifier Colon statement
	| attribute_specifier_seq? Case constant_expression Colon statement
	| attribute_specifier_seq? Default Colon statement
	;

expression_statement:
	expression? Semicolon
	;

compound_statement:
	LBrace statement_seq? RBrace
	;

statement_seq:
	statement+
	;

selection_statement:
	If Constexpr? LParen init_statement? condition RParen statement
	| If Constexpr? LParen init_statement? condition RParen statement Else statement
	| Switch LParen init_statement? condition RParen statement
	;

iteration_statement:
	While LParen condition RParen statement
	| Do statement While LParen expression RParen Semicolon
	| For LParen init_statement condition? Semicolon expression? RParen statement
	| For LParen init_statement? for_range_declaration Colon for_range_initializer RParen statement
	;

for_range_declaration:
	attribute_specifier_seq? decl_specifier_seq declarator
	| attribute_specifier_seq? decl_specifier_seq ref_qualifier? LBracket identifier_list RBracket
	;

for_range_initializer:
	expr_or_braced_init_list
	;

jump_statement:
	Break Semicolon
	| Continue Semicolon
	| Return expr_or_braced_init_list? Semicolon
	| coroutine_return_statement
	| Goto Identifier Semicolon
	;

coroutine_return_statement:
	Co_return expr_or_braced_init_list? Semicolon
	;

declaration_statement:
	block_declaration
	;
// End statements

// Start expressions
primary_expression:
	literal
	| This
	| LParen expression RParen
	| id_expression
	| lambda_expression
	| fold_expression
	| requires_expression
	;

id_expression:
	unqualified_id
	| qualified_id
	;

unqualified_id:
	Identifier
	| operator_function_id
	| conversion_function_id
	| literal_operator_id
	| Tilde type_name
	| Tilde decltype_specifier
	| template_id
	;

qualified_id:
	nested_name_specifier Template? unqualified_id
	;

nested_name_specifier:
	DoubleColon
	| type_name DoubleColon
	| namespace_name DoubleColon
	| decltype_specifier DoubleColon
	| nested_name_specifier Identifier DoubleColon
	| nested_name_specifier Template? simple_template_id DoubleColon
	;

lambda_expression:
	lambda_introducer lambda_declarator? compound_statement
	| lambda_introducer Less template_parameter_list Greater requires_clause? lambda_declarator? compound_statement
	;

lambda_introducer:
	LBracket lambda_capture? RBracket
	;

lambda_declarator:
	LParen parameter_declaration_clause RParen decl_specifier_seq? noexcept_specifier? attribute_specifier_seq? trailing_return_type? requires_clause?
	;

lambda_capture:
	capture_default
	| capture_list
	| capture_default Comma capture_list
	;

capture_default:
	Ampersand
	| Equal
	;

capture_list:
	capture (Comma capture)*
	;

capture:
	simple_capture
	| init_capture
	;

simple_capture:
	Identifier ThreeDots?
	| Ampersand Identifier ThreeDots?
	| This
	| Star This
	;

init_capture:
	ThreeDots? Identifier initializer
	| Ampersand ThreeDots? Identifier initializer
	;

fold_expression:
	LParen cast_expression fold_operator ThreeDots RParen
	| LParen ThreeDots fold_operator cast_expression RParen
	| LParen cast_expression fold_operator ThreeDots fold_operator cast_expression RParen
	;

fold_operator:
	Plus
	| Minus
	| Star
	| Slash
	| Percent
	| Caret
	| Ampersand
	| Pipe
	| DoubleLess
	| DoubleGreater
	| PlusEqual
	| MinusEqual
	| StarEqual
	| SlashEqual
	| PercentEqual
	| CaretEqual
	| AmpersandEqual
	| DoubleLessEqual
	| DoubleGreaterEqual
	| Equal
	| DoubleEqual
	| ExclamationEqual
	| Less
	| Greater
	| LessEqual
	| GreaterEqual
	| DoubleAmpersand
	| DoublePipe
	| Comma
	| DotStar
	| ArrowStar
	;

requires_expression:
	Requires requirement_parameter_list? requirement_body
	;

requirement_parameter_list:
	LParen parameter_declaration_clause RParen
	;

requirement_body:
	LBrace requirement_seq? RBrace
	;

requirement_seq:
	requirement+
	;

requirement:
	simple_requirement
	| type_requirement
	| compound_requirement
	| nested_requirement
	;

simple_requirement:
	expression Semicolon
	;

type_requirement:
	Typename nested_name_specifier? type_name Semicolon
	;

compound_requirement:
	LBrace expression RBrace Noexcept? return_type_requirement? Semicolon
	;

return_type_requirement:
	Arrow type_constraint
	;

nested_requirement:
	Requires constraint_expression Semicolon
	;

postfix_expression:
	primary_expression
	| postfix_expression LBracket expr_or_braced_init_list RBracket
	| postfix_expression LParen expression_list? RParen
	| simple_type_specifier LParen expression_list? RParen
	| typename_specifier LParen expression_list? RParen
	| simple_type_specifier braced_init_list
	| typename_specifier braced_init_list
	| postfix_expression Dot Template? id_expression
	| postfix_expression Arrow Template? id_expression
	| postfix_expression DoublePlus
	| postfix_expression DoubleMinus
	| Dynamic_cast Less type_id Greater LParen expression RParen
	| Static_cast Less type_id Greater LParen expression RParen
	| Reinterpret_cast Less type_id Greater LParen expression RParen
	| Const_cast Less type_id Greater LParen expression RParen
	| Typeid LParen expression RParen
	| Typeid LParen type_id RParen
	;

expression_list:
	initializer_list
	;

unary_expression:
	postfix_expression
	| unary_operator cast_expression
	| DoublePlus cast_expression
	| DoubleMinus cast_expression
	| await_expression
	| Sizeof unary_expression
	| Sizeof LParen type_id RParen
	| Sizeof ThreeDots LParen Identifier RParen
	| Alignof LParen type_id RParen
	| noexcept_expression
	| new_expression
	| delete_expression
	;

unary_operator:
	Star
	| Ampersand
	| Plus
	| Minus
	| Exclamation
	| Tilde
	;

await_expression:
	Co_await cast_expression
	;

noexcept_expression:
	Noexcept LParen expression RParen
	;

new_expression:
	DoubleColon? New new_placement? new_type_id new_initializer?
	| DoubleColon? New new_placement? LParen type_id RParen new_initializer?
	;

new_placement:
	LParen expression_list RParen
	;

new_type_id:
	type_specifier_seq new_declarator?
	;

new_declarator:
	ptr_operator new_declarator?
	| noptr_new_declarator
	;

noptr_new_declarator:
	LBracket expression? RBracket attribute_specifier_seq?
	| noptr_new_declarator LBracket constant_expression RBracket attribute_specifier_seq?
	;

new_initializer:
	LParen expression_list? RParen
	| braced_init_list
	;

delete_expression:
	DoubleColon? Delete cast_expression
	| DoubleColon? Delete LBracket RBracket cast_expression
	;

cast_expression:
	unary_expression
	| LParen type_id RParen cast_expression
	;

pm_expression:
	cast_expression
	| pm_expression DotStar cast_expression
	| pm_expression ArrowStar cast_expression
	;

multiplicative_expression:
	pm_expression
	| multiplicative_expression Star pm_expression
	| multiplicative_expression Slash pm_expression
	| multiplicative_expression Percent pm_expression
	;

additive_expression:
	multiplicative_expression
	| additive_expression Plus multiplicative_expression
	| additive_expression Minus multiplicative_expression
	;

shift_expression:
	additive_expression
	| shift_expression DoubleLess additive_expression
	| shift_expression DoubleGreater additive_expression
	;

compare_expression:
	shift_expression
	| compare_expression Spaceship shift_expression
	;

relational_expression:
	compare_expression
	| relational_expression Less compare_expression
	| relational_expression Greater compare_expression
	| relational_expression LessEqual compare_expression
	| relational_expression GreaterEqual compare_expression
	;

equality_expression:
	relational_expression
	| equality_expression DoubleEqual relational_expression
	| equality_expression ExclamationEqual relational_expression
	;

and_expression:
	equality_expression
	| and_expression Ampersand equality_expression
	;

exclusive_or_expression:
	and_expression
	| exclusive_or_expression Caret and_expression
	;

inclusive_or_expression:
	exclusive_or_expression
	| inclusive_or_expression Pipe exclusive_or_expression
	;

logical_and_expression:
	inclusive_or_expression
	| logical_and_expression DoubleAmpersand inclusive_or_expression
	;

logical_or_expression:
	logical_and_expression
	| logical_or_expression DoublePipe logical_and_expression
	;

conditional_expression:
	logical_or_expression
	| logical_or_expression Question expression Colon assignment_expression
	;

yield_expression:
	Co_yield assignment_expression
	| Co_yield braced_init_list
	;

throw_expression:
	Throw assignment_expression?
	;

assignment_expression:
	conditional_expression
	| yield_expression
	| throw_expression
	| logical_or_expression assignment_operator initializer_clause
	;

assignment_operator:
	Equal
	| StarEqual
	| SlashEqual
	| PercentEqual
	| PlusEqual
	| MinusEqual
	| DoubleLessEqual
	| DoubleGreaterEqual
	| AmpersandEqual
	| CaretEqual
	| PipeEqual
	;

expression:
	assignment_expression
	| expression Comma assignment_expression
	;

constant_expression:
	conditional_expression
	;

// End expressions

// Misc
identifier_list:
	Identifier (Comma Identifier)*
	;

literal:
	IntegerLiteral
	| CharacterLiteral
	| FloatingPointLiteral
	| StringLiteral
	| BoolLiteral
	| PointerLiteral
	| user_defined_literal
	;

user_defined_literal:
	UdIntegerLiteral
	| UdFloatingPointLiteral
	| UdStringLiteral
	| UdCharacterLiteral
	;