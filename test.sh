#!/bin/bash
# Convenience wrapper for the main test runner
# Allows running tests from project root without typing full path

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/tests/scripts/run_tests.sh" "$@"
