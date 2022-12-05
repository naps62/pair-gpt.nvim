local api = vim.api
local o = vim.o

local bin = "~/projects/pair/target/debug/pair-gpt"

local function clean_prompt(prompt)
  local stripable = "/\\%*-"
  local ret = prompt

  ret = prompt:gsub("^[" .. stripable .. "]", "")
  ret = ret:gsub("[" .. stripable .. "]$", "")

  return ret
end

local function pair_cmd(subcmd, lang, prompt)
  local cmd = bin .. " --lang " .. lang .. " --prompt \"" .. prompt .. "\" " .. subcmd

  -- run cmd
  local handle = assert(io.popen(cmd, 'r'))
  local output = assert(handle:read('*a'))
  handle:close()

  -- split by lines
  lines = {}
  for s in output:gmatch("[^\r\n]+") do
    table.insert(lines, s)
  end

  return lines
end

function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k, v in pairs(o) do
      if type(k) ~= 'number' then k = '"' .. k .. '"' end
      s = s .. '[' .. k .. '] = ' .. dump(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

local function write()
  local win = api.nvim_get_current_win()

  local line = api.nvim_get_current_line()
  local lang = o.syntax
  local buf = api.nvim_get_current_buf()
  local linenr = api.nvim_win_get_cursor(win)[1]
  local prompt = clean_prompt(line)

  local output = pair_cmd("write", lang, prompt)

  api.nvim_buf_set_lines(buf, linenr, linenr, false, output)
end

return {
  write = write
}
