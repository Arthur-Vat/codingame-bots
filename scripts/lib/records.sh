# shellcheck shell=bash
# Shared by scripts/check-determinism.sh and scripts/check-minified.sh, which
# compare the arena's records of games played twice. Needs jq. Source it
# from the repository root.

# What a record must keep from run to run: the lines of every answer, by
# turn, the winner and how the game ended (times, which vary, are ignored).
# A record without these fields is an error, never "identical".
readonly canon='
  if .format != 1 then error("unexpected record format")
  elif (has("winner") | not) then error("record without winner")
  else
    {turns: [.turns[] | [.[] | (.lines // error("record without lines"))]],
     winner,
     end: (.end.kind // error("record without end kind"))}
  end'

# Where records $1 and $2 first differ: "turn N", "the winner" or "the end".
first_difference() {
  jq -n -r --slurpfile a <(jq -c "$canon" "$1") --slurpfile b <(jq -c "$canon" "$2") '
    ($a[0].turns) as $x | ($b[0].turns) as $y
    | ([range(0; [($x | length), ($y | length)] | max)]
       | map(select($x[.] != $y[.])) | first) as $turn
    | if $turn != null then "turn \($turn + 1)"
      elif $a[0].winner != $b[0].winner then "the winner"
      else "the end" end'
}

# Compares the records of two runs: directories $1 and $2 of *.json files,
# game by game, by file name. Prints nothing and returns 0 when every game
# has the same canonical record in both and neither has other games; else
# prints the problem (for example "run 2 differs from run 1 in g.json at
# turn 3") and returns 1. $3 and $4 name the runs in that text.
compare_runs() {
  local first_dir="$1" second_dir="$2" first_name="$3" second_name="$4"
  local first second base canon_first canon_second games=0
  for first in "$first_dir"/*.json; do
    games=$((games + 1))
    base="$(basename "$first")"
    second="$second_dir/$base"
    if [[ ! -f $second ]]; then
      echo "$second_name has no $base"
      return 1
    fi
    # Callers run this where `set -e` is off: a record jq rejects is a
    # problem, never "identical".
    canon_first="$(jq -c "$canon" "$first" 2>&1)" || {
      echo "cannot read $first: $canon_first"
      return 1
    }
    canon_second="$(jq -c "$canon" "$second" 2>&1)" || {
      echo "cannot read $second: $canon_second"
      return 1
    }
    if [[ $canon_first != "$canon_second" ]]; then
      echo "$second_name differs from $first_name in $base at $(first_difference "$first" "$second")"
      return 1
    fi
  done
  if [[ $(find "$second_dir" -name '*.json' | wc -l) -ne $games ]]; then
    echo "$second_name has other games than $first_name"
    return 1
  fi
}
