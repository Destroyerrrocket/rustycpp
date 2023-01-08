namespace [[rustycpp::tagDecl(1)]] Enum {
	[[rustycpp::tagDecl(2)]]
	__rustycpp__(enum A);
}
[[rustycpp::unused]]
alignas(1)
[[rustycpp::tagDecl(3)]]
__rustycpp__(enum A);

[[rustycpp::checkSymbolMatchTag(true, A)]];
[[rustycpp::checkSymbolMatchTag(3, A)]];
[[rustycpp::checkSymbolMatchTag(1, Enum)]];
[[rustycpp::checkSymbolMatchTag(false, NotFound)]];
