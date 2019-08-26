/**
 * \file
 * \author  Natanael Josue Rabello
 * \brief   Json Codec Tests.
 * \version 0.1
 * \date    2019-08-25
 * \copyright Copyright (c) 2019
 */

#include "gtest/gtest.h"
#include <iostream>
#include "ger/json.h"

/**************************************************************************************/

TEST(JsonCodecTest, ChangeInfo)
{
    capnp::MallocMessageBuilder arena_;
    ger::JsonCodec codec_;

    constexpr std::string_view json = R"({"status":"MERGED"})";

    auto orphan = codec_.decode<gerrit::changes::ChangeInfo>({ json.begin(), json.end() },
                                                             arena_.getOrphanage());
    auto changeinfo = orphan.get();
    EXPECT_EQ(gerrit::changes::ChangeStatus::MERGED, changeinfo.getStatus());
}

/**************************************************************************************/

class MyTestSuite : public testing::TestWithParam<int> {
};

TEST_P(MyTestSuite, MyTest)
{
    std::cout << "Example Test Param: " << GetParam() << std::endl;
}
