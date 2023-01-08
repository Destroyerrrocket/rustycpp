[[rustycpp::checkSymbolMatchTag(false, Enum)]];
[[rustycpp::checkSymbolMatchTag(false, A)]];
[[rustycpp::checkSymbolMatchTag(false, B)]];
namespace [[rustycpp::tagDecl(1)]] Enum {
	[[rustycpp::checkSymbolMatchTag(1, Enum)]];
	[[rustycpp::checkSymbolMatchTag(false, A)]];
	[[rustycpp::checkSymbolMatchTag(false, B)]];
	[[rustycpp::tagDecl(2)]]
	__rustycpp__(enum A);
	[[rustycpp::checkSymbolMatchTag(1, Enum)]];
	[[rustycpp::checkSymbolMatchTag(2, A)]];
	[[rustycpp::checkSymbolMatchTag(false, B)]];
	namespace [[rustycpp::tagDecl(3)]] Enum {
		[[rustycpp::checkSymbolMatchTag(3, Enum)]];
		[[rustycpp::checkSymbolMatchTag(2, A)]];
		[[rustycpp::checkSymbolMatchTag(false, B)]];
		[[rustycpp::tagDecl(4)]]
		__rustycpp__(enum A);
		[[rustycpp::checkSymbolMatchTag(3, Enum)]];
		[[rustycpp::checkSymbolMatchTag(4, A)]];
		[[rustycpp::checkSymbolMatchTag(false, B)]];
	}
	[[rustycpp::checkSymbolMatchTag(3, Enum)]];
	[[rustycpp::checkSymbolMatchTag(2, A)]];
	[[rustycpp::checkSymbolMatchTag(false, B)]];
	namespace [[rustycpp::tagDecl(5)]] Enum {
		[[rustycpp::checkSymbolMatchTag(3, Enum)]];
		[[rustycpp::checkSymbolMatchTag(4, A)]];
		[[rustycpp::checkSymbolMatchTag(false, B)]];
		[[rustycpp::tagDecl(6)]]
		__rustycpp__(enum B);
		[[rustycpp::checkSymbolMatchTag(3, Enum)]];
		[[rustycpp::checkSymbolMatchTag(4, A)]];
		[[rustycpp::checkSymbolMatchTag(6, B)]];
		namespace [[rustycpp::tagDecl(7)]] Enum {
			[[rustycpp::checkSymbolMatchTag(7, Enum)]];
			[[rustycpp::checkSymbolMatchTag(4, A)]];
			[[rustycpp::checkSymbolMatchTag(6, B)]];
			[[rustycpp::tagDecl(8)]]
			__rustycpp__(enum A);
			[[rustycpp::checkSymbolMatchTag(7, Enum)]];
			[[rustycpp::checkSymbolMatchTag(8, A)]];
			[[rustycpp::checkSymbolMatchTag(6, B)]];
		}
		[[rustycpp::checkSymbolMatchTag(7, Enum)]];
		[[rustycpp::checkSymbolMatchTag(4, A)]];
		[[rustycpp::checkSymbolMatchTag(6, B)]];
	}
	[[rustycpp::checkSymbolMatchTag(3, Enum)]];
	[[rustycpp::checkSymbolMatchTag(2, A)]];
	[[rustycpp::checkSymbolMatchTag(false, B)]];
}
[[rustycpp::checkSymbolMatchTag(1, Enum)]];
[[rustycpp::checkSymbolMatchTag(false, A)]];
[[rustycpp::checkSymbolMatchTag(false, B)]];
[[rustycpp::tagDecl(9)]]
__rustycpp__(enum A);
[[rustycpp::checkSymbolMatchTag(9, A)]];
