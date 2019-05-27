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
#include "njr/enum_t.h"

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
    VERSION,
};

/**
 * \brief Change command handler.
 * \param argv Argument list.
 * \return 0 on success, negative if error.
 */
int RunChangeCommand(const std::vector<std::string>& argv);

} /* namespace ger */

/************************************************************************************************/

/**
 * \brief Translate Command enumerators to string.
 * \return command name.
 */
template<>
constexpr const char* ::njr::enum_t<ger::Command>::name() const
{
    using ger::Command;
    switch (enum_) {
        case Command::UNKNOWN: return "unknown";
        case Command::HELP: return "help";
        case Command::CHANGE: return "change";
        case Command::REVIEW: return "review";
        case Command::CONFIG: return "config";
        case Command::VERSION: return "version";
    }
    return nullptr;
}