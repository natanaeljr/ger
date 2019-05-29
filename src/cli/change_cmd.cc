/**
 * \file change_cmd.cc
 * \author Natanael Josue Rabello
 * \brief Change command.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/commands.h"

#include <unistd.h>
#include <capnp/compat/json.h>

#include "fmt/core.h"
#include "fmt/ranges.h"
#include "fmt/printf.h"
#include "fmt/format.h"
#include "fmt/color.h"
#include "gsl/gsl"
#include "curl/curl.h"
#include "nlohmann/json.hpp"
#include "docopt.h"

#include "gerrit/changes.capnp.h"
#include "util/listmap_handler.h"

namespace ger {

/************************************************************************************************/
static constexpr const char kGerChangeCmdHelp[] = R"(Ger Change command.
usage: change [-h|--help] [<change>]

positional arguments:
  <change>        Show information about a specific change.

options:
  -h, --help      Show this screen.)";

/************************************************************************************************/
static size_t writeFunction(void* ptr, size_t size, size_t nmemb, std::string* data)
{
    data->append((char*) ptr, size * nmemb);
    return size * nmemb;
}

/************************************************************************************************/
int RunChangeCommand(const std::vector<std::string>& argv)
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerChangeCmdHelp, argv, true, {}, true);

    if (args["<change>"]) {
        fmt::print("Arguments not yet implemented.\n");
        return -1;
    }

    CURL* curl = nullptr;
    CURLcode res;

    curl_global_init(CURL_GLOBAL_ALL);
    auto _clean_global_curl = gsl::finally([] { curl_global_cleanup(); });

    curl = curl_easy_init();
    if (!curl) {
        fmt::print(stderr, "Failed to init easy curl\n");
        return -1;
    }
    auto _clean_easy_curl = gsl::finally([&] { curl_easy_cleanup(curl); });

    // curl_easy_setopt(curl, CURLOPT_URL,
    //                  "localhost:8080/a/changes/?q=is:open+owner:self&o=DETAILED_LABELS");
    curl_easy_setopt(curl, CURLOPT_URL,
                     "https://gerrit.ped.datacom.ind.br/a/changes/?q=is:open+owner:self");
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);

    // curl_easy_setopt(curl, CURLOPT_USERPWD,
    //                  "natanaeljr:ot+XfXZockCTMWs9A0yfPtnUgMT52rbQ2NZaG9M17w");
    curl_easy_setopt(curl, CURLOPT_USERPWD,
                     "natanael.rabello.cwi:9of//kYGdM8g3PDcYL2JAHncMRwQ2algDYlgE2CsdA");
    curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_DIGEST);
    // curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.42.0");
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
        return -1;
    }

    // SUCCESS
    if (response_string.compare(0, 5, ")]}'\n")) {
        fmt::print(stderr, "Unrecognized response from server:\n\n{}", response_string);
        return -2;
    }
    auto json = nlohmann::json::parse(response_string.data() + 4);
    // fmt::print("{}\n", json[0].dump(2));
    if (json.size() == 0) {
        fmt::print("No changes");
        return 0;
    }

    {
        capnp::MallocMessageBuilder message;
        auto change_build = message.initRoot<gerrit::changes::ChangeInfo>();
        // change_build.setId("abcde");
        // change_build.setNumber(12345);
        capnp::JsonCodec json_codec;
        json_codec.handleByAnnotation<gerrit::changes::ChangeStatus>();
        json_codec.handleByAnnotation<gerrit::changes::ReviewerState>();
        // json_codec.setPrettyPrint(true);
        auto listmap_handler1 =
            capnp::ListMapJsonCodecHandler<gerrit::changes::ReviewerStateKey,
                                           ::capnp::List<gerrit::changes::AccountInfo>>();
        auto listmap_handler2 =
            capnp::ListMapJsonCodecHandler<::capnp::Text, ::capnp::Text>();
        json_codec.addTypeHandler(listmap_handler1);
        json_codec.addTypeHandler(listmap_handler2);

        // const char input[] = R"({"id": "other", "_number": 404, "status": "draft"})";
        // json_codec.decode(input, change_build);
        // auto values = change_build.initValues(2);
        // values.set(0, "first");
        // values.set(1, "second");
        // auto json = change_build.initJson();
        // json.setName("CC");
        // json.initValue().setNumber(5);
        change_build.setId("mydumbid");
        change_build.setProject("mydrone");
        auto reviewers = change_build.initReviewers();
        {
            auto entries = reviewers.initEntries(2);
            entries[0].initKey().setKey(::gerrit::changes::ReviewerState::REVIEWER);
            {
                auto values = entries[0].initValue(1);
                values[0].setId(1);
                values[0].setName("joao");
            }
            entries[1].initKey().setKey(::gerrit::changes::ReviewerState::CC);
            {
                auto values = entries[1].initValue(2);
                values[0].setId(1);
                values[0].setName("marcos");
                values[1].setId(2);
                values[1].setName("lucas");
            }
        }
        auto others = change_build.initOthers();
        {
            auto entries = others.initEntries(2);
            entries[0].setKey("kkkkk");
            entries[0].setValue("hello world");
            entries[1].setKey("ttttt");
            entries[1].setValue("hello world");
        }

        kj::String string = json_codec.encode(change_build.asReader());
        // fmt::print("encode: {}\n", string.cStr());
        auto json = nlohmann::json::parse(string.cStr());
        fmt::print("{}\n", json.dump(2));
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

    // TODO: output all in bold: (project/branch/topic)
    // TODO: place (project/branch/topic) before subject, link git
    fmt::memory_buffer output;
    for (auto change : changes) {
        /* number */
        fmt::format_to(output, "* ");
        fmt::format_to(
            output, "{} ",
            fmt::format(fmt::fg(fmt::terminal_color::yellow), "{}", change.number));

        /* project/branch/topic */
        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), "("));
        fmt::format_to(
            output, "{}",
            fmt::format(fmt::fg(fmt::terminal_color::bright_cyan) | fmt::emphasis::bold,
                        change.project));
        fmt::format_to(
            output, "{}",
            fmt::format(fmt::fg(fmt::terminal_color::bright_green) | fmt::emphasis::bold,
                        "{}", "/" + change.branch));
        if (!change.topic.empty()) {
            fmt::format_to(output, "{}",
                           fmt::format(fmt::fg(fmt::terminal_color::bright_green) |
                                           fmt::emphasis::bold,
                                       "{}", "/" + change.topic));
        }
        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), ")"));

        /* subject */
        fmt::format_to(output, " {}\n", change.subject);
    }
    fmt::print("{}", fmt::to_string(output));

    return 0;
}

} /* namespace ger */
