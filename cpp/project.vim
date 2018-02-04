" Vim settings for this project
set makeprg=./bb
noremap <leader>m :wa<CR>:make<CR>
set errorformat^=%-GERROR:\ %f:%l:%c:%m
