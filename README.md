# kak-preservecase

### Command line utility

This is a simple command-line tool to replace text in a way that preserves case.

Usage: `kak-preservecase REGEX REPLACEMENT`
Reads from stdin, where it replaces REGEX with REPLACEMENT in a case preserving manner.
Example: `kak-preservecase alpha Beta`

REGEX is an case insensitive extended regular expression.
Every instance of REGEX will be replaced with REPLACEMENT.
The replacement character is uppercase if REPLACEMENT is uppercase or the matched text is uppercase.

### As a kakoune plugin
Using [plug.kak]( https://github.com/robertmeta/plug.kak ), you can include this plugin the following way:
```
plug "https://github.com/pimpale/kak-preservecase" ensure do %{
    cargo install --locked --force --path .
} noload config %{
    # I chose s because it was ergonomic, but you can bind it to something else
    map global user s ':preserve-case' -docstring 'case sensitive replace'
}
```

### vim-abolish equivalent
I find that this plugin is a good kakoune equivalent for [vim-abolish]( https://github.com/tpope/vim-abolish )
when used alongside [kak-subvert]( https://github.com/dmerejkowsky/kak-subvert ).
Install it like so:
```
plug "https://github.com/dmerejkowsky/kak-subvert" ensure do %{
    cargo install --locked --force --path .
} noload config %{
    declare-user-mode subvert
    map global user n ':enter-user-mode subvert<ret>' -docstring 'enter subvert user mode'
    map global subvert c '<esc>|kak-subvert camel<ret>' -docstring 'convert to camel case'
    map global subvert k '<esc>|kak-subvert kebab<ret>' -docstring 'convert to kebab case'
    map global subvert p '<esc>|kak-subvert pascal<ret>' -docstring 'convert to pascal case'
    map global subvert . '<esc>|kak-subvert sentence<ret>' -docstring 'convert to sentence case'
    map global subvert s '<esc>|kak-subvert snake<ret>' -docstring 'convert to snake case'
    map global subvert S '<esc>|kak-subvert screaming<ret>' -docstring 'convert to screaming case'
    map global subvert t '<esc>|kak-subvert train<ret>' -docstring 'convert to train case'
    map global subvert T '<esc>|kak-subvert title<ret>' -docstring 'convert to title case'
    map global subvert u '<esc>|kak-subvert ugly<ret>' -docstring 'convert to ugly case'
}
```

### More usage examples

##### Example 1 (simple replace)
Text before:
```
camelCase
CAMELCASE
CamelCase
```
You enter:
```
preserve-case camel abcde
```
Text afer:
```
abcdeCase
ABCDECASE
AbcdeCase
```

### Caveats and limitations
This plugin doesn't work well when we have arguments of different lengths.

Example:
```
userMail
```
You enter:
```
preserve-case usermail otherUserMail
```
Text after:
```
otheRUserMail
```
The capitalization is applied like a mask, without regard for word boundaries.
Since `userMail` has a capital at index 4, so does `otheRUserMail`.
A future update may find a fix to this issue.
