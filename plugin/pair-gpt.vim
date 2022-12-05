if exists('g:loaded_pair_gpt') | finish | endif

let s:save_cpo = &cpo
set cpo&vim

command! PairGPTWrite lua require'pair-gpt'.write()
command! PairGPTRefactor lua require'pair-gpt'.refactor()

let &cpo = s:save_cpo
unlet s:save_cpo

let g:loaded_pair_gpt = 1
