/**
 * \file list_map_handler.h
 * \author Natanael Josue Rabello
 * \brief ListMap Handler for JsonCodec.
 * \date 2019-05-28
 * \copyright Copyright (c) 2019
 */

#pragma once

#include <capnp/compat/json.h>
#include "util/listmap.capnp.h"

#include <assert.h>

/************************************************************************************************/

namespace capnp {

template<typename Key, typename Value>
class JsonCodec::Handler<::util::ListMap<Key, Value>, Style::STRUCT>
    : public JsonCodec::Handler<DynamicStruct> {

    static_assert(kind<Key>() == Kind::STRUCT || kind<Key>() == Kind::BLOB,
                  "ListMap::Key type must be pointer type");

    using ListMapType = ::util::ListMap<Key, Value>;

    virtual void encode(const JsonCodec& codec, DynamicStruct::Reader input,
                        JsonValue::Builder output) const override
    {
        encode(codec, input.as<ListMapType>(), output);
    }
    virtual void decode(const JsonCodec& codec, JsonValue::Reader input,
                        DynamicStruct::Builder output) const override
    {
        decode(codec, input, output.as<ListMapType>());
    }

    friend class JsonCodec;

   public:
    inline void encode_key(const JsonCodec& codec, DynamicValue::Reader field,
                           JsonValue::Builder output) const
    {
        switch (field.getType()) {
            case DynamicValue::TEXT: {
                codec.encode(field, Schema::from<Text>(), output);
                break;
            }
            case DynamicValue::ENUM: {
                codec.encode(field, field.as<DynamicEnum>().getSchema(), output);
                break;
            }
            case DynamicValue::STRUCT: {
                auto s = field.as<DynamicStruct>();
                auto fs = s.getSchema().getFields();
                encode_key(codec, s.get(*fs.begin()), output);
                break;
            }
            default: break;
        }
    }

    inline void decode_key(const JsonCodec& codec, Text::Reader text,
                           DynamicStruct::Builder output) const
    {
        auto field = *output.getSchema().getFields().begin();
        switch (field.getType().which()) {
            case schema::Type::Which::TEXT: {
                auto orphanage = Orphanage::getForMessageContaining(output);
                output.adopt(field, orphanage.newOrphanCopy(text));
                break;
            }
            case schema::Type::Which::ENUM: {
                auto orphanage = Orphanage::getForMessageContaining(output);
                auto j = orphanage.newOrphan<JsonValue>();
                j.get().setString(text);
                output.adopt(field, codec.decode(j.get(), field.getType(), orphanage));
                break;
            }
            case schema::Type::Which::STRUCT: {
                decode_key(codec, text, output.init(field).as<DynamicStruct>());
                break;
            }
            default: break;
        }
    }

    inline void encode(const JsonCodec& codec, typename ListMapType::Reader input,
                       JsonValue::Builder output) const
    {
        if (input.hasEntries()) {
            auto in_entries = input.getEntries();
            auto out_entries = output.initObject(in_entries.size());
            for (size_t i = 0; i < in_entries.size(); ++i) {
                auto orphanage = Orphanage::getForMessageContaining(output);
                auto orphan = orphanage.newOrphan<JsonValue>();
                encode_key(codec, in_entries[i].getKey(), orphan.get());
                out_entries[i].setName(orphan.get().asReader().getString());
                codec.encode(in_entries[i].getValue(), out_entries[i].initValue());
            }
        }
    }

    inline void decode(const JsonCodec& codec, JsonValue::Reader input,
                       typename ListMapType::Builder output) const
    {
        if (input.hasObject()) {
            auto in_fields = input.getObject();
            auto out_entries = output.initEntries(in_fields.size());
            auto out_entry = out_entries.begin();
            for (auto in_field : in_fields) {
                decode_key(codec, in_field.getName(), *out_entry);
                auto d = DynamicStruct::Builder(*out_entry);
                auto orphanage = Orphanage::getForMessageContaining(output);
                d.adopt("value", codec.decode(in_field.getValue(), Type::from<Value>(),
                                              orphanage));
                out_entry++;
            }
        }
    }
};

template<typename Key, typename Value>
using ListMapJsonCodecHandler =
    JsonCodec::Handler<::util::ListMap<Key, Value>, Style::STRUCT>;

} /* namespace capnp */
