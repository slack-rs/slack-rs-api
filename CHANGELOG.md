# 0.18.0
* Serde 1.0.0 **breaking change, your serde must also be ~1.0.0**
* Updates to API to include fields for "Threading messages" and a couple of other missing fields in messages

# 0.17.0
* Updates generated API definitions for the Slack API as of April 29

## API Changes
* Several additional error types were added across many API functions
* Several documentation fixes and updates

### `channels`
* `create`, `join` and `rename` had a new `validate` field added
* `list` had a new `exclude_members` field added

### `chat`
* `unfurl` was added. See [https://api.slack.com/methods/chat.unfurl](https://api.slack.com/methods/chat.unfurl)

### `files_comments`
* `add` had the `channel` parameter removed

### `groups`
* `create` and `rename` had a new `validate` field added

### `ims`
* `open` had its `return_im` field change from `Option<&str>` to `Option<bool>`

### `reactions`
* `list` had its `full` field change from `Option<&str>` to `Option<bool>`

### `rtm`
* `connect` was added. See [https://api.slack.com/methods/rtm.connect](https://api.slack.com/methods/rtm.connect)
* `start` had its `no_unreads` and `mpim_aware` fields change from `Option<&str>` to `Option<bool>`
* `start` had a new `no_latest` field added

# 0.16.2
* This release adds a new top-level function: `default_client()`
  * This function can be used by users to get a default client that implements `SlackWebRequestSender` so they don't have to pull in `reqwest` and version match in order to send requests.
  * Thanks to @bwasty for adding this in #42!

# 0.16.1
* This release works around a Slack API bug causing deserialization issues
  * Thanks to @dten for implementing the fix in #39!

# 0.16.0
- This release contains **several breaking changes**
  - `hyper` has been replaced with `reqwest`, but is still an optional dependency
  - All response fields are now considered optional to prevent this library from breaking if Slack removes them
  - All methods now take in a request object (e.g. `PostMessageRequest`) instead of each parameter separately
    - This allows better clarity when calling the API as it simulates named params
    - All request objects implement `Default`, so you can fill in the request with a `..PostMessageRequest::default()` (or equivalent) to elide fields that are not being set
  - All error types have been reworked to obtain better information from the Slack API about the issue
- Support added for most other Slack APIs, including `dnd`, `usergroups`, `mpim`, and `files.comments`

# 0.15.0
* `hyper` is now an optional (but default) feature that can be disabled to use a different HTTP client
  * The only change this requires is in error handling. `Error::Internal` and `Error::Http` are gone, and `Error::HttpRequest` has been added. `Error` also no longer allows exhaustive checking to allow for future flexibility, though this limitation will be lifted before 1.0.
  * See [the pull request](https://github.com/slack-rs/slack-rs-api/pull/24) for more information
* `hyper` dependency also bumped to version `0.9.4`, if it's being used (thanks to @jgulotta)
* Fixed several changes in the schema from the Slack API that would cause erroneous deserialization errors (thanks to @kiyoto)
  * [Make BotMessage handle attachments](https://github.com/slack-rs/slack-rs-api/pull/16)
  * [Make url optional for File type](https://github.com/slack-rs/slack-rs-api/pull/17)
  * [Make text optional field in Attachment struct](https://github.com/slack-rs/slack-rs-api/pull/21)

# 0.14.0
* Removed `events` module. Now returned to [slack](https://github.com/slack-rs/slack-rs) crate.
* Add missing `pub` fields on some returned response objects
* Fix `api.test` method to not require a token to call

# 0.13.1
* Removed unused `Error::Utf8`
* Removed `Error::Url`, panicing instead of returning an `Err` on url parse failures due to no user input used

# 0.13.0
* Initial release – extracted from [slack](https://github.com/slack-rs/slack-rs) crate 