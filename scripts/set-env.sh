# Usage: source scripts/set-env.sh [env]

if [ -z "$1" ]; then
    echo "provide an env name";
else
    export cyndra_API_ENV="$1"
    unset cyndra_API
    unset cyndra_API_KEY
    export PS1="(cyndra: $cyndra_API_ENV) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"
fi
