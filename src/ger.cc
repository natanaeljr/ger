#include "gsl/gsl"
#include "fmt/core.h"
#include "absl/strings/str_format.h"
#include "libssh2.h"

namespace ger {

int ger(int argc, const char* argv[])
{
    std::string s = absl::StrFormat("%s", "Hi");
    libssh2_init(0);
    gsl::finally([]{});
    fmt::print("Ger!\n");
    return 0;
}

} /* namespace ger */
