#!/bin/bash
#################################################
# Name: libshstyle.sh #
# A library for styling text using escape sequence.
#
# Usage: style <option> <text> #
# Author: Tabriik # Date: 2023/08/26 
#################################################

# Changes the foreground color #
# param: color instruction #
# returns: escape sequence representing color.
__fg_color() {
	case $1 in
		# Basic colors
		black) shift 1; fgcolor="\x1b[30m";;
		red) shift 1; fgcolor="\x1b[31m";;
		green) shift 1; fgcolor="\x1b[32m";;
		yellow) shift 1; fgcolor="\x1b[33m";;
		blue) shift 1; fgcolor="\x1b[34m";;
		magenta) shift 1; fgcolor="\x1b[35m";;
		cyan) shift 1; fgcolor="\x1b[36m";;
		white) shift 1; fgcolor="\x1b[37m";;
		
		# RGB Colors
		rgb\(*\))
			frgb="$1"   # asign $1 to $frgb
			frgb=$(builtin echo "$frgb" | sed 's/rgb(//' | sed 's/)//' | sed 's/\,/\;/g')
			fgcolor="\x1b[38;2;${frgb}m"
			shift 1
			;;
		*) style -b -c red Error: unknown fg color -q yellow $1 . >&2; return 1;;
	esac
	string="$string$fgcolor"
	
}

# Changes the background color #
# param: color instruction #
# returns: escape sequence representing color.
__bg_color() {
	case $1 in
		# Basic colors
		black) shift 1; bgcolor="\x1b[40m";;
		red) shift 1; bgcolor="\x1b[41m";;
		green) shift 1; bgcolor="\x1b[42m";;
		yellow) shift 1; bgcolor="\x1b[43m";;
		blue) shift 1; bgcolor="\x1b[44m";;
		magenta) shift 1; bgcolor="\x1b[45m";;
		cyan) shift 1; bgcolor="\x1b[46m";;
		white) shift 1; bgcolor="\x1b[47m";;
		
		# RGB Colors
		rgb\(*\)) 
			brgb="$1"
			brgb=$(builtin echo "$brgb" | sed 's/rgb(//' | sed 's/)//' | sed 's/\,/\;/g')
			bgcolor="\x1b[48;2;${brgb}m"
			shift 1
			;;
		*) style -b -c red Error: unknown bg color -q yellow $1 . >&2; return 1;;
	esac
	string="$string$bgcolor"
	
}

# Select the letter to use for quoting #
# param: Letter to use for quoting #
# returns: Letter in right position.
__quotes() {
	case $1 in
	\[) q1=[
		q2=];;
	\() q1=\(
		q2=\);;
	{) q1={
		q2=};;
	“) q1=“
		q2=”;;
	‘) q1=‘
		q2=’;;
	«) q1=«
		q2=»;;
	﴾) q1=﴾
		q2=﴿;;
	*) q1=$1
		q2=$1;;
	esac
}

# Style text #
# param: Styling Intruction and text #
# returns: Styled Text.
style() {
	# VARIABLES
	save1=$reset    # save $reset if defined
	save2=$string   # save $string if defined
	reset="\x1b[0m" # reset Colors
	local bgc       # background color
	local fgc       # foreground color
	local quotes    # quoted text
	local n         # newline character
	string=""       # reset string
	local s=1     # space
	while [ -n "$1" ]; do
		case $1 in
			#foreground colors
			--fg | -c)
				fgc="$2"
				__fg_color "$fgc" || return 1
				shift 2
				;;
			--fg=* | -c=*)
				fgc="${1#*=}"
				__fg_color "$fgc" || return 1
				shift 1
				;;
			
			# background colors
			--bg | -x)
				bgc="$2"
				__bg_color "$bgc" || return 1
				shift 2
				;;
			--bg=* | -x=*)
				bgc="${1#*=}"
				__bg_color "$bgc" || return 1
				;;
			
			# Bold
			-b) string="$string\x1b[1m"; shift 1;;
			# Remove Bold
			-rmb) string="$string\x1b[21m"; shift 1;;
			
			# Italics
			-i) string="$string\x1b[3m"; shift 1;;
			# Remove italics
			-rmi) string="$string\x1b[23m"; shift 1;;
			
			# Remove colors
			-rmc) string="$string\x1b[39m"; shift 1;;
			-rmx) string="$string\x1b[49m"; shift 1;;
			
			# Underline
			-u) string="$string\x1b[4m"; shift 1;;
			-rmu) string="$string\x1b[24m"; shift 1;;
			
			# Strike through
			-s) string="$string\x1b[9m"; shift 1;;
			-rms) string="$string\x1b[29m"; shift 1;;
			
			# Quote
			-q)
				save=$fgcolor
				__fg_color $2
				chose=$fgcolor
				quotes="$(builtin echo -n $save \' $chose $3 $save \')"
				quotes=$(builtin echo "$quotes" | sed 's/ //g')
				string="$string$s$quotes"
				shift 3
				;;
			-Q)
				save=$fgcolor
				__fg_color $2
				chose=$fgcolor
				quotes="$(builtin echo -n $save \" $chose $3 $save \")"
				quotes=$(builtin echo "$quotes" | sed 's/ //g')
				string="$string$s$quotes"
				shift 3
				;;
			-q=*)
				quo="${1#*=}" # Extract text after =
				__quotes "$quo"
				save=$fgcolor
				__fg_color $2
				chose=$fgcolor
				quotes="$(builtin echo -n $save $q1 $chose $3 $save $q2)"
				quotes=$(builtin echo "$quotes" | sed 's/ //g')
				string="$string$s$quotes"
				shift 2
				;;
			
			# no newline
			-n) n="n"; shift 1;;
			
			# Unknown Options
			-*) shift 1;;
			
			# Strings
			*) 
			if ((s == 1)); then
				s=""
			else
				s=" "
			fi
			string="$string$s$1"
			shift 1
			;;
		esac
		done
	string=$(builtin echo "$string" | sed 's/ *$//' | sed 's/ \./\./')
	builtin echo -${n}e ${string}${reset}
	
	reset=$save1   # restore reset
	string=$save2  # restore string
}
#style -b -s -c red Hello -rms -c blue World! -c green My -c yellow Name -c magenta is -c cyan Mubarak -rmc Umoru -i -Q blue Yes
