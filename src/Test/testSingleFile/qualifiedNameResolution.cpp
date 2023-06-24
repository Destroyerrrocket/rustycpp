[[rustycpp::checkSymbolMatchTag(false, ::A)]];
[[rustycpp::checkSymbolMatchTag(false, Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
[[rustycpp::tagDecl(1)]]
__rustycpp__(enum A);
[[rustycpp::checkSymbolMatchTag(1, ::A)]];
[[rustycpp::checkSymbolMatchTag(false, Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
namespace [[rustycpp::tagDecl(2)]] Enum {
	[[rustycpp::checkSymbolMatchTag(1, ::A)]];
	[[rustycpp::checkSymbolMatchTag(false, Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, ::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
	[[rustycpp::tagDecl(3)]]
	__rustycpp__(enum A);
	[[rustycpp::checkSymbolMatchTag(1, ::A)]];
	[[rustycpp::checkSymbolMatchTag(3, Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(3, ::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
	namespace [[rustycpp::tagDecl(4)]] Enum {
		[[rustycpp::checkSymbolMatchTag(1, ::A)]];
		[[rustycpp::checkSymbolMatchTag(false, Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(3, ::Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(false, ::Enum::Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
		[[rustycpp::tagDecl(5)]]
		__rustycpp__(enum A);
		[[rustycpp::checkSymbolMatchTag(1, ::A)]];
		[[rustycpp::checkSymbolMatchTag(5, Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(3, ::Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(5, ::Enum::Enum::A)]];
		[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
	}
	[[rustycpp::checkSymbolMatchTag(1, ::A)]];
	[[rustycpp::checkSymbolMatchTag(5, Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(3, ::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(5, ::Enum::Enum::A)]];
	[[rustycpp::checkSymbolMatchTag(false, Enum::Enum::A)]];
}
[[rustycpp::checkSymbolMatchTag(1, ::A)]];
[[rustycpp::checkSymbolMatchTag(3, Enum::A)]];
[[rustycpp::checkSymbolMatchTag(3, ::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(5, ::Enum::Enum::A)]];
[[rustycpp::checkSymbolMatchTag(5, Enum::Enum::A)]];
