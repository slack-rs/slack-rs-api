# Contributing

To update this library to the latest Slack API definitions, create a branch in your clone of [`slack-api-schemas`](https://github.com/slack-rs/slack-api-schemas) and then run the api scraper with:

```bash
$ node index.js --schema_dir ..
```

Watch for any interesting error messages and manually override the output in the scraper if necessary. Once the updates have been verified, commit and push that branch, then create a pull request against master.

After that pull request has been merged, this library can be updated. Create a branch in this your clone of this repository, and update the schemas submodule:

```bash
$ git submodule update --init
$ cd codegen/slack-api-schemas
$ git checkout master
$ git pull
```

Once the schemas are updated, simply run the code generator:

```bash
$ cd codegen
$ cargo run
```

That will create the Rust modules and types for the schemas. After that, just push and PR!
