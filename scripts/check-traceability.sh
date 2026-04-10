#!/usr/bin/env bash
# Vérifie que les références dans la matrice de traçabilité pointent
# vers des fichiers et des tests qui existent réellement.
set -euo pipefail

MATRIX="docs/references/traceability-matrix.md"
errors=0

# --- Vérification des fichiers source ----------------------------------------

echo "=== Checking source file references ==="
checked=0

# Extraire les chemins de fichiers (pattern: `crates/…/file.rs`)
while IFS= read -r f; do
    checked=$((checked + 1))
    if [ ! -f "$f" ]; then
        echo "  MISSING file: $f"
        errors=$((errors + 1))
    fi
done < <(grep -oP '`(crates/[^`]+\.rs)`' "$MATRIX" | tr -d '`' | sort -u)

echo "  $checked file reference(s) checked."

# --- Vérification des noms de tests ------------------------------------------

echo ""
echo "=== Checking test function references ==="
checked_tests=0

# Extraire les noms de tests depuis la colonne Test(s) des tableaux.
# Les tableaux Markdown ont le format :
#   | BT-1 | Description | Source | `test_name` | Status |
# awk -F'|' donne : $1="" $2=BT $3=Desc $4=Source $5=Tests $6=Status $7=""
# On filtre les headers (BT/BR, ---) et on extrait les identifiants backtick du champ $5.
while IFS= read -r t; do
    [[ -z "$t" ]] && continue
    checked_tests=$((checked_tests + 1))
    if ! grep -rq "fn ${t}\b" crates/; then
        echo "  MISSING test: $t"
        errors=$((errors + 1))
    fi
done < <(
    grep '^|' "$MATRIX" \
        | grep -v '^| *[-]' \
        | grep -v '^| *BT/BR' \
        | grep -v '^| *BR ' \
        | grep -v '^| *Requirement' \
        | awk -F'|' '{print $5}' \
        | grep -oP '`([a-z][a-z0-9_]+)`' \
        | tr -d '`' \
        | sort -u
)

echo "  $checked_tests test reference(s) checked."

# --- Résumé -------------------------------------------------------------------

echo ""
total=$((checked + checked_tests))
if [ $errors -eq 0 ]; then
    echo "OK: all $total references in the traceability matrix are valid."
else
    echo "FAILED: $errors broken reference(s) out of $total checked."
    exit 1
fi
