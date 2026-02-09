" Wayfinder Debug Adapter for Neovim
" Plugin entry point

if exists("g:loaded_wayfinder_plugin")
  finish
endif
let g:loaded_wayfinder_plugin = 1

" Check if Neovim is running
if !has("nvim")
  echohl WarningMsg
  echom "Wayfinder requires Neovim"
  echohl None
  finish
endif

" Check if nvim-dap is available
try
  call luaeval("require('dap')")
catch
  echohl WarningMsg
  echom "Wayfinder requires nvim-dap plugin. Please install it first."
  echohl None
  finish
endtry

" Setup Wayfinder with default configuration
" Users can customize before sourcing with g:wayfinder_config
lua << EOF
  local wayfinder = require("wayfinder")

  -- Check if user has custom config in vim.g
  if vim.g.wayfinder_config == nil then
    vim.g.wayfinder_config = {}
  end

  -- Auto-setup
  if not vim.g.wayfinder_disable_auto_setup then
    wayfinder.setup(vim.g.wayfinder_config)
  end
EOF

" Default key mappings (can be disabled with g:wayfinder_use_keymaps = 0)
if get(g:, 'wayfinder_use_keymaps', 1)
  " These are actually set in commands.lua, but we document them here
  " Ctrl+F5: Debug current file
  " Ctrl+Shift+R: Select runtime
  " Ctrl+Shift+A: Attach to process

  " Standard DAP mappings (if available)
  nnoremap <silent> <F5> <Cmd>DapContinue<CR>
  nnoremap <silent> <F10> <Cmd>DapStepOver<CR>
  nnoremap <silent> <F11> <Cmd>DapStepInto<CR>
  nnoremap <silent> <S-F11> <Cmd>DapStepOut<CR>
  nnoremap <silent> <F9> <Cmd>DapToggleBreakpoint<CR>
  nnoremap <silent> <C-F9> <Cmd>DapSetConditionalBreakpoint<CR>
endif

" Command abbreviations
if get(g:, 'wayfinder_command_abbrev', 1)
  cabbrev <buffer> WDF WayfinderDebugFile
  cabbrev <buffer> WSR WayfinderSelectRuntime
  cabbrev <buffer> WAP WayfinderAttachProcess
  cabbrev <buffer> WRT WayfinderRuntimes
endif

" Print initialization message
if !get(g:, 'wayfinder_quiet', 0)
  echom "Wayfinder Debug Adapter loaded. Use :WayfinderDebugFile to start debugging."
endif
