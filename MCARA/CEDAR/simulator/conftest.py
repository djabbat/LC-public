"""Make `cdata_sim` importable from the repository root without installation."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
