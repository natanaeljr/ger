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

/************************************************************************************************/

namespace capnp {

template<>
class JsonCodec::Handler<::util::ListMap<capnp::Text, capnp::Text>>
    : public capnp::JsonCodec::Handler<DynamicStruct> {

    using Type = ::util::ListMap<capnp::Text, capnp::Text>;

   public:
    void encode(const capnp::JsonCodec& codec, DynamicStruct::Reader input,
                capnp::JsonValue::Builder output) const override
    {
        //     auto fields = input.getSchema().getFields();
        //     auto items = output.initObject(1);
        //     auto id_field = fields[0];
        //     items[0].setName(input.get(id_field).as<capnp::Text>());

        //     int number = 0;
        //     for (auto field = fields.begin() + 1; field != fields.end(); field++) {
        //         if (input.has(*field)) {
        //             number++;
        //         }
        //     }

        //     auto value = items[0].initValue().initObject(number);

        //     auto index = 0;
        //     for (auto field = fields.begin() + 1; field != fields.end(); field++) {
        //         // KJ_REQUIRE(field->getIndex() <= value.size());
        //         if (input.has(*field)) {
        //             auto item = value[index++];
        //             item.setName(field->getProto().getName());
        //             codec.encode(input.get(*field), field->getType(),
        //             item.initValue());
        //         }
        //         // else {
        //         //     item.setNull();
        //         // }
        //     }
    }

    void decode(const capnp::JsonCodec& codec, capnp::JsonValue::Reader input,
                DynamicStruct::Builder output) const override
    {
        //     auto orphanage = capnp::Orphanage::getForMessageContaining(output);
        //     auto fields = output.getSchema().getFields();
        //     auto items = input.getArray();
        //     for (auto field : fields) {
        //         KJ_REQUIRE(field.getIndex() < items.size());
        //         auto item = items[field.getIndex()];
        //         if (!item.isNull()) {
        //             output.adopt(field, codec.decode(item, field.getType(),
        //             orphanage));
        //         }
        //     }
    }
};

} /* namespace capnp */
