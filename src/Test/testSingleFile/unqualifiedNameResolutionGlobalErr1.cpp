namespace [[rustycpp::tagDecl(1)]] Enum {
	[[rustycpp::tagDecl(2)]]
	__rustycpp__(enum A);
}

[[rustycpp::tagDecl(3)]]
__rustycpp__(enum A);
__rustycpp__(enum Untagged);
__rustycpp__(enum Repeated);
__rustycpp__(enum Repeated);


[[rustycpp::checkSymbolMatchTag(1, A)]];
[[rustycpp::checkSymbolMatchTag(false, A)]];
[[rustycpp::checkSymbolMatchTag(true, NotFound)]];
[[rustycpp::checkSymbolMatchTag(1, Untagged)]];
[[rustycpp::checkSymbolMatchTag(1, Repeated)]];
