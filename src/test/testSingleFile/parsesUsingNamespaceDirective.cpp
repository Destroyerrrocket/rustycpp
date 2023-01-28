[[rustycpp::checkSymbolMatchTag(false, A)]];
[[rustycpp::checkSymbolMatchTag(false, B)]];
[[rustycpp::checkSymbolMatchTag(false, ::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::B)]];
[[rustycpp::checkSymbolMatchTag(false, ::A::B)]];
[[rustycpp::checkSymbolMatchTag(false, A::B)]];
namespace [[rustycpp::tagDecl(1)]] A {
	[[rustycpp::checkSymbolMatchTag(1, A)]];
	[[rustycpp::checkSymbolMatchTag(false, B)]];
	[[rustycpp::checkSymbolMatchTag(1, ::A)]];
	[[rustycpp::checkSymbolMatchTag(false, ::B)]];
	[[rustycpp::checkSymbolMatchTag(false, ::A::B)]];
	[[rustycpp::checkSymbolMatchTag(false, A::B)]];
	namespace [[rustycpp::tagDecl(2)]] B {
		[[rustycpp::checkSymbolMatchTag(1, A)]];
		[[rustycpp::checkSymbolMatchTag(2, B)]];
		[[rustycpp::checkSymbolMatchTag(1, ::A)]];
		[[rustycpp::checkSymbolMatchTag(false, ::B)]];
		[[rustycpp::checkSymbolMatchTag(2, ::A::B)]];
		[[rustycpp::checkSymbolMatchTag(2, A::B)]];
	}
	[[rustycpp::checkSymbolMatchTag(1, A)]];
	[[rustycpp::checkSymbolMatchTag(2, B)]];
	[[rustycpp::checkSymbolMatchTag(1, ::A)]];
	[[rustycpp::checkSymbolMatchTag(false, ::B)]];
	[[rustycpp::checkSymbolMatchTag(2, ::A::B)]];
	[[rustycpp::checkSymbolMatchTag(2, A::B)]];
}
[[rustycpp::checkSymbolMatchTag(1, A)]];
[[rustycpp::checkSymbolMatchTag(false, B)]];
[[rustycpp::checkSymbolMatchTag(1, ::A)]];
[[rustycpp::checkSymbolMatchTag(false, ::B)]];
[[rustycpp::checkSymbolMatchTag(2, ::A::B)]];
[[rustycpp::checkSymbolMatchTag(2, A::B)]];
using namespace A;
[[rustycpp::checkSymbolMatchTag(1, A)]];
[[rustycpp::checkSymbolMatchTag(2, B)]];
[[rustycpp::checkSymbolMatchTag(1, ::A)]];
[[rustycpp::checkSymbolMatchTag(2, ::B)]];
[[rustycpp::checkSymbolMatchTag(2, ::A::B)]];
[[rustycpp::checkSymbolMatchTag(2, A::B)]];


[[rustycpp::checkSymbolMatchTag(false, C::D::E)]];
[[rustycpp::checkSymbolMatchTag(false, E)]];
namespace C {
	namespace D {
		namespace [[rustycpp::tagDecl(3)]] E {

		}
	}
}
[[rustycpp::checkSymbolMatchTag(3, C::D::E)]];
[[rustycpp::checkSymbolMatchTag(false, E)]];
using namespace C::D;
[[rustycpp::checkSymbolMatchTag(3, C::D::E)]];
[[rustycpp::checkSymbolMatchTag(3, E)]];
