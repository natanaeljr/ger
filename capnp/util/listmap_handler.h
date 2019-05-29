/**
 * \file list_map_handler.h
 * \author Natanael Josue Rabello
 * \brief ListMap Handler for JsonCodec.
 * \date 2019-05-28
 * \copyright Copyright (c) 2019
 */

#pragma once

#include <tuple>
#include <type_traits>
#include <capnp/compat/json.h>
#include "util/listmap.capnp.h"

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
    inline void GetKeyString(const JsonCodec& codec, DynamicValue::Reader field,
                             JsonValue::Builder output) const
    {
        switch (field.getType()) {
            case DynamicValue::TEXT: {
                codec.encode(field.as<Text>(), output);
            }
            case DynamicValue::ENUM: {
                codec.encode(field.as<DynamicEnum>(), output);
            }
            case DynamicValue::STRUCT: {
                auto s = field.as<DynamicStruct>();
                auto fs = s.getSchema().getFields();
                GetKeyString(codec,s.get(*fs.begin()), output);
            }
            default: break;
        }
    }

    inline void encode(const JsonCodec& codec, typename ListMapType::Reader input,
                       JsonValue::Builder output) const
    {
        if (input.hasEntries()) {
            auto entries = input.getEntries();
            auto out_entries = output.initObject(entries.size());
            for (size_t i = 0; i < entries.size(); ++i) {
                auto o = Orphanage();
                auto x = o.newOrphan<JsonValue>().get();
                GetKeyString(codec, entries[i].getKey(), x);
                codec.encode(entries[i].getValue(), out_entries[i].initValue());
            }
        }
    }

    inline void decode(const JsonCodec& codec, JsonValue::Reader input,
                       typename ListMapType::Builder output) const
    {
    }
};

template<typename Key, typename Value>
using ListMapJsonCodecHandler =
    JsonCodec::Handler<::util::ListMap<Key, Value>, Style::STRUCT>;

} /* namespace capnp */
