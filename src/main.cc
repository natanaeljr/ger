#include <cstdio>

namespace ger {
extern int ger(int argc, const char* argv[]);
}

int main(int argc, const char* argv[])
{
    ger::ger(argc, argv);
    printf("Main\n");
}
