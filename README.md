# pair GPT - Neovim plugin

[packer]: https://github.com/wbthomason/packer.nvim
[openai-api]: https://openai.com/api/
[openai-pricing]: https://openai.com/api/pricing/
[openai-apikeys]: https://beta.openai.com/account/api-keys

A GPT-assisted pair programming partner for Neovim.

## Features

**Write code from text prompts**

[![asciicast](https://asciinema.org/a/gIXsFqG3ZDVhVFRkpIkBipz9I.svg)](https://asciinema.org/a/gIXsFqG3ZDVhVFRkpIkBipz9I)

**Refactor existing code**

[![asciicast](https://asciinema.org/a/0IrC8uV5aHt4kXnfJK273Hcez.svg)](https://asciinema.org/a/0IrC8uV5aHt4kXnfJK273Hcez)

## Install

You need an account for [OpenAI's API][openai-api]. Check out the [pricing page][openai-pricing] to see how far you can go with the free plan (spoiler alert: should be more than enough to take this plugin out for a testdrive).

Export the API key as an environment variable. Add this to your `.bashrc` / `.zshrc` / etc:

```
export OPENAI_API_KEY=xxxxxxxxxxxxxxx
```

Install the binary:

```bash
cargo install --git https://github.com/naps62/pair-gpt.nvim
```

Install the plugin: with [packer][packer]

```vim
" with vim-plug:
Plug 'naps62/pair-gpt.nvim'

" with packer:
use { 'naps62/pair-gpt.nvim',
  config = function()
    require('pair-gpt').setup()
  end
}
```

## Setup (optional)

Setup should be run in a lua file or in a lua heredoc [:help lua-heredoc](https://neovim.io/doc/user/lua.html) if using in a vim file.

```lua
-- examples for your init.lua

-- empty setup using defaults
-- can be done directly via packer's config option, as seen above
require("pair-gpt.nvim").setup()

-- OR setup with some options
require("pair-gpt.nvim").setup({
  bin = "pair-gpt",
  model = "text-davinci-001"
})
```

### Vim plugin setup

With [packer][packer]:


## Usage

Open a file with your editor. ensure filetype detection is live (this plugin uses that to pass down to the bot).

There's currently a single command (with more to come):

* `PairGPTWrite` - takes the current line, and sends that as a query to OpenAI. Comment characters such as `//` at the beginning or end of the line are automatically removed. The output from the bot will be written right below this line
