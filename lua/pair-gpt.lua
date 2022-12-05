local api = vim.api
local o = vim.o

local bin = "pair-gpt"

local function clean_prompt(prompt)
  local stripable = "/\\%*-"
  local ret = prompt

  ret = prompt:gsub("^[" .. stripable .. "]", "")
  ret = ret:gsub("[" .. stripable .. "]$", "")

  return ret
end

local function pair_cmd(subcmd, lang, prompt)
  local cmd = bin .. " --lang " .. lang .. " " .. subcmd .. " \" " .. prompt .. "\""
  print(cmd)

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

-- M.send_selection = function()
--     local line1 = vim.fn.getpos("'<")[2]
--     local line2 = vim.fn.getpos("'>")[2]
--
--     local lines = ""
--     for _, v in pairs(vim.fn.getline(line1, line2)) do
--         lines = lines .. v .. "\n"
--     end
--
--     M.send(lines)
-- end
--
-- function _G.reload_current_file() vim.cmd(":luafile %") end

return {
  write = write
}
