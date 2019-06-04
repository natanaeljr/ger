@0x8cded0298d2ab273;

using cxx = import "/capnp/c++.capnp";
using json = import "/capnp/compat/json.capnp";
using ListMap = import "/util/listmap.capnp".ListMap;
using accounts = import "/gerrit/accounts.capnp";

$cxx.namespace("gerrit::changes");

##############################################################################

enum HttpMethod {
  get @0 $json.name("GET");
  post @1 $json.name("POST");
  put @2 $json.name("PUT");
  delete @3 $json.name("DELETE");
}

struct ActionInfo {
  method @0 :HttpMethod;
  label @1 :Text;
  title @2 :Text;
  enabled @3 :Bool;
}

struct VoltingRangeInfo {
  min @0 :Int32;
  max @1 :Int32;
}

struct ApprovalInfo {
  account_id @0 :UInt32;
  name @1 :Text;
  email @2 :Text;
  username @3 :Text;
  value @4 :Int32;
  permitted_voting_range @5 :VoltingRangeInfo;
  date @6 :Text;
  tag @7 :Text;
  post_submit @8 :Bool;
}

enum RequirementStatus {
  ok @0 $json.name("OK");
  not_ready @1 $json.name("NOT_READY");
  rule_error @2 $json.name("RULE_ERROR");
}

struct Requirement {
  status @0 :RequirementStatus;
  fallbackText @1 :Text;
  type @2 :Text;
  data @3 :ListMap(Text, Text);
}

enum ReviewValue {
  n2 @0 $json.name("-2")
  n1 @1 $json.name("-1")
  zero @2 $json.name("0")
  p1 @3 $json.name("+1")
  p2 @4 $json.name("+2")
}
struct ReviewValuesKey {
  key @0 :ReviewValue;
}

struct LabelInfo {
  optional @0 :Bool;
  approved @1 :accounts.AccountInfo;
  rejected @2 :accounts.AccountInfo;
  recommended @3 :accounts.AccountInfo;
  disliked @4 :accounts.AccountInfo;
  blocking @5 :Bool;
  value @6 :Text;
  default_value @7 :Int32;
  all @8 :List(ApprovalInfo);
  values @9 :ListMap(ReviewValuesKey, Text);
}

enum ReviewerState {
  reviewer @0 $json.name("REVIEWER");
  cc @1 $json.name("CC");
  removed @2 $json.name("REMOVED");
}
struct ReviewerStateKey {
  key @0 :ReviewerState;
}

struct ReviewerUpdateInfo {
  updated @0 :Text;
  updated_by @1 :accounts.AccountInfo;
  reviewer @2 :accounts.AccountInfo;
  state @3 :ReviewerState;
}

struct ChangeMessageInfo {
  id @0 :Text;
  author @1 :accounts.AccountInfo;
  data @2 :Text;
  message @3 :Text;
  tag @4 :Text;
  revision_number @5 :UInt32;
}

enum RevisionKind {
  rework @0 $json.name("REWORK");
  trivial_rebase @1 $json.name("TRIVIAL_REBASE");
  merge_first_parent_update @2 $json.name("MERGE_FIRST_PARENT_UPDATE");
  no_code_change @3 $json.name("NO_CODE_CHANGE");
  no_change @4 $json.name("NO_CHANGE");
}

struct RevisionInfo {
  draft @0 :Bool;
  kind @1 :RevisionKind;
  number @2 :Text;
  created @4 :Text;
  uploader @5 :accounts.AccountInfo;
  ref @6 :Text;
  # fetch @7 :ListMap(Text, FetchInfo);
  # commit @8 :CommitInfo;
  # files @9 :ListMap(Text, FileInfo);
  actions @7 :ListMap(Text, ActionInfo);
  reviewed @8 :Bool;
  messageWithFooter @9 :Text;
  # push_certificate @10 :PushCertificateInfo;
  description @10 :Text;
}

struct TrackingInfo {
  system @0 :Text;
  id @1 :Text;
}

enum ProblemStatus {
  fixed @0 $json.name("FIXED");
  fix_failed @0 $json.name("FIX_FAILED");
}

struct ProblemInfo {
  message @0 :Text;
  status @1 :ProblemStatus;
  outcome @2 :Text;
}

enum ChangeStatus {
  new @0 $json.name("NEW");
  merged @1 $json.name("MERGED");
  abandoned @2 $json.name("ABANDONED");
  draft @3 $json.name("DRAFT");
}

struct AccountInfo {
  id @0 :UInt32;
  name @1 :Text;
}

struct ChangeInfo {
  id @0 :Text;
  project @1 :Text;
  branch @2 :Text;
  topic @3 :Text;
  assignee @4 :accounts.AccountInfo;
  hashtags @5 :List(Text);
  change_id @6 :Text;
  subject @7 :Text;
  status @8 :ChangeStatus;
  created @9 :Text;
  updated @10 :Text;
  submitted @11 :Text;
  submitter @12 :accounts.AccountInfo;
  starred @13 :Bool;
  stars @14 :List(Text);
  reviewed @15 :Bool;
  submit_type @16 :Text;
  mergeable @17 :Bool;
  insertions @18 :UInt32;
  deletions @19 :UInt32;
  total_comment_count @20 :UInt32;
  unresolved_comment_count @21 :UInt32;
  number @22 :UInt32;
  owner @23 :accounts.AccountInfo;
  actions @24 :ListMap(Text, ActionsInfo);
  requirements @25 :List(Requirement);
  labels @26 :ListMap(Text, LabelInfo);
  permitted_lables @27 :ListMap(Text, List(LabelInfo));
  removable_reviewers @28 :List(accounts.AccountInfo);
  reviewers @29 :ListMap(ReviewerStateKey, List(accounts.AccountInfo));
  pending_reviewers @30 :ListMap(ReviewerStateKey, List(accounts.AccountInfo));
  reviewer_updates @31 :ListMap(Text, ReviewerUpdateInfo);
  messages @32 :List(ChangeMessageInfo);
  current_revision @33 :Text;
  revisions @34 :ListMap(Text, RevisionInfo);
  tracking_ids @35 :List(TrackingIdInfo);
  more_changes @36 :Bool;
  problems @37 :List(ProblemInfo);
  is_private @38 :Bool;
  work_in_progress @39 :Bool;
  has_review_started @40 :Bool;
  revert_of @41 :Text;
  base_change @42 :Text;
}