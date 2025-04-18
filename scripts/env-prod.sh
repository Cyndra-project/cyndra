export cyndra_API="https://api.cyndra.dev"
unset cyndra_API_KEY
export PS1="(cyndra: Production) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"
