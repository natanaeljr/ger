/**
 * \file main.cc
 * \author Natanael Josue Rabello
 * \brief
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include <cstdio>

#include "ger/cli/gercli.h"

/************************************************************************************************/

int main(int argc, const char* argv[])
{
    return ger::cli::GerCli::Launch(argc, argv);
}