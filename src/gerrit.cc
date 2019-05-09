#include <stdio.h>

#include "argparse/argparse.hpp"

int gerrit(int argc, char* argv[])
{
    argparse::ArgumentParser program{ "gerrit" };

    program.add_argument("--changes").help("Show changes").action([](auto a) {
        return a;
    });
    program.parse_args(argc, argv);

    std::string s = program.get("changes");
    // if (s.empty())
        // program.print_help();

    printf("Gerrit desktop client!\n");
    return 0;
}