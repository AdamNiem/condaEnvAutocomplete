# condaEnvAutocomplete
Small conda env autocompleter

Paste this into your .bashrc (should be found in your home directory)
```
# User specific aliases and functions

alias sau='source activate universal'

function sai() {
        source activate "$1"
}

function sdi() {
        conda deactivate
}

# Fast tab-completion for `sai` using cached list of envs
# Completion function using your Rust binary
_sai_completions() {
  local cur="${COMP_WORDS[COMP_CWORD]}"
  local envs
  envs=$(~/conda_envs)  # update path to your compiled Rust binary

  COMPREPLY=( $(compgen -W "${envs}" -- "${cur}") )
}
# only for sai: list all matches on the first TAB,
# and do NOT fall back to filename completion
complete -F _sai_completions sai
```

And then you will need rustup to compile the rust executable. You should place the executable in your home directory or the same path as mentioned in the _sai_completions() function at "envs=$(~/conda_envs)  # update path to your compiled Rust binary"
```
rustc conda_envs.rs -o conda_envs
```
