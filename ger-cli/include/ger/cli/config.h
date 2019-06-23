/**
 * \file config.h
 * \author Natanael Josue Rabello
 * \brief Config.
 * \date 2019-06-12
 * \copyright Copyright (c) 2019
 */

#pragma once

#include <string>
#include <string_view>
#include <vector>

/************************************************************************************************/

/* Forward declarations */
namespace YAML {
class Node;
}

namespace ger {
namespace cli {

/**
 * Remote data structure.
 */
struct Remote {
    std::string name;
    std::string url;
    std::string username;
    std::string http_password;
    uint16_t port;
};

/**
 * Configuration data structure.
 */
struct Config {
    std::vector<Remote> remotes;
};

/**
 * Parse config from yaml file.
 */
class ConfigParser {
   public:
    /**
     * \brief Read configuration from file.
     * \param filepath
     */
    Config Read(std::string_view filepath);

   private:
    std::vector<Remote> ParseRemotes(const YAML::Node yaml);
};

} /* namespace cli */
} /* namespace ger */
