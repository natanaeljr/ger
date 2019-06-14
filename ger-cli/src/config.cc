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

namespace ger {
namespace cli {

/************************************************************************************************/
Config::Config(std::string_view filename)
{
    YAML::Node config = YAML::LoadFile(filename.data());

    // fmt::print("first remote: {}\n",
    //            config["remotes"].begin()->operator[]("name").as<std::string>());
}

/************************************************************************************************/
Config::~Config()
{
}

} /* namespace cli */
} /* namespace ger */
