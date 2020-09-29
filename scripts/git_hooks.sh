#! /bin/bash

cat > .git/hooks/pre-commit <<EOL
#! /bin/bash
cargo test
EOL
chmod +x .git/hooks/pre-commit
