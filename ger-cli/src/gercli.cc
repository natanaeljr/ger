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
#include "fmt/color.h"
#include "njr/enum_t.h"

#include "ger/cli/command.h"
#include "ger/cli/config.h"

namespace ger {
namespace cli {

/************************************************************************************************/
static constexpr const char kGerMainHelp[] =
    R"(usage: ger [options] [<command>] [<args>...]

Gerrit command-line client.

commands:
  change                    List changes in the gerrit server.
  review                    Review changes through the command-line.
  config                    Configure ger options.
  version                   Show version.
  help                      Show help for a given command or concept.

options:
  -c, --config-file <path>  Specifiy an alternate configuration file path.
  -v, --verbose             Verbose output.
  --version                 Show version.
  --help                    Show this screen.)";

static constexpr const char kGerVersionStr[] = "ger version 0.1-alpha";

/************************************************************************************************/
int GerCli::Launch(int argc, const char* argv[])
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerMainHelp, { argv + 1, argv + argc }, true,
                               kGerVersionStr, true);

    const bool verbose = [&] {
        auto it = args["--verbose"];
        return it ? it.asBool() : false;
    }();

    /* Read configuration file */
    auto& config_file = args["--config-file"];
    std::optional<Config> config =
        ReadConfig(config_file ? config_file.asString() : "", verbose);
    if (!config) {
        return -2;
    }

    /* Check if we have been given a command */
    if (not args["<command>"]) {
        fmt::print("{}\n", kGerMainHelp);
        return 0;
    }

    /* Get command in enum format and pass it to runner */
    Command command = ParseCommand(args["<command>"].asString());

    return RunCommand(command, args["<args>"].asStringList(), *config, verbose);
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
int GerCli::RunCommand(Command cmd, const std::vector<std::string>& args,
                       const Config& config, const bool verbose)
{
    /* Dispatch to command handler */
    switch (cmd) {
        case Command::CHANGE: {
            if (config.remotes.empty()) {
                fmt::print(fmt::fg(fmt::terminal_color::red), "no remote configured\n");
                return -2;
            }
            return RunChangeCommand(args, *config.remotes.begin(), verbose);
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

/************************************************************************************************/
std::optional<Config> GerCli::ReadConfig(std::string_view config_filepath, bool verbose)
{
    Config config;
    std::string config_file;

    if (config_filepath.empty()) {
        const char* config_filename = "ger.yml";
        const char* config_home_path = getenv("XDG_CONFIG_HOME");
        if (not config_home_path || config_home_path[0] == '\0') {
            config_filename = ".ger.yml";
            config_home_path = getenv("HOME");
        }
        config_file = fmt::format("{}/{}", config_home_path, config_filename);
    }
    else {
        config_file = config_filepath;
    }

    if (verbose) {
        fmt::print("+ config-file: {}\n", config_file);
    }

    try {
        config = ConfigParser().Read(config_file);
        if (verbose) {
            fmt::print("+ Remotes:\n");
            for (auto remote : config.remotes) {
                fmt::print(
                    "+ - name: '{}', url: '{}', port: '{}', username: '{}', "
                    "http-password: '{}'\n",
                    remote.name, remote.url, remote.port, remote.username,
                    remote.http_password);
            }
        }
    }
    catch (const std::runtime_error& e) {
        fmt::print(fmt::fg(fmt::terminal_color::red), "Failed to read config file: {}\n",
                   e.what());
        return std::nullopt;
    }

    return config;
}  // namespace cli

}  // namespace cli
} /* namespace ger */
