#include "ger/cli/ger.h"

#include <vector>
#include <string>

#include <unistd.h>

#include "fmt/core.h"
#include "fmt/ranges.h"
#include "fmt/printf.h"
#include "fmt/format.h"
#include "fmt/color.h"
#include "gsl/gsl"
#include "curl/curl.h"
#include "nlohmann/json.hpp"

namespace ger {

enum class Command {
    NONE,
    HELP,
    CHANGES,
};

static void print_main_help()
{
    fmt::print("Gerrit terminal client.\n");
    fmt::print("USAGE: ger <command>\n\n");
    fmt::print("Commands:\n");
    fmt::print("change - Show all changes\n");
}

static size_t writeFunction(void* ptr, size_t size, size_t nmemb, std::string* data)
{
    data->append((char*)ptr, size * nmemb);
    return size * nmemb;
}

static void change(gsl::span<std::string_view> args)
{
    // no more args for now
    if (args.size() != 0) {
        fmt::print("'change' command takes no arguments.\n", args.size());
        return;
    }
    CURL* curl = nullptr;
    CURLcode res;

    curl_global_init(CURL_GLOBAL_ALL);
    auto _clean_global_curl = gsl::finally([] { curl_global_cleanup(); });

    curl = curl_easy_init();
    if (!curl) {
        fmt::print(stderr, "Failed to init easy curl\n");
        return;
    }
    auto _clean_easy_curl = gsl::finally([&] { curl_easy_cleanup(curl); });

    curl_easy_setopt(curl, CURLOPT_URL,
                     "https://gerrit.ped.datacom.ind.br/a/changes/?q=is:open+owner:self");
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);

    curl_easy_setopt(curl, CURLOPT_USERPWD,
                     "natanael.rabello.cwi:9of//kYGdM8g3PDcYL2JAHncMRwQ2algDYlgE2CsdA");
    // curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.42.0");
    curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_DIGEST);
    // curl_easy_setopt(curl, CURLOPT_MAXREDIRS, 50L);
    // curl_easy_setopt(curl, CURLOPT_TCP_KEEPALIVE, 1L);

    std::string response_string;
    std::string header_string;
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeFunction);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response_string);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &header_string);

    char* url;
    long response_code;
    double elapsed;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
    curl_easy_getinfo(curl, CURLINFO_TOTAL_TIME, &elapsed);
    curl_easy_getinfo(curl, CURLINFO_EFFECTIVE_URL, &url);

    /* Perform the request, res will get the return code */
    res = curl_easy_perform(curl);
    /* Check for errors */
    if (res != CURLE_OK) {
        fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        return;
    }

    // SUCCESS
    if (response_string.compare(0, 5, ")]}'\n")) {
        fmt::print(stderr, "Unrecognized response from server:\n\n{}", response_string);
        return;
    }
    auto json = nlohmann::json::parse(response_string.data() + 4);
    // fmt::print("{}\n", json[1].dump(2));
    if (json.size() == 0) {
        fmt::print("No changes");
        return;
    }
    struct ChangeBrief {
        int number;
        std::string subject;
        std::string project;
        std::string branch;
        std::string topic;
    };
    std::vector<ChangeBrief> changes;
    changes.reserve(json.size());
    size_t subject_maxlen = 0;
    for (auto change : json) {
        changes.emplace_back(ChangeBrief{
            .number = change.at("_number"),
            .subject = change.at("subject"),
            .project = change.at("project"),
            .branch = change.at("branch"),
            .topic = change.value("topic", ""),
        });
        auto this_subject_len = changes.back().subject.length();
        if (this_subject_len > subject_maxlen) {
            subject_maxlen = this_subject_len;
        }
    }
    fmt::memory_buffer output;
    for (auto change : changes) {
        fmt::format_to(output, "* ");
        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), "{}", change.number));
        fmt::format_to(output, " ");
        fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::yellow), "("));
        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::bright_cyan), change.project));
        fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::yellow), "/"));
        fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::bright_green), "{}",
                                                 change.branch));
        if (!change.topic.empty()) {
            fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::yellow), "/"));
            fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::bright_green),
                                                     "{}", change.topic));
        }
        fmt::format_to(output, "{}", fmt::format(fmt::fg(fmt::terminal_color::yellow), ")"));
        fmt::format_to(output, " {}\n", change.subject);
    }
    fmt::print("{}", fmt::to_string(output));

    // TODO: alignment (including change.number width)
}

/* TODO:
- parse config from file
  - remote:
    - host
    - port
    - default connection type: http/ssh
    - username
    - http password
  - ger change display options and filters:
    - commit-sha1, change-id, project, branch, topic, patch-set number, parent-sha1
    - status, owner, last-updated, size, code-review, verified
    - aligned-columns, parent-graph
    - owner, open, closed, watched, stared, review
  - label the lists when there is more than one filter
- ger profile push/pop from a stack, default uses the ~/.ger file.
*/

int ger(int argc, const char* argv[])
{
    auto command = Command::NONE;

    if (argc <= 1) {
        print_main_help();
        return 0;
    }

    // auto tty = isatty(STDOUT_FILENO);
    // fmt::print(stdout, fmt::fg(fmt::terminal_color::yellow), "Hello");

    std::vector<std::string_view> args{ &argv[1], &argv[argc] };

    std::string_view cmd_arg{ args[0] };
    if (cmd_arg == "change") {
        gsl::span<std::string_view> span_args{ &*(args.begin() + 1), &*args.end() };
        change(span_args);
    } else if (cmd_arg == "help") {
        print_main_help();
    } else {
        print_main_help();
        return 1;
    }

    return 0;
}

} /* namespace ger */
