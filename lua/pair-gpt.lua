local api = vim.api
local o = vim.o
local fn = vim.fn

local bin = "pair-gpt"

local function clean_prompt(prompt)
  local stripable = "/\\%*-"
  local ret = prompt

  ret = prompt:gsub("^[" .. stripable .. "]", "")
  ret = ret:gsub("[" .. stripable .. "]$", "")
  ret = ret:gsub("\"", "\\\"")

  return ret
end

local function pair_cmd(subcmd, lang, prompt)
  local cmd = bin .. " --lang " .. lang .. " " .. subcmd .. " \" " .. prompt .. "\""

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

local function get_visual_selection(buf)
  local s_start = vim.fn.getpos("'<")
  local s_end = vim.fn.getpos("'>")
  local lines = vim.api.nvim_buf_get_lines(buf, s_start[2] - 1, s_end[2], false)

  -- TODO currently grabbing entire lines, not exact visual selection
  -- local n_lines = math.abs(s_end[2] - s_start[2]) + 1
  -- lines[1] = string.sub(lines[1], s_start[3], -1)
  -- if n_lines == 1 then
  --   lines[n_lines] = string.sub(lines[n_lines], 1, s_end[3] - s_start[3] + 1)
  -- else
  --   lines[n_lines] = string.sub(lines[n_lines], 1, s_end[3])
  -- end

  return table.concat(lines, '\\n')
end

local function write()
  local line = api.nvim_get_current_line()
  local lang = o.syntax
  local buf = api.nvim_get_current_buf()

  -- clean prompt. remove comment characters
  local prompt = clean_prompt(line)

  -- query OpenAI. this is blocking
  local output = pair_cmd("write", lang, prompt)

  api.nvim_buf_set_lines(buf, 0, 0, false, output)
end

local function refactor()
  local s_start = fn.getpos("'<")
  local s_end = fn.getpos("'>")
  local lang = o.syntax
  local buf = api.nvim_get_current_buf()

  local input = clean_prompt(get_visual_selection(buf))
  local output = pair_cmd("refactor", lang, input)

  -- writ_ output right below the prompt line
  -- TODO currently replacing whole lines, not exact visual selection
  -- api.nvim_buf_set_text(buf, s_start[2] - 1, s_start[3] - 1, s_end[2] - 1, s_end[3], output)
  api.nvim_buf_set_lines(buf, s_start[2] - 1, s_end[2], false, output)

end

--
-- function _G.reload_current_file() vim.cmd(":luafile %") end

return {
  write = write,
  refactor = refactor
}
