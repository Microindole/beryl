#!/bin/bash

# 1. Rename .brl to .lcy
echo "Renaming .brl files to .lcy..."
find . -name "*.brl" | while read f; do
    mv "$f" "${f%.brl}.lcy"
done

# 2. Rename directories in crates/
echo "Renaming directories in crates/..."
if [ -d "crates" ]; then
    for dir in crates/beryl_*; do
        if [ -d "$dir" ]; then
            newname="${dir/beryl/lency}"
            echo "Renaming $dir to $newname"
            mv "$dir" "$newname"
        fi
    done
fi

# 3. Rename binaries/other specific dirs if needed
# (Assuming binaries are managed by cargo and output to target, which we'll clean)

# 4. Text Replacement
# We'll use perl for in-place replacement as sed -i differs between BSD/GNU (though Linux env is likely GNU).
# Exclude target/, .git/
echo "Replacing text content..."

find . -type f -not -path "./target/*" -not -path "./.git/*" -not -path "./.gemini/*" -not -name "rename_project.sh" | while read -r file; do
    # Check if file serves as text
    if grep -qI "." "$file"; then
        # beryl -> lency
        sed -i 's/beryl/lency/g' "$file"
        # Beryl -> Lency
        sed -i 's/Beryl/Lency/g' "$file"
        # BERYL -> LENCY
        sed -i 's/BERYL/LENCY/g' "$file"
    fi
done

# 5. Clean target to avoid confusion
echo "Cleaning target..."
rm -rf target

echo "Done."
