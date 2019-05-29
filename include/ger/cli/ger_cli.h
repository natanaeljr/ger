/**
 * \file ger_cli.h
 * \author Natanael Josue Rabello
 * \brief Gerrit command-line client.
 * \date 2019-05-14
 */

#pragma once

#include "ger/cli/commands.h"

/************************************************************************************************/

namespace ger {

class GerCli {
   public:
    /**
     * \brief Deleted constructor. GerCli is static class.
     */
    GerCli() = delete;

    /**
     * \brief Launch CLI from main entrance style.
     * \param argc Argument count.
     * \param argv Argument list.
     * \return 0 on success, negative if error.
     */
    static int Launch(int argc, const char* argv[]);

    /**
     * \brief Run a specific command
     * \param cmd Command to run.
     * \param args Argument list.
     * \return 0 on success, negative if error.
     */
    static int RunCommand(Command cmd, const std::vector<std::string>& args);

   private:
    /**
     * \brief Parse input command from string to enum format.
     * \param input_command Command to parse.
     * \return Corresponding command, or UNKNOWN if command not valid.
     */
    static Command ParseCommand(std::string_view input_command);
};

} /* namespace ger */