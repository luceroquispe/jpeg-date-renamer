import logging
from pathlib import Path

# Set up logging
LOGLEVEL = logging.INFO

# Create global logger
logging.basicConfig(
    level=LOGLEVEL,
    format="{asctime} - {levelname} - {module}::{funcName}::{lineno} - {message}",
    style="{",
    datefmt="%Y-%d-%m %I:%M:%S %z",
)

logger = logging.getLogger(__name__)

# Project paths
ROOT_PATH = Path(__file__).parents[1].absolute()
SRC_PATH = ROOT_PATH / "src"
UNITTEST_PATH = ROOT_PATH / "unit_tests"
JPEG_SAMPLE_PATH = Path(__file__).parents[2].absolute() / "vista.jpg"
