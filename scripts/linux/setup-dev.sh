#!/usr/bin/env bash
set -euo pipefail

persist=0
llvm_prefix=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --persist)
      persist=1
      shift
      ;;
    --llvm-prefix)
      llvm_prefix="${2:-}"
      shift 2
      ;;
    -h|--help)
      cat <<'EOF'
Usage: ./scripts/linux/setup-dev.sh [--persist] [--llvm-prefix <path>]

Options:
  --persist              Persist LLVM_SYS_150_PREFIX into shell rc file.
  --llvm-prefix <path>   Override auto-detection with a specific LLVM prefix.
EOF
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

test_llvm_prefix() {
  local prefix="$1"
  [[ -d "$prefix" ]] || return 1

  local llvm_config=""
  if [[ -x "$prefix/bin/llvm-config-15" ]]; then
    llvm_config="$prefix/bin/llvm-config-15"
  elif [[ -x "$prefix/bin/llvm-config" ]]; then
    llvm_config="$prefix/bin/llvm-config"
  else
    return 1
  fi

  local version
  version="$("$llvm_config" --version 2>/dev/null || true)"
  [[ "$version" == 15.* ]]
}

resolve_llvm_prefix() {
  local preferred="${1:-}"
  if [[ -n "$preferred" ]] && test_llvm_prefix "$preferred"; then
    echo "$preferred"
    return 0
  fi

  if [[ -n "${LLVM_SYS_150_PREFIX:-}" ]] && test_llvm_prefix "${LLVM_SYS_150_PREFIX}"; then
    echo "${LLVM_SYS_150_PREFIX}"
    return 0
  fi

  local cfg
  for cmd in llvm-config-15 llvm-config; do
    if command -v "$cmd" >/dev/null 2>&1; then
      cfg="$(command -v "$cmd")"
      local bin_dir
      bin_dir="$(cd "$(dirname "$cfg")" && pwd)"
      local prefix
      prefix="$(cd "$bin_dir/.." && pwd)"
      if test_llvm_prefix "$prefix"; then
        echo "$prefix"
        return 0
      fi
    fi
  done

  local candidates=(
    "/usr/lib/llvm-15"
    "/usr/local/lib/llvm-15"
    "/opt/llvm-15"
    "/usr/local/opt/llvm@15"
    "/opt/homebrew/opt/llvm@15"
  )

  local c
  for c in "${candidates[@]}"; do
    if test_llvm_prefix "$c"; then
      echo "$c"
      return 0
    fi
  done

  local roots=("$HOME" "/opt" "/usr/local")
  local root
  for root in "${roots[@]}"; do
    [[ -d "$root" ]] || continue
    while IFS= read -r -d '' found; do
      local p
      p="$(cd "$(dirname "$found")/.." && pwd)"
      if test_llvm_prefix "$p"; then
        echo "$p"
        return 0
      fi
    done < <(find "$root" -maxdepth 4 -type f \( -name "llvm-config-15" -o -name "llvm-config" \) -print0 2>/dev/null)
  done

  if command -v brew >/dev/null 2>&1; then
    local brew_prefix
    brew_prefix="$(brew --prefix llvm@15 2>/dev/null || true)"
    if [[ -n "$brew_prefix" ]] && test_llvm_prefix "$brew_prefix"; then
      echo "$brew_prefix"
      return 0
    fi
  fi

  return 1
}

prefix="$(resolve_llvm_prefix "$llvm_prefix" || true)"
if [[ -z "$prefix" ]]; then
  echo "No usable LLVM 15 installation found." >&2
  echo "Expected llvm-config --version to start with 15.x." >&2
  echo "Install LLVM 15 and rerun: ./scripts/linux/setup-dev.sh --llvm-prefix /usr/lib/llvm-15 --persist" >&2
  exit 1
fi

export LLVM_SYS_150_PREFIX="$prefix"
llvm_config="$prefix/bin/llvm-config"
if [[ -x "$prefix/bin/llvm-config-15" ]]; then
  llvm_config="$prefix/bin/llvm-config-15"
fi

version="$("$llvm_config" --version)"
targets="$("$llvm_config" --targets-built)"

echo "Detected LLVM prefix: $prefix"
echo "llvm-config version: $version"
echo "targets-built: $targets"

if [[ $persist -eq 1 ]]; then
  shell_name="$(basename "${SHELL:-bash}")"
  if [[ "$shell_name" == "zsh" ]]; then
    rc_file="$HOME/.zshrc"
  else
    rc_file="$HOME/.bashrc"
  fi

  touch "$rc_file"
  if grep -q '^export LLVM_SYS_150_PREFIX=' "$rc_file"; then
    sed -i.bak "s|^export LLVM_SYS_150_PREFIX=.*|export LLVM_SYS_150_PREFIX=\"$prefix\"|" "$rc_file"
  else
    printf '\nexport LLVM_SYS_150_PREFIX="%s"\n' "$prefix" >> "$rc_file"
  fi
  echo "Saved LLVM_SYS_150_PREFIX to $rc_file"
else
  echo "Set for current shell only. Re-run with --persist to save permanently."
fi

echo "Next step: cargo build -v"
