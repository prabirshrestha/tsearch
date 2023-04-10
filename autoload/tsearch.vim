function! tsearch#cancel() abort
   if exists('s:job_id')
    call job_stop(s:job_id)
   endif
endfunction

function! tsearch#search(q) abort
    call tsearch#cancel()

    let l:dir=expand(getcwd())

    let l:cmd = [
        \ 'tsearch',
        \ '-q',
        \ '-p',
        \ l:dir,
        \ '--',
        \ a:q]
    
    echo 'Starting TSearch'

    let l:ctx = { 'error': '', 'qfid': 0, 'cache': {} }

    let s:job_id = job_start(l:cmd, {
        \ 'out_cb': function('s:out_cb', [l:ctx]),
        \ 'err_cb': function('s:err_cb', [l:ctx]),
        \ 'exit_cb': function('s:exit_cb', [l:ctx]),
        \ })
    
    call setqflist([], ' ', {'title' : 'TSearch Results'})
    let l:ctx.qfid = getqflist({'id' : 0}).id
    botright copen
endfunction

function! s:out_cb(ctx, id, data) abort
    let l:parts = split(a:data, ':')
    if has('win32')
        let l:path = l:parts[0] . ':' . l:parts[1]
        let l:line = l:parts[2]
        let l:col = l:parts[3]
    else
        let l:path = l:parts[0]
        let l:line = l:parts[1]
        let l:col = l:parts[2]
    endif

    if getqflist({'id' : a:ctx.qfid}).id == a:ctx.qfid
        if has_key(a:ctx['cache'], l:path) 
            let l:text = a:ctx['cache'][l:path][l:line - 1]
        else
            let l:contents = getbufline(l:path, '$')
            if empty(l:contents)
                let l:contents = readfile(l:path)
                let a:ctx['cache'][l:path] = l:contents
            endif
            let l:text = get(l:contents, l:line - 1, '')
        endif
        let l:item = {
            \ 'filename': l:path,
            \ 'col': l:col,
            \ 'lnum': l:line,
            \ 'text': l:text
            \ }
        call setqflist([], 'a', {'id' : a:ctx.qfid, 'items' : [l:item]})
    else
        call tsearch#cancel(a:id)
    endif
endfunction

function! s:err_cb(ctx, id, data) abort
    echom a:data
    let a:ctx['error'] .= a:data
endfunction

function! s:exit_cb(ctx, id, data) abort
    unlet s:job_id
    if a:data == 0
        echo 'TSearch complete'
    else
        echo 'TSearch failed: ' . a:ctx['error']
    endif
endfunction
