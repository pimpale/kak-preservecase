# kak-preservecase

### Command line utility

This is a simple command-line tool to replace text in a way that preserves case.

```
Usage: kak-preservecase QUERY REPLACEMENT
Reads from stdin, where it replaces QUERY with REPLACEMENT in a case preserving manner.
Example: kak-preservecase alpha Beta

For matching purposes, QUERY is an case insensitive string.
It does not accept regex, as this would cause ambiguity.
Every instance of QUERY will be replaced with REPLACEMENT.
For every match at index i, uppercaseness is: (!QUERY[i] && MATCH[i]) || REPLACEMENT[i]
```

### As a kakoune plugin
Using [plug.kak]( https://github.com/robertmeta/plug.kak ), you can include this plugin the following way:
```
plug "https://github.com/pimpale/kak-preservecase" ensure do %{
    cargo install --locked --force --path .
} config %{
    # I chose s because it was ergonomic, but you can bind it to something else
    map global user s ':preserve-case ' -docstring 'case sensitive replace'
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

### Usage example

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

### Caveats and Limitations

We can't support regex because it would introduce ambiguity. 
Regex would prevent us from specifying the the case of the query. 
This would cause problems when we have arguments of different lengths.

Example:
```
userMail
UserMail
```
You enter:
```
preserve-case user otherUser
```
Text after:
```
otheRUserMail
OtheRUserMail
```
The capitalization is applied like a mask, without regard for word boundaries.
Since `userMail` has a capital at index 4, so does `otheRUserMail`.
If you have this issue, it means you need to put a capital in your query string to mask it in the source text.

### Fixed usage example

To avoid the above issue, you have to specify any capitals you want to be ignored in the source text.

Text before:
```
userMail
UserMail
```
You enter:
```
preserve-case user otherUser
```
Text after:
```
otherUserMail
OtherUserMail
```

