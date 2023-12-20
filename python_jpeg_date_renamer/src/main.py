"""
This python script reads a jpeg file and extracts the date time 
when the jpeg image was created. See README.md for logic and details.

NOTE: This doesn't handle edgecases and makes assumptions 
"""

import struct
from typing import Union

from src import JPEG_SAMPLE_PATH


def get_exif_data_slice(filename: str) -> bytes:
    """Find the byte index of exif start and return byte slice of Exif data

    The Exif contains metadata including the datetime
    """
    with open(filename, "rb") as f:
        data = f.read()
        print(f"First 20 bytes: {data[:20]}")
    start = data.find(b"\xFF\xE1")
    if start == -1:
        print("No Exif data found")
        return None
    # ">" represents bigendian and "H" represents unsigned short integers
    length = struct.unpack(">H", data[start + 2 : start + 4])[0]
    return data[start + 4 : start + length]

def endianness_from_exif_data(exif_bytes: bytes) -> Union[str, None]:
   byte_to_endianness = {b"II": "<", b"MM": ">"}
   if exif_bytes in byte_to_endianness:
       return byte_to_endianness[exif_bytes]
   else:
       print("Invalid TIFF header: Unexpected byte order")
       return None

def get_date_string_from_exif_data(exif_data) -> Union[str, None]:
    """Exif data starts with 'Exif\0\0'

    After that, the TIFF header starts which begins with the byte
    order ('II' for little-endian, 'MM' for big-endian), followed
    by '42' (the magic number), and then the offset to the Image File Directory (IFD),
    which is usually 8."""
    # Read the byte order from the TIFF header
    tiff_header_start = exif_data[6:]
    byte_order = tiff_header_start[:2]
    endianness = endianness_from_exif_data(byte_order)
    # Read the offset to the first IFD
    first_ifd_offset = struct.unpack(endianness + "I", tiff_header_start[4:8])[0]

    # The IFD starts with the number of entries in the directory. Each entry is
    # 12 bytes long and consists of the tag number, the data format, the number
    # of components, and the value itself.
    ifd = tiff_header_start[first_ifd_offset:]
    num_entries = struct.unpack(endianness + "H", ifd[:2])[0]
    print(f"Num entries: {num_entries}")

    # Loop over the entries to find the tag numbers corresponding with time:
    #     DateTimeOriginal number 36867
    #     DateTimeDigitized number 36898
    #     DateTime 306

    for i in range(num_entries):
        # each entry has 12 bytes. Iterate over 12 byte chunks
        # the first 2 bytes indicate number of entries only
        entry = ifd[2 + i * 12 : 2 + (i + 1) * 12]
        tag_number = struct.unpack(endianness + "H", entry[:2])[0]

        if tag_number in [36867, 36868, 306]:
            # The value is a string and is located at the offset given in the
            # last 4 bytes of the entry
            value_offset = struct.unpack(endianness + "I", entry[8:])[0]
            # 19 bytes in the ASCII datetime string format called ExifDateTime
            # e.g. 2023:11:08 12:34:56
            value = tiff_header_start[value_offset : value_offset + 19]
            return value.decode("utf-8")

    print("No DateTimeOriginal tag found in Exif data")
    return None


if __name__ == "__main__":
    path = JPEG_SAMPLE_PATH
    exif_data = get_exif_data_slice(path.__str__())
    if exif_data is not None:
        date = get_date_string_from_exif_data(exif_data)
        if date is not None:
            print("Date taken:", date)
