/**
 * \file config.h
 * \author your name
 * \brief Config.
 * \date 2019-06-12
 * \copyright Copyright (c) 2019
 */

#pragma once

#include <string_view>

/************************************************************************************************/

namespace ger {
namespace cli {

class Config {
   public:
    /**
     * \brief Config contructor.
     */
    Config(std::string_view filename);

    /**
     * \brief Config destructor.
     */
    ~Config();
};

} /* namespace cli */
} /* namespace ger */
