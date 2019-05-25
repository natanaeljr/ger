/**
 * \file ger_cli.cc
 * \author Natanael Josue Rabello
 * \brief Ger CLI.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/ger_cli.h"

#include <array>
#include <vector>
#include <string>
#include "docopt.h"
#include "fmt/core.h"
#include "njr/enum_t.h"

#include "ger/cli/commands.h"

namespace ger {

/************************************************************************************************/
static constexpr const char kGerMainHelp[] = R"(Gerrit command-line client.
usage: ger [-h|--help] [--version] [<command> [<args>...]]

commands:
  help            Show help for a given command or concept.
  change          List changes in the gerrit server.
  review          Review changes through the command-line.
  config          Configure ger options.

options:
  -h, --help      Show this screen.
  --version       Show version.)";

/************************************************************************************************/
int GerCli::Launch(int argc, const char* argv[])
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerMainHelp, { argv + 1, argv + argc }, true,
                               "Ger version: 0.1-alpha", true);

    /* Check if we have been given a command */
    if (not args["<command>"]) {
        fmt::print("{}\n", kGerMainHelp);
        return 0;
    }

    /* Get command in enum format and pass it to runner */
    Command command = ParseCommand(args["<command>"].asString());

    return RunCommand(command, args["<args>"].asStringList());
}

/************************************************************************************************/
Command GerCli::ParseCommand(std::string_view input_command)
{
    Command ret = Command::UNKNOWN;

    /* Find matching command and return it */
    for (auto command : njr::enum_t<Command>::values::array()) {
        if (input_command == command.name()) {
            ret = command;
            break;
        }
    }

    return ret;
}

/************************************************************************************************/
int GerCli::RunCommand(Command cmd, const std::vector<std::string>& args)
{
    /* Dispatch to command handler */
    switch (cmd) {
        case Command::CHANGE: {
            return RunChangeCommand(args);
        }
        case Command::REVIEW: {
            fmt::print("Not yet implemented.\n");
            return -1;
        }
        case Command::CONFIG: {
            fmt::print("Not yet implemented.\n");
            return -1;
        }
        case Command::HELP: {
            fmt::print("{}\n", kGerMainHelp);
            return 0;
        }
        case Command::UNKNOWN: {
            fmt::print("Unknown command.\n\n");
            fmt::print("{}\n", kGerMainHelp);
            return -1;
        }
    }

    return 0;
}

} /* namespace ger */
