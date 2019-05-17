#include <cstdio>
#include <string>
#include <iostream>
#include <sstream>
#include <array>

#include "gtest/gtest.h"
#include "gmock/gmock.h"

#include "ger/ger.h"

/* TODO:
ger daemon start/stop
ger daemon status
ger config daemon notification enable
ger config daemon notification interval <t>
--interactive, --no-interactive
 */

/**************************************************************************************/
TEST(Gerrit, NoCommand)
{
    testing::internal::CaptureStdout();

    std::array argv = { "gerrit" };
    ger::ger(argv.size(), argv.begin());

    std::string output = testing::internal::GetCapturedStdout();
    std::cout << output << std::flush;

    EXPECT_EQ("Gerrit help\n", output);
}

/**************************************************************************************/
TEST(Gerrit, UnknownCommand)
{
    testing::internal::CaptureStdout();

    std::array argv = { "gerrit", "whitewalker" };
    ger::ger(argv.size(), argv.begin());

    std::string output = testing::internal::GetCapturedStdout();
    std::cout << output << std::flush;

    EXPECT_EQ(
        "Unknown argment: whitewalker\n"
        "Gerrit help\n",
        output);
}

/**************************************************************************************/
TEST(Gerrit, ChangesCommand)
{
    testing::internal::CaptureStdout();

    std::array argv = { "gerrit", "changes" };
    ger::ger(argv.size(), argv.begin());

    std::string output = testing::internal::GetCapturedStdout();
    std::cout << output << std::flush;

    EXPECT_EQ("Gerrit changes\n", output);
}
