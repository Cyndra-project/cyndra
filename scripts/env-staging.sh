export cyndra_API="https://api.staging.cyndra.dev"
unset cyndra_API_KEY
export PS1="(cyndra: Staging) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"
