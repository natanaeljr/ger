/**
 * \file commands.h
 * \author Natanael Josue Rabello
 * \brief Commands
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#pragma once

#include <vector>
#include <string>

/************************************************************************************************/

namespace ger {

/**
 * Available Commands.
 */
enum class Command {
    UNKNOWN,
    HELP,
    CHANGE,
    REVIEW,
    CONFIG,
};

/**
 * \brief Change command handler.
 * \param argv Argument list.
 * \return 0 on success, negative if error.
 */
int RunChangeCommand(const std::vector<std::string>& argv);

} /* namespace ger */
