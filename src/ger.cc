#include <libssh/libssh.h>
#include "gsl/gsl"
#include "fmt/core.h"

namespace ger {

int ger(int argc, const char* argv[])
{
    ssh_new();
    gsl::finally([]{});
    fmt::print("Ger!\n");
    return 0;
}

} /* namespace ger */
