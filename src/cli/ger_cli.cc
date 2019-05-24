/**
 * \file ger_cli.cc
 * \author Natanael Josue Rabello
 * \brief Ger CLI.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/ger_cli.h"

#include <vector>
#include <string>
#include <type_traits>

#include "fmt/core.h"
#include "fmt/ranges.h"
#include "docopt.h"

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
struct CmdArg {
    Command cmd;
    std::string_view arg;
};

/* Available commands */
static constexpr const std::array kCommands = {
    CmdArg{ .cmd = Command::HELP, .arg = "help" },
    CmdArg{ .cmd = Command::CHANGE, .arg = "change" },
    CmdArg{ .cmd = Command::REVIEW, .arg = "review" },
    CmdArg{ .cmd = Command::CONFIG, .arg = "config" },
};

/************************************************************************************************/
int GerCli::Launch(int argc, const char* argv[])
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerMainHelp, { argv + 1, argv + argc }, true,
                               "Ger version: 0.1-alpha", true);

    /* Check if we have been given a command */
    if (!args["<command>"]) {
        fmt::print("{}\n", kGerMainHelp);
        return 0;
    }

    /* Final command */
    auto cmd = Command::UNKNOWN;

    /* Get command */
    std::string_view input_command = args["<command>"].asString();
    for (auto& command : kCommands) {
        if (input_command == command.arg) {
            cmd = command.cmd;
            break;
        }
    }

    /* Run the command */
    RunCommand(cmd, args["<args>"].asStringList());

    return 0;
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
            fmt::print("Unkown command.\n\n");
            fmt::print("{}\n", kGerMainHelp);
            return -1;
        }
    }

    return 0;
}

} /* namespace ger */
