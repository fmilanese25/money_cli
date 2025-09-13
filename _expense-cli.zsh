#compdef expense-cli

autoload -U is-at-least

_expense-cli() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_expense-cli_commands" \
"*::: :->money_cli" \
&& ret=0
    case $state in
    (money_cli)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:expense-cli-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':amount:_default' \
':category:_default' \
'::message:_default' \
'::date:_default' \
'::latitude:_default' \
'::longitude:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_expense-cli__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:expense-cli-help-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_expense-cli_commands] )) ||
_expense-cli_commands() {
    local commands; commands=(
'add:' \
'list:' \
'completions:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'expense-cli commands' commands "$@"
}
(( $+functions[_expense-cli__add_commands] )) ||
_expense-cli__add_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli add commands' commands "$@"
}
(( $+functions[_expense-cli__completions_commands] )) ||
_expense-cli__completions_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli completions commands' commands "$@"
}
(( $+functions[_expense-cli__help_commands] )) ||
_expense-cli__help_commands() {
    local commands; commands=(
'add:' \
'list:' \
'completions:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'expense-cli help commands' commands "$@"
}
(( $+functions[_expense-cli__help__add_commands] )) ||
_expense-cli__help__add_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli help add commands' commands "$@"
}
(( $+functions[_expense-cli__help__completions_commands] )) ||
_expense-cli__help__completions_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli help completions commands' commands "$@"
}
(( $+functions[_expense-cli__help__help_commands] )) ||
_expense-cli__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli help help commands' commands "$@"
}
(( $+functions[_expense-cli__help__list_commands] )) ||
_expense-cli__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli help list commands' commands "$@"
}
(( $+functions[_expense-cli__list_commands] )) ||
_expense-cli__list_commands() {
    local commands; commands=()
    _describe -t commands 'expense-cli list commands' commands "$@"
}

if [ "$funcstack[1]" = "_expense-cli" ]; then
    _expense-cli "$@"
else
    compdef _expense-cli expense-cli
fi
