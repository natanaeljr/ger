/**
 * \file config.cc
 * \author your name
 * \brief Config.
 * \date 2019-06-13
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/config.h"

#include <cassert>
#include "yaml-cpp/yaml.h"
#include "fmt/core.h"
#include "fmt/color.h"

namespace ger {
namespace cli {

/************************************************************************************************/
Config ConfigParser::Read(std::string_view filepath)
{
    YAML::Node yaml;

    try {
        yaml = YAML::LoadFile(filepath.data());
    }
    catch (const YAML::ParserException& e) {
        throw std::runtime_error(e.what());
    }
    catch (const YAML::BadFile& e) {
        throw std::runtime_error(e.what());
    }

    Config config{
        .remotes = ParseRemotes(yaml),
    };

    return config;
}

/************************************************************************************************/
std::vector<Remote> ConfigParser::ParseRemotes(const YAML::Node yaml)
{
    std::vector<Remote> remotes;

    const YAML::Node yaml_remotes = yaml["remotes"];
    if (yaml_remotes) {
        if (not yaml_remotes.IsSequence()) {
            throw std::runtime_error("'remotes' must be a sequence.");
        }

        remotes.reserve(yaml_remotes.size());
        for (auto yaml_remote : yaml_remotes) {
            remotes.emplace_back(Remote{
                .name = yaml_remote["name"].as<std::string>(),
                .url = yaml_remote["url"].as<std::string>(),
                .username = yaml_remote["username"].as<std::string>(),
                .http_password = yaml_remote["http-password"].as<std::string>(),
                .port = yaml_remote["port"].as<uint16_t>(8080),
            });
        }
    }

    return remotes;
}

} /* namespace cli */
} /* namespace ger */
