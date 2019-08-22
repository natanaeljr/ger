/**
 * \file change_cmd.cc
 * \author Natanael Josue Rabello
 * \brief Change command.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/command.h"

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
namespace cli {

struct QueryOpts {
};

/************************************************************************************************/
static constexpr const char kGerChangeCmdHelp[] = R"(usage: change [options] [<change>]

List changes in the gerrit server.

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
static std::string RequestJson(std::string_view url, std::string_view userauth,
                               const bool verbose)
{
    CURL* curl = nullptr;
    CURLcode res;

    curl_global_init(CURL_GLOBAL_ALL);
    auto _clean_global_curl = gsl::finally([] { curl_global_cleanup(); });

    curl = curl_easy_init();
    if (!curl) {
        fmt::print(stderr, "Failed to init easy curl\n");
        return {};
    }
    auto _clean_easy_curl = gsl::finally([curl] { curl_easy_cleanup(curl); });

    if (verbose) {
        fmt::print("quering server at {}\n", url);
        fflush(stdout);
    }

    // curl_easy_setopt(curl, CURLOPT_URL,
    //                  "localhost:8080/a/changes/?q=is:open+owner:self&o=DETAILED_LABELS");
    curl_easy_setopt(curl, CURLOPT_URL, url.data());
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);

    if (verbose) {
        fmt::print("user authentication: {}\n", userauth);
        fflush(stdout);
    }

    // curl_easy_setopt(curl, CURLOPT_USERPWD,
    //                  "natanaeljr:ot+XfXZockCTMWs9A0yfPtnUgMT52rbQ2NZaG9M17w");
    curl_easy_setopt(curl, CURLOPT_USERPWD, userauth.data());
    curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_DIGEST);
    // curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.42.0");
    // curl_easy_setopt(curl, CURLOPT_MAXREDIRS, 50L);
    // curl_easy_setopt(curl, CURLOPT_TCP_KEEPALIVE, 1L);

    std::string response_string;
    std::string header_string;
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeFunction);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response_string);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &header_string);

    char* effect_url;
    long response_code;
    double elapsed;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
    curl_easy_getinfo(curl, CURLINFO_TOTAL_TIME, &elapsed);
    curl_easy_getinfo(curl, CURLINFO_EFFECTIVE_URL, &effect_url);

    /* Perform the request, res will get the return code */
    res = curl_easy_perform(curl);
    /* Check for errors */
    if (res != CURLE_OK) {
        fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        return {};
    }

    return response_string;
}

static capnp::Orphan<capnp::List<gerrit::changes::ChangeInfo>> ParseChanges(
    std::string_view json_input, capnp::Orphanage orphanage)
{
    capnp::JsonCodec codec;
    codec.handleByAnnotation<gerrit::changes::HttpMethod>();
    codec.handleByAnnotation<gerrit::changes::ApprovalInfo>();
    codec.handleByAnnotation<gerrit::changes::RequirementStatus>();
    codec.handleByAnnotation<gerrit::changes::ReviewValue>();
    codec.handleByAnnotation<gerrit::changes::LabelInfo>();
    codec.handleByAnnotation<gerrit::changes::ReviewerState>();
    codec.handleByAnnotation<gerrit::changes::ReviewerUpdateInfo>();
    codec.handleByAnnotation<gerrit::changes::ChangeMessageInfo>();
    codec.handleByAnnotation<gerrit::changes::RevisionKind>();
    codec.handleByAnnotation<gerrit::changes::ProblemStatus>();
    codec.handleByAnnotation<gerrit::changes::ChangeStatus>();
    codec.handleByAnnotation<gerrit::changes::ChangeInfo>();

    auto orphan = codec.decode<capnp::List<gerrit::changes::ChangeInfo>>(
        { json_input.begin(), json_input.end() }, orphanage);

    return orphan;
}

/************************************************************************************************/
int RequestOneChange(uint32_t number, const Remote& remote, const bool verbose)
{

    std::string url = fmt::format(
        "{}/a/changes/?q=change:{}&o=CURRENT_REVISION&o=CURRENT_COMMIT&o=CURRENT_FILES",
        remote.url, number);
    std::string userauth = fmt::format("{}:{}", remote.username, remote.http_password);

    std::string response = RequestJson(url, userauth, verbose);
    if (response.empty()) {
        return -1;
    }

    constexpr std::string_view kMagicPrefix = ")]}'\n";
    if (response.compare(0, kMagicPrefix.length(), kMagicPrefix) != 0) {
        fmt::print(stderr, "Unrecognized response from server:\n\n{}", response);
        return -1;
    }

    fmt::print("{}", response);

    capnp::MallocMessageBuilder arena;
    auto orphan = ParseChanges(response.data() + 5, arena.getOrphanage());
    auto changes = orphan.getReader();

    if (changes.size() == 0) {
        fmt::print("No changes.");
        return 0;
    }

    for (auto change : changes) {
        assert(change.getRevisions().hasEntries());
        auto current_revision = change.getRevisions().getEntries().begin()->getValue();
        fmt::print("{} {}\n{}\n",
                   fmt::format(fmt::fg(fmt::terminal_color::yellow), "change {}",
                               change.getNumber()),
                   fmt::format(fmt::fg(fmt::terminal_color::bright_green), "{}",
                               change.getBranch().cStr()),
                   current_revision.getMessageWithFooter().cStr());
    }

    return 0;
}

/************************************************************************************************/
int RunChangeCommand(const std::vector<std::string>& argv, const Remote& remote,
                     const bool verbose)
{
    /* Parse arguments */
    auto args = docopt::docopt(kGerChangeCmdHelp, argv, true, {}, true);
    if (args["<change>"]) {
        RequestOneChange(args["<change>"].asLong(), remote, verbose);
    }
    else {
        fmt::print(stderr, "work in progress");
    }

    return 0;
}

} /* namespace cli */
} /* namespace ger */
