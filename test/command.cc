#include <cstdio>
#include <string>
#include <iostream>
#include <sstream>
#include <array>

#include "gtest/gtest.h"
#include "gmock/gmock.h"

#include "gerritc/gerrit.h"

/**************************************************************************************/
TEST(Gerrit, NoCommand)
{
    testing::internal::CaptureStdout();

    std::array argv = { "gerrit" };
    gerritc::gerrit(argv.size(), argv.begin());

    std::string output = testing::internal::GetCapturedStdout();
    std::cout << output << std::flush;

    EXPECT_EQ("Gerrit help\n", output);
}

/**************************************************************************************/
TEST(Gerrit, UnknownCommand)
{
    testing::internal::CaptureStdout();

    std::array argv = { "gerrit", "whitewalker" };
    gerritc::gerrit(argv.size(), argv.begin());

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
    gerritc::gerrit(argv.size(), argv.begin());

    std::string output = testing::internal::GetCapturedStdout();
    std::cout << output << std::flush;

    EXPECT_EQ("Gerrit changes\n", output);
}
