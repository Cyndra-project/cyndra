# Use this to make cargo-cyndra target the unstable (staging) env.
#
# Usage:
#     source scripts/unstable.sh

export cyndra_API="https://api.unstable.cyndra.rs"
unset cyndra_API_KEY
export PS1="(cyndra: unstable) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"
