@0x8cded0298d2ab273;

using Cxx = import "/capnp/c++.capnp";
using Json = import "/capnp/compat/json.capnp";
using ListMap = import "/util/listmap.capnp".ListMap;
using Accounts = import "/gerrit/accounts.capnp";

$Cxx.namespace("gerrit::changes");

##############################################################################

enum HttpMethod {
  get @0 $Json.name("GET");
  post @1 $Json.name("POST");
  put @2 $Json.name("PUT");
  delete @3 $Json.name("DELETE");
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
  accountId @0 :UInt32 $Json.name("_account_id");
  name @1 :Text;
  email @2 :Text;
  username @3 :Text;
  value @4 :Int32;
  permittedVotingRange @5 :VoltingRangeInfo $Json.name("permitted_voting_range");
  date @6 :Text;
  tag @7 :Text;
  postSubmit @8 :Bool $Json.name("post_submit");
}

enum RequirementStatus {
  ok @0 $Json.name("OK");
  notReady @1 $Json.name("NOT_READY");
  ruleError @2 $Json.name("RULE_ERROR");
}

struct Requirement {
  status @0 :RequirementStatus;
  fallbackText @1 :Text;
  type @2 :Text;
  data @3 :ListMap(Text, Text);
}

enum ReviewValue {
  n2 @0 $Json.name("-2");
  n1 @1 $Json.name("-1");
  zero @2 $Json.name("0");
  p1 @3 $Json.name("+1");
  p2 @4 $Json.name("+2");
}
struct ReviewValuesKey {
  key @0 :ReviewValue;
}

struct LabelInfo {
  optional @0 :Bool;
  approved @1 :Accounts.AccountInfo;
  rejected @2 :Accounts.AccountInfo;
  recommended @3 :Accounts.AccountInfo;
  disliked @4 :Accounts.AccountInfo;
  blocking @5 :Bool;
  value @6 :Text;
  defaultValue @7 :Int32 $Json.name("default_value");
  all @8 :List(ApprovalInfo);
  values @9 :ListMap(ReviewValuesKey, Text);
}

enum ReviewerState {
  reviewer @0 $Json.name("REVIEWER");
  cc @1 $Json.name("CC");
  removed @2 $Json.name("REMOVED");
}
struct ReviewerStateKey {
  key @0 :ReviewerState;
}

struct ReviewerUpdateInfo {
  updated @0 :Text;
  updatedBy @1 :Accounts.AccountInfo $Json.name("updated_by");
  reviewer @2 :Accounts.AccountInfo;
  state @3 :ReviewerState;
}

struct ChangeMessageInfo {
  id @0 :Text;
  author @1 :Accounts.AccountInfo;
  data @2 :Text;
  message @3 :Text;
  tag @4 :Text;
  revisionNumber @5 :UInt32 $Json.name("_revision_number");
}

enum RevisionKind {
  rework @0 $Json.name("REWORK");
  trivialRebase @1 $Json.name("TRIVIAL_REBASE");
  mergeFirstParentUpdate @2 $Json.name("MERGE_FIRST_PARENT_UPDATE");
  noCodeChange @3 $Json.name("NO_CODE_CHANGE");
  noChange @4 $Json.name("NO_CHANGE");
}

struct GitPersonInfo {
  name @0 :Text;
  email @1 :Text;
  date @2 :Text;
  tz @3 :Int32;
}

struct WebLinkInfo {
  name @0 :Text;
  url @1 :Text;
  imageUrl @2 :Text $Json.name("image_url");
}

enum FileStatus {
  modified @0 $Json.name("M");
  added @1 $Json.name("A");
  deleted @2 $Json.name("D");
  renamed @3 $Json.name("R");
  copied @4 $Json.name("C");
  rewritten @5 $Json.name("W");
}

struct FetchInfo {
  url @0 :Text;
  ref @1 :Text;
  commands @2 :ListMap(Text, Text);
}

struct CommitInfo {
  commit @0 :Text;
  parents @1 :List(CommitInfo);
  author @2 :GitPersonInfo;
  committer @3 :GitPersonInfo;
  subject @4 :Text;
  message @5 :Text;
  webLinks @6 :List(WebLinkInfo) $Json.name("web_links");
}

struct FileInfo {
  status @0 :FileStatus;
  binary @1 :Bool;
  oldPath @2 :Text $Json.name("old_path");
  linesInserted @3 :UInt32 $Json.name("lines_inserted");
  linesDeleted @4 :UInt32 $Json.name("lines_deleted");
  sizeDelta @5 :UInt32 $Json.name("size_delta");
  size @6 :UInt32;
}

