#! /bin/bash

cat > .git/hooks/pre-commit <<EOL
#! /bin/bash

scripts/test.sh
EOL
chmod +x .git/hooks/pre-commit
