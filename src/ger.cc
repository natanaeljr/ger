#include <vector>
#include <string>

#include "fmt/core.h"
#include "fmt/ranges.h"
#include "gsl/gsl"
#include "curl/curl.h"
#include "nlohmann/json.hpp"

namespace ger {

enum class Command {
    NONE,
    HELP,
    CHANGES,
};

static void print_help()
{
    fmt::print("Gerrit terminal client.\n");
    fmt::print("USAGE: ger <command>\n\n");
    fmt::print("Commands:\n");
    fmt::print("changes - Show all changes\n");
}

static size_t writeFunction(void* ptr, size_t size, size_t nmemb, std::string* data)
{
    data->append((char*)ptr, size * nmemb);
    return size * nmemb;
}

static void changes(gsl::span<std::string_view> args)
{
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
    curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.42.0");
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
    if (!response_string.compare(0, 3, ")]}'")) {
        fmt::print(stderr, "Unrecognized response from server");
        return;
    }
    auto json = nlohmann::json::parse(response_string.data() + 4);
    fmt::print("{}\n", json[0].dump(2));
}

int ger(int argc, const char* argv[])
{
    auto command = Command::NONE;

    if (argc <= 1) {
        print_help();
        return 1;
    }

    std::vector<std::string_view> args{ &argv[1], &argv[argc] };

    std::string_view cmd_arg{ args[0] };
    if (cmd_arg == "changes") {
        gsl::span<std::string_view> span_args{ &*(args.begin()++), &*args.end() };
        changes(span_args);
    } else if (cmd_arg == "help") {
        print_help();
    } else {
        print_help();
        return 1;
    }

    return 0;
}

} /* namespace ger */
