/**
 * \file
 * \author Natanael Josue Rabello
 * \brief Json Codec.
 * \date 2019-05-24
 * \copyright Copyright (c) 2019
 */

#pragma once

#include "capnp/list.h"
#include "capnp/compat/json.h"
#include "util/listmap_handler.h"
#include "gerrit/changes.capnp.h"
#include "gerrit/accounts.capnp.h"

/**************************************************************************************/

namespace ger {

class JsonCodec : public ::capnp::JsonCodec {
   public:
    /** Constructor */
    JsonCodec() noexcept(false)
    {
        // handleByAnnotation<gerrit::changes::HttpMethod>();
        // handleByAnnotation<gerrit::changes::ApprovalInfo>();
        // handleByAnnotation<gerrit::changes::RequirementStatus>();
        // handleByAnnotation<gerrit::changes::ReviewValue>();
        // handleByAnnotation<gerrit::changes::LabelInfo>();
        // handleByAnnotation<gerrit::changes::ReviewerState>();
        // handleByAnnotation<gerrit::changes::ReviewerUpdateInfo>();
        // handleByAnnotation<gerrit::changes::ChangeMessageInfo>();
        // handleByAnnotation<gerrit::changes::RevisionKind>();
        // handleByAnnotation<gerrit::changes::ProblemStatus>();
        handleByAnnotation<gerrit::changes::ChangeStatus>();
        // handleByAnnotation<gerrit::changes::ChangeInfo>();
        // handleByAnnotation<gerrit::changes::WebLinkInfo>();
        // handleByAnnotation<gerrit::changes::FileStatus>();
        // handleByAnnotation<gerrit::changes::FileInfo>();
        // handleByAnnotation<gerrit::changes::CommitInfo>();
        // handleByAnnotation<gerrit::changes::RevisionInfo>();

        // addTypeHandler(listmap_handler1_);
        // addTypeHandler(listmap_handler2_);
        // addTypeHandler(listmap_handler3_);
        // addTypeHandler(listmap_handler4_);
        // addTypeHandler(listmap_handler5_);
        // addTypeHandler(listmap_handler6_);
    }

    ~JsonCodec() noexcept(false) {}

   private:
    // ::capnp::ListMapJsonCodecHandler<gerrit::changes::ReviewerStateKey,
    //                                  ::capnp::List<gerrit::accounts::AccountInfo>>
    //     listmap_handler1_;
    // ::capnp::ListMapJsonCodecHandler<capnp::Text, capnp::Text> listmap_handler2_;
    // ::capnp::ListMapJsonCodecHandler<capnp::Text, gerrit::changes::RevisionInfo>
    //     listmap_handler3_;
    // ::capnp::ListMapJsonCodecHandler<capnp::Text, gerrit::changes::FetchInfo>
    //     listmap_handler4_;
    // ::capnp::ListMapJsonCodecHandler<capnp::Text, gerrit::changes::FileInfo>
    //     listmap_handler5_;
    // ::capnp::ListMapJsonCodecHandler<capnp::Text, gerrit::changes::ActionInfo>
    //     listmap_handler6_;
};

} /* namespace ger */
