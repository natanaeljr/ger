/**
 * \file change_cmd.cc
 * \author Natanael Josue Rabello
 * \brief Change command.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#include "ger/cli/commands.h"

#include <unistd.h>
// #include <google/protobuf/util/json_util.h>
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

// #include "gerrit/changes.pb.h"
// #include "gerrit/accounts.pb.h"
#include "gerrit/changes.capnp.h"

namespace capnp {
class MapJsonCodeHandler : public JsonCodec::Handler<DynamicStruct> {
    // Almost identical to Style::STRUCT except that we pass the struct type to decode().

   public:
    virtual void encode(const JsonCodec& codec, DynamicStruct::Reader input,
                        JsonValue::Builder output) const override
    {
        auto fields = input.getSchema().getFields();
        auto items = output.initObject(1);
        auto id_field = fields[0];
        items[0].setName(input.get(id_field).as<capnp::Text>());

        int number = 0;
        for (auto field = fields.begin() + 1; field != fields.end(); field++) {
            if (input.has(*field)) {
                number++;
            }
        }

        auto value = items[0].initValue().initObject(number);

        auto index = 0;
        for (auto field = fields.begin() + 1; field != fields.end(); field++) {
            // KJ_REQUIRE(field->getIndex() <= value.size());
            if (input.has(*field)) {
                auto item = value[index++];
                item.setName(field->getProto().getName());
                codec.encode(input.get(*field), field->getType(), item.initValue());
            }
            // else {
            //     item.setNull();
            // }
        }
    }
    virtual void decode(const JsonCodec& codec, JsonValue::Reader input,
                        DynamicStruct::Builder output) const override
    {
        auto orphanage = Orphanage::getForMessageContaining(output);
        auto fields = output.getSchema().getFields();
        auto items = input.getArray();
        for (auto field : fields) {
            KJ_REQUIRE(field.getIndex() < items.size());
            auto item = items[field.getIndex()];
            if (!item.isNull()) {
                output.adopt(field, codec.decode(item, field.getType(), orphanage));
            }
        }
    }

   private:
    //   void encodeBase(const JsonCodec& codec, DynamicValue::Reader input,
    //                   JsonValue::Builder output) const override final {
    //     encode(codec, input.as<DynamicStruct>(), output);
    //   }
    //   Orphan<DynamicValue> decodeBase(const JsonCodec& codec, JsonValue::Reader input,
    //                                   Type type, Orphanage orphanage) const override
    //                                   final {
    //     return decode(codec, input, type.asStruct(), orphanage);
    //   }
    //   void decodeStructBase(const JsonCodec& codec, JsonValue::Reader input,
    //                         DynamicStruct::Builder output) const override final {
    //     decode(codec, input, output.as<DynamicStruct>());
    //   }
    friend class JsonCodec;
};

class MyMapJsonCodeHandler : public JsonCodec::Handler<gerrit::changes::ChangeInfo> {
   public:
    void encode(const JsonCodec& codec, gerrit::changes::ChangeInfo::Reader input,
                JsonValue::Builder output) const override
    {
        dynamicHandler.encode(codec, input, output);
    }

    void decode(const JsonCodec& codec, JsonValue::Reader input,
                gerrit::changes::ChangeInfo::Builder output) const override
    {
        dynamicHandler.decode(codec, input, output);
    }

   private:
    MapJsonCodeHandler dynamicHandler;
};
}  // namespace capnp

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

/*
static google::protobuf::util::Status ConvertProtobufMessages(
    const google::protobuf::Message& from_msg, google::protobuf::Message* to_msg)
{
    std::string json;
    auto status = google::protobuf::util::MessageToJsonString(from_msg, &json);
    if (not status.ok()) {
        return status;
    }
    status = google::protobuf::util::JsonStringToMessage(json, to_msg);
    return status;
}

template<typename T>
static google::protobuf::util::Status ConvertProtobufMessages(
    const google::protobuf::ListValue& list_value,
    google::protobuf::RepeatedPtrField<T>* msgs)

{
    for (auto& value : list_value.values()) {
        auto status = ConvertProtobufMessages(value, msgs->Add());
        if (not status.ok()) {
            return status;
        }
    }
    return {};
}
*/

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

    curl_easy_setopt(curl, CURLOPT_URL,
                     "localhost:8080/a/changes/?q=is:open+owner:self&o=DETAILED_LABELS");
    // curl_easy_setopt(curl, CURLOPT_URL,
    //                  "https://gerrit.ped.datacom.ind.br/a/changes/?q=is:open+owner:self");
    // curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    // curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);

    curl_easy_setopt(curl, CURLOPT_USERPWD,
                     "natanaeljr:ot+XfXZockCTMWs9A0yfPtnUgMT52rbQ2NZaG9M17w");
    // curl_easy_setopt(curl, CURLOPT_USERPWD,
    //                  "natanael.rabello.cwi:9of//kYGdM8g3PDcYL2JAHncMRwQ2algDYlgE2CsdA");
    // curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.42.0");
    // curl_easy_setopt(curl, CURLOPT_HTTPAUTH, CURLAUTH_DIGEST);
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
    fmt::print("{}\n", json[0].dump(2));
    if (json.size() == 0) {
        fmt::print("No changes");
        return 0;
    }

    /*{
        using google::protobuf::ListValue;
        using google::protobuf::RepeatedPtrField;

        google::protobuf::ListValue change_list;
        auto sts = google::protobuf::util::JsonStringToMessage(response_string.data() + 4,
                                                               &change_list);
        if (not sts.ok()) {
            fmt::print("sts({}):{}\n", __LINE__, sts.ToString());
            return -3;
        }
        // change_list.PrintDebugString();
        google::protobuf::RepeatedPtrField<gerrit::changes::ChangeInfo> changes;
        sts = ConvertProtobufMessages(change_list, &changes);
        if (not sts.ok()) {
            fmt::print("sts({}):{}\n", __LINE__, sts.ToString());
            return -3;
        }
        changes.begin()->PrintDebugString();
    }*/

    {
        capnp::MallocMessageBuilder message;
        auto change_build = message.initRoot<gerrit::changes::ChangeInfo>();
        // change_build.setId("abcde");
        // change_build.setNumber(12345);
        capnp::JsonCodec json_codec;
        json_codec.handleByAnnotation<gerrit::changes::ChangeStatus>();
        // json_codec.setPrettyPrint(true);
        auto more = capnp::MyMapJsonCodeHandler();
        json_codec.addTypeHandler(more);

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

        // change_build.initAne().initAs<gerrit::changes::ChangeInfo>();

        kj::String string = json_codec.encode(change_build.asReader());
        fmt::print("encode:\n{}\n", string.cStr());
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
        fmt::format_to(
            output, "{}",
            fmt::format(fmt::fg(fmt::terminal_color::yellow), "{}", change.number));
        fmt::format_to(output, " {} ", change.subject);

        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), "("));
        fmt::format_to(
            output, "{}",
            fmt::format(fmt::fg(fmt::terminal_color::bright_cyan), change.project));
        fmt::format_to(output, "{}",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), "/"));
        fmt::format_to(
            output, "{}",
            fmt::format(fmt::fg(fmt::terminal_color::bright_green), "{}", change.branch));
        if (!change.topic.empty()) {
            fmt::format_to(output, "{}",
                           fmt::format(fmt::fg(fmt::terminal_color::yellow), "/"));
            fmt::format_to(output, "{}",
                           fmt::format(fmt::fg(fmt::terminal_color::bright_green), "{}",
                                       change.topic));
        }
        fmt::format_to(output, "{}\n",
                       fmt::format(fmt::fg(fmt::terminal_color::yellow), ")"));
    }
    fmt::print("{}", fmt::to_string(output));

    return 0;
}

} /* namespace ger */
