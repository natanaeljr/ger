#include <vector>
#include <string>

#include "fmt/core.h"
#include "fmt/ranges.h"

#include "gsl/gsl"

namespace ger {

enum class Command {
    NONE,
    HELP,
    CHANGES,
};

static void print_help()
{
    fmt::print("Ger help.\n");
}

static void changes(gsl::span<std::string_view> args)
{
    fmt::print("change args: {}\n", fmt::join(args, " "));
}

int ger(int argc, const char* argv[])
{
    auto command = Command::NONE;

    if (argc <= 1) {
        print_help();
        return 1;
    }

    std::vector<std::string_view> args{ &argv[1], &argv[argc] };

    std::string_view cmd_arg{ args[0] };
    if (cmd_arg == "changes") {
        changes({ &*args.begin() + 1, &*args.end() });
    }
    else if (cmd_arg == "help") {
        print_help();
    }
    else {
        print_help();
        return 1;
    }

    return 0;
}

} /* namespace ger */
