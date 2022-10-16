//typedef int Pc;
void f(const Pc); // void f(char* const) (not const char*)
void g(const int Pc); // void g(const int)
void h(unsigned Pc); // void h(unsigned int)
void k(unsigned int Pc); // void k(unsigned int)
