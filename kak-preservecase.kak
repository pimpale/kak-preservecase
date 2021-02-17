define-command preserve-case -docstring "case preserving replace" -params 2 %{
  execute-keys "<esc>|kak-preservecase %arg{1} %arg{2}<ret>" 
}
