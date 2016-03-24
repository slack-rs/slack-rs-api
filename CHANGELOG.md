# 0.14.0
* Removed `events` module. Now returned to [slack](https://github.com/slack-rs/slack-rs) crate.
* Add missing `pub` fields on some returned response objects
* Fix `api.test` method to not require a token to call

# 0.13.1
* Removed unused `Error::Utf8`
* Removed `Error::Url`, panicing instead of returning an `Err` on url parse failures due to no user input used

# 0.13.0
* Initial release – extracted from [slack](https://github.com/slack-rs/slack-rs) crate 