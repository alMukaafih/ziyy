function ziyy_escape {
    local first_digit=$1
    local second_digit=$2
    ziyy_return="\x1b[${first_digit}${second_digit}m"
}
function ziyy_color_value {
    local color=$2
    local rgb
    case $color in
        black)
            ziyy_escape $1 0
            ziyy_color=$ziyy_return
        ;;
        red)
            ziyy_escape $1 1
            ziyy_color=$ziyy_return
        ;;
        green)
            ziyy_escape $1 2
            ziyy_color=$ziyy_return
        ;;
        yellow)
            ziyy_escape $1 3
            ziyy_color=$ziyy_return
        ;;
        blue)
            ziyy_escape $1 4
            ziyy_color=$ziyy_return
        ;;
        magenta)
            ziyy_escape $1 5
            ziyy_color=$ziyy_return
        ;;
        cyan)
            ziyy_escape $1 6
            ziyy_color=$ziyy_return
        ;;
        white)
            ziyy_escape $1 7
            ziyy_color=$ziyy_return
        ;;
        rgb\(*\))
            rgb=${color#rgb(}
            rgb=${rgb%)}
            rgb=${rgb//,/;}
            rgb=${rgb// /}
            ziyy_escape $1 "8;2;$rgb"
            ziyy_color=$ziyy_return
        ;;
    esac
}

function ziyy_substitute {
    text=$1
    tag=$2
    tag=${tag/\[/\\[}
    tag=${tag/\]/\\]}
    ziyy_return=${text/$tag/$ziyy_color}
}

function ziyy_parse {
    local text=$1
    ziyy_tags=(s)
    ziyy_result=""
    ziyy_open=0
    ziyy_esc=0
    local len=${#text}
    local tag=""
    local i=0
    while ((i < len)); do
        x=${text:i:1}
        if [[ $x == "\\" ]] && [[ $ziyy_esc -eq 0 ]]; then
            ziyy_esc=1
        elif [[ $ziyy_esc -eq 1 ]]; then
            ziyy_result+=$x
        elif [[ $x == "[" ]]; then
            ziyy_open=1
            ziyy_result+=$x
            tag+=$x
        elif [[ $x == "]" ]]; then
            ziyy_open=0
            ziyy_result+=$x
            tag+=$x
            ziyy_tags+=($tag)
            tag=""
        elif [[ $ziyy_open -eq 1 ]] && [[ $x != " " ]]; then
            ziyy_result+=$x
            tag+=$x
        elif [[ $ziyy_open -eq 1 ]] && [[ $x == " " ]]; then
            $x
        else
            ziyy_result+=$x
        fi
            ((i++))
    done
}

function style {
    text="$1"
    RESET="\x1b[0m"
    ziyy_parse "$text"
    text=$ziyy_result
    for tag in "${ziyy_tags[@]:1}"; do
        len=${#tag}
        to=$((len - 4))
        case $tag in
        \[c:*)
            value=${tag:3:$to}
            ziyy_color_value 3 $value
            ziyy_substitute "$text" $tag
            text=$ziyy_return
        ;;
        \[x:*)
            value=${tag:3:$to}
            ziyy_color_value 4 $value
            ziyy_substitute "$text" $tag
            text=$ziyy_return
        ;;
        esac
        # Bold
        if [[ $tag == "[b]" ]]; then
            text=${text//\[b\]/\\x1b[1m}
        # Remove Bold
        elif [[ $tag == "[/b]" ]]; then
            text=${text//\[\/b\]/\\x1b[22m}

        # Italics
        elif [[ $tag == "[i]" ]]; then
            text=${text//\[i\]/\\x1b[3m}
        # Remove italics
        elif [[ $tag == "[/i]" ]]; then
            text=${text//\[\/i\]/\\x1b[23m}

        # Remove colors
        elif [[ $tag == "[/c]" ]]; then
            text=${text//\[\/c\]/\\x1b[39m}
        elif [[ $tag == "[/x]" ]]; then
            text=${text//\[\/x\]/\\x1b[39m}

        # Underline
        elif [[ $tag == "[u]" ]]; then
            text=${text//\[u\]/\\x1b[4m}
        elif [[ $tag == "[/u]" ]]; then
            text=${text//\[\/u\]/\\x1b[24m}

        # Strike through
        elif [[ $tag == "[s]" ]]; then
            text=${text//\[s\]/\\x1b[9m}
        elif [[ $tag == "[/s]" ]]; then
            text=${text//\[\/s\]/\\x1b[29m}
        
        elif [[ $tag == "[/0]" ]]; then
            text=${text//\[\/0\]/$RESET}
        fi
    done
    return="${text}${RESET}"
}

if [[ ${0##*/} == "libziyy.sh" ]]; then
    style "[b][c: red]Hello World!"
    echo -e $return
fi