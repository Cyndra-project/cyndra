# Use this to make cargo-cyndra target the production env.
# Useful when running cargo-cyndra in debug mode, since that targets the local stack by default.
#
# Usage:
#     source scripts/production.sh

export cyndra_API="https://api.cyndra.rs"
unset cyndra_API_KEY
unset cyndra_BETA
export PS1="(cyndra: production) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"
