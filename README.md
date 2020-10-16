# git-to-the-action

Simple tool that outputs the common prefix directory for any changed files in
current git repo.

I use this in my custom `cd` command to automatically jump to the directory
with changes, when entering a git repo:

```shell
# .zshrc

OLD_GIT_REPO_ROOT="$(git_repo_root)"

cd() {
  builtin cd "$@"

  local repo_root="$(git_repo_root)"
  # clear cached git repo value if not in a repo
  if [[ "$repo_root" == "" && "$OLD_GIT_REPO_ROOT" != "" ]]; then
    OLD_GIT_REPO_ROOT=''
  fi
  if [[ "$repo_root" != "" && "$repo_root" != "$OLD_GIT_REPO_ROOT" ]]; then
    # entering new git repo
    local d="$(git-to-the-action)"
    # only change to the action dir if cd'ing into repo root (not a subdir)
    if [[ "$PWD" == "$repo_root" && "$d" != "." && "$d" != "" ]]; then
      # go to base dir of changes
      echo "(heading to the action: $d)"
      builtin cd "$d"
    fi
    git status --short
    OLD_GIT_REPO_ROOT="$repo_root"
  else
    exa -F --group-directories-first
  fi
}

# get git repo root of current directory. empty string indicates not in a git repo
function git_repo_root {
    git rev-parse --show-toplevel 2>/dev/null
}
```

TODO: handle renamed files better. currently uses path of old file name.
