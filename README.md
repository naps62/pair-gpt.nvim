# pAIr

[packer]: https://github.com/wbthomason/packer.nvim
[openai-api]: https://openai.com/api/
[openai-pricing]: https://openai.com/api/pricing/
[openai-apikeys]: https://beta.openai.com/account/api-keys

A GPT-assisted pair programming partner.

[![asciicast](https://asciinema.org/a/gIXsFqG3ZDVhVFRkpIkBipz9I.svg)](https://asciinema.org/a/gIXsFqG3ZDVhVFRkpIkBipz9I)


# Prerequisites

You need an account for [OpenAI's API][openai-api]. Check out the [pricing page][openai-pricing] to see how far you can go with the free plan (spoiler alert: should be more than enough to take this plugin out for a testdrive).


# Install

1. Create an API key through [OpenAI's dashboard][openai-apikeys], and set it to an `OPENAI_API_KEY` environment variable.
2. Install the `pair-gpt` binary: `cargo install --git https://github.com/naps62/pair-gpt`
3. Setup the vim plugin (see below)

## Vim plugin setup

With [packer][packer]:

```vimscript
use { 'naps62/pair-gpt', rtp = "nvim" }
```

# Usage

Open a file with your editor. ensure filetype detection is live (this plugin uses that to pass down to the bot).

There's currently a single command (with more to come):

* `PairGPTWrite` - takes the current line, and sends that as a query to OpenAI. Comment characters such as `//` at the beginning or end of the line are automatically removed. The output from the bot will be written right below this line
