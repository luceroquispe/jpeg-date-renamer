from src import JPEG_SAMPLE_PATH
    
def test_sample_file_exists():
    assert JPEG_SAMPLE_PATH.exists()
