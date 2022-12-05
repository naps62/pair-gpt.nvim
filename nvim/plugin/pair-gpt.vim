if exists('g:loaded_pair_gpt') | finish | endif

let s:save_cpo = &cpo
set cpo&vim

command! Write lua require'pair-gpt'.write()
command! Refactor lua require'pair-gpt'.refactor()

let &cpo = s:save_cpo
unlet s:save_cpo

let g:loaded_pair_gpt = 1
