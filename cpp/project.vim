" Vim settings for this project
set makeprg=./bb
noremap <leader>m :wa<CR>:make<CR>
set errorformat^=%-GERROR:\ %f:%l:%c:%m

" Your filetype specific options
" let g:clang_format#filetype_style_options = {
"             \   'cpp' : {"Standard" : "C++11", "IndentWidth" : "4"},
"             \ }

" let g:clang_format#code_style = "google"
"

