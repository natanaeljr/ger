/**
 * \file ger_cli.cc
 * \author Natanael Josue Rabello
 * \brief Ger CLI.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/gercli.h"

#include <array>
#include <vector>
#include <string>
#include "docopt.h"
#include "fmt/core.h"
#include "njr/enum_t.h"

#include "ger/cli/command.h"
#include "ger/cli/config.h"

namespace ger {
namespace cli {

/************************************************************************************************/
static constexpr const char kGerMainHelp[] = R"(Gerrit command-line client.
usage: ger [options] <command> [<args>...]

commands:
  help            Show help for a given command or concept.
  change          List changes in the gerrit server.
  review          Review changes through the command-line.
  config          Configure ger options.
  version         Show version.

options:
  --help          Show this screen.
  --version       Show version.)";

static constexpr const char kGerVersionStr[] = "ger version 0.1-alpha";

/************************************************************************************************/
int GerCli::Launch(int argc, const char* argv[])
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerMainHelp, { argv + 1, argv + argc }, true,
                               kGerVersionStr, true);

    const char* config_home_path = getenv("XDG_CONFIG_HOME");
    if (not config_home_path || config_home_path[0] == '\0') {
        config_home_path = getenv("HOME");
    }
    std::string config_filepath = config_home_path + std::string("/ger.yml");
    // fmt::print("config file: {}", config_file);
    // fflush(stdout);
    try {
        Config config = ConfigParser().Read(config_filepath);
        for (auto remote : config.remotes) {
            fmt::print(
                "name: '{}', url: '{}', port: '{}', username: '{}', http-password: "
                "'{}'\n",
                remote.name, remote.url, remote.port, remote.username,
                remote.http_password);
        }
    }
    catch (const std::runtime_error& e) {
        fmt::print("Failed to read config file: {}\n", e.what());
        return -2;
    }

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
    /* Find matching command and return it */
    for (auto command : njr::enum_t<Command>::values::array()) {
        if (input_command == command.name()) {
            return command;
        }
    }

    return Command::UNKNOWN;
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
        case Command::VERSION: {
            fmt::print("{}\n", kGerVersionStr);
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

} /* namespace cli */
} /* namespace ger */
