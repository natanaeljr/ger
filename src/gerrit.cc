/**
 * \file gerrit.cc
 * \author Natanael Josue Rabello
 * \brief  Gerrit
 * \date 2019-05-09
 * \copyright Copyright (c) 2019
 */

#include <cstdio>
#include <vector>
#include <string>
#include <iostream>

#include <libnotify/notify.h>

/**************************************************************************************/

namespace gerritc {

enum class Command {
    NONE,
    HELP,
    CHANGES,
};

int gerrit(int argc, const char* argv[])
{
    auto command = Command::NONE;
    const std::vector<std::string_view> args{ &argv[1], &argv[argc] };

    if (!args.empty()) {
        auto& arg = args[0];
        if (arg == "changes") {
            command = Command::CHANGES;
        } else {
            printf("Unknown argment: %s\n", arg.data());
            command = Command::HELP;
        }
    }

    switch (command) {
        case Command::CHANGES:
            printf("Gerrit changes\n");
            break;
        case Command::HELP:
        case Command::NONE:
            printf("Gerrit help\n");
            break;
    }

    notify_init("Gerrit");
    NotifyNotification* hello = notify_notification_new(
        "Hello World", "This is an notification example", "dialog-information");
    notify_notification_show(hello, NULL);
    g_object_unref(G_OBJECT(hello));
    notify_uninit();

    return 0;
}

} /* namespace gerritc */