struct PushCertificateInfo {
  certificate @0 :Text;
  key @1 :Accounts.GpgKeyInfo; }

struct RevisionInfo {
  draft @0 :Bool;
  kind @1 :RevisionKind;
  number @2 :Text $Json.name("_number");
  created @3 :Text;
  uploader @4 :Accounts.AccountInfo;
  ref @5 :Text;
  fetch @6 :ListMap(Text, FetchInfo);
  commit @7 :CommitInfo;
  files @8 :ListMap(Text, FileInfo);
  actions @9 :ListMap(Text, ActionInfo);
  reviewed @10 :Bool;
  messageWithFooter @11 :Text;
  pushCertificate @12 :PushCertificateInfo $Json.name("push_certificate");
  description @13 :Text;
}

struct TrackingIdInfo {
  system @0 :Text;
  id @1 :Text;
}

enum ProblemStatus {
  fixed @0 $Json.name("FIXED");
  fixFailed @1 $Json.name("FIX_FAILED");
}

struct ProblemInfo {
  message @0 :Text;
  status @1 :ProblemStatus;
  outcome @2 :Text;
}

enum ChangeStatus {
  new @0 $Json.name("NEW");
  merged @1 $Json.name("MERGED");
  abandoned @2 $Json.name("ABANDONED");
  draft @3 $Json.name("DRAFT");
}

enum SubmitType {
  mergeIfNecessary @0 $Json.name("MERGE_IF_NECESSARY");
  fastForwardOnly @1 $Json.name("FAST_FORWARD_ONLY");
  rebaseIfNecessary @2 $Json.name("REBASE_IF_NECESSARY");
  rebaseAlways @3 $Json.name("REBASE_ALWAYS");
  mergeAlways @4 $Json.name("MERGE_ALWAYS");
  cherryPick @5 $Json.name("CHERRY_PICK");
}

struct ChangeInfo {
  id @0 :Text;
  project @1 :Text;
  branch @2 :Text;
  topic @3 :Text;
  assignee @4 :Accounts.AccountInfo;
  hashtags @5 :List(Text);
  changeId @6 :Text $Json.name("change_id");
  subject @7 :Text;
  status @8 :ChangeStatus;
  created @9 :Text;
  updated @10 :Text;
  submitted @11 :Text;
  submitter @12 :Accounts.AccountInfo;
  starred @13 :Bool;
  stars @14 :List(Text);
  reviewed @15 :Bool;
  submitType @16 :SubmitType $Json.name("submit_type");
  mergeable @17 :Bool;
  insertions @18 :UInt32;
  deletions @19 :UInt32;
  totalCommentCount @20 :UInt32 $Json.name("total_comment_count");
  unresolvedCommentCount @21 :UInt32 $Json.name("unresolved_comment_count");
  number @22 :UInt32 $Json.name("_number");
  owner @23 :Accounts.AccountInfo;
  actions @24 :ListMap(Text, ActionInfo);
  requirements @25 :List(Requirement);
  labels @26 :ListMap(Text, LabelInfo);
  permittedLables @27 :ListMap(Text, List(LabelInfo)) $Json.name("permitted_lables");
  removableReviewers @28 :List(Accounts.AccountInfo) $Json.name("removable_reviewers");
  reviewers @29 :ListMap(ReviewerStateKey, List(Accounts.AccountInfo));
  pendingReviewers @30 :ListMap(ReviewerStateKey, List(Accounts.AccountInfo)) $Json.name("pending_reviewers");
  reviewerUpdates @31 :ListMap(Text, ReviewerUpdateInfo) $Json.name("reviewer_updates");
  messages @32 :List(ChangeMessageInfo);
  currentRevision @33 :Text $Json.name("current_revision");
  revisions @34 :ListMap(Text, RevisionInfo);
  trackingIds @35 :List(TrackingIdInfo) $Json.name("tracking_ids");
  moreChanges @36 :Bool $Json.name("more_changes");
  problems @37 :List(ProblemInfo);
  isPrivate @38 :Bool $Json.name("is_private");
  workInProgress @39 :Bool $Json.name("work_in_progress");
  hasReviewStarted @40 :Bool $Json.name("has_review_started");
  revertOf @41 :Text $Json.name("revert_of");
  baseChange @42 :Text $Json.name("base_change");
}