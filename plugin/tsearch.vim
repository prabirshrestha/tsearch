if exists('g:tsearch_loaded')
    finish
endif

let g:tsearch_loaded = 1

command! -nargs=? TSearch call tsearch#search(<q-args>)
command! TSearchCancel call tsearch#cancel()
