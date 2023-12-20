
# JPEG find date 

## Overview
This repo intends to demonstrate extracting dates from a jpeg file using type systems. More importantly, this is a repo intending to demonstrate how to use various language type systems to effectively "design-out" bad behaviour. Having learned python as a first language, I don't have intuition for how to use type systems properly and want to improve. 

The code included in this repo aims to handle the minimum number of edgecases for input jpeg files. 

### JPEG byte structure
The below diagram illustrates the typical byte breakdown of a jpeg file

```markdown
+---------------------+
| Start of Image (SOI)| 2 bytes - Marks the start of the image
+---------------------+
| Application Data    | Varies (typically 1,000 bytes) 
| (APP0, APP1 - Exif) | Contains metadata, including Exif data include datetime
+---------------------+
| Quantization Tables | 64 bytes - Contains two tables used in the JPEG compression process
+---------------------+
| Huffman Tables      | Varies (typically 1,000 bytes). Contains two Huffman tables used for compression
+---------------------+
| Start of Frame (SOF)| 19 bytes. Image's basic dimensions and the type of JPEG compression used
+---------------------+
| Scan Data           | Varies (large), Contains the actual image data, which has been compressed using the Huffman tables and quantization tables
+---------------------+
| End of Image (EOI)  | 2 bytes - Marks the end of the image
+---------------------+
```


### JPEG Exif byte structure
Similar to above, the below diagram illustrates a byte breakdown of the Exif data section which contains created date.
NOTE: we are looking for Date Create which corresponds with integers 36867, 36868, 306 as shown below:

```markdown
+---------------------+
| Start of Image (SOI)| 2 bytes
+---------------------+
| Application Data    |
| (APP0, APP1 - Exif) | Varies, but typically around 1,000 bytes
| +-------------------+
| | Exif Start        | 2 bytes - Marks the start of the Exif data 
| +-+-----------------+
| | | TIFF Header     | 8 bytes - Contains information about the file
| | +-----------------+
| | | Byte Order      | 2 bytes - Indicates the byte order ('II' for little-endian, 'MM' for big-endian)
| | +-----------------+
| | | Magic Number    | 2 bytes - Always '42'
| | +-----------------+
| | | Offset to IFD   | 4 bytes - Integer offset of bytes to the first Image File Directory (IFD). Usually 8
| | +-----------------+
| +-------------------+
| | IFD               | Varies - Contains metadata about the image
| | +-----------------+
| | | Number of Entries| 2 bytes - Number of entries in the IFD
| | +-----------------+
| | | Entries         | Varies - Each entry is 12 bytes long and consists of the tag number, the data format, the number of components, and the value itself
| | +-----------------+
| +-------------------+
| | Date Created      | Typically located in the entry with tag number 36867 (DateTimeOriginal) or 36868 (DateTimeDigitized), or 306 (DateTime)
| +-------------------+
+---------------------+
| EVERYTHING ELSE     |
+---------------------+
| End of Image (EOI)  | 2 bytes
+---------------------+
```

Using above table as a reference lets break down the JPEG EXIF byte structure. For readability it has been converted to ASCII and split across multiple lines:

```
\255\216\255\225\ETX\177Exif\NUL\NULII*\NUL\b\NUL\NUL\NUL\r\NUL\NUL\SOH\ETX\NUL\SOH\NUL\NUL\NUL\192\SI\NUL\NUL\SOH\SOH\ETX\NUL\SOH\NUL\NUL\NUL\208\v\
NUL\NUL\SI\SOH\STX\NUL\a\NUL\NUL\NUL\170\NUL\NUL\NUL\DLE\SOH\STX\NUL\b\NUL\NUL\NUL\177\NUL\NUL\NUL\DC2\SOH\ETX\NUL\SOH\NUL\NUL\NUL\SOH\NUL\NUL\NUL\SUB
\SOH\ENQ\NUL\SOH\NUL\NUL\NUL\185\NUL\NUL\NUL\ESC\SOH\ENQ\NUL\SOH\NUL\NUL\NUL\193\NUL\NUL\NUL(\SOH\ETX\NUL\SOH\NUL\NUL\NUL\STX\NUL\NUL\NUL1\SOH\STX\NUL
\NAK\NUL\NUL\NUL\201\NUL\NUL\NUL2\SOH\STX\NUL\DC4\NUL\NUL\NUL\222\NUL\NUL\NUL\DC3\STX\ETX\NUL\SOH\NUL\NUL\NUL\SOH\NUL\NUL\NULi\135\EOT\NUL\SOH\NUL\NUL
\NUL\242\NUL\NUL\NUL%\136\EOT\NUL\SOH\NUL\NUL\NUL\131\ETX\NUL\NUL\NUL\NUL\NUL\NULGoogle\NULPixel
```

#### Start of Image (SOI) - 2 bytes

```
\255\216
```

These are specific to jpeg and should always be 255 and 216... Not sure why

#### Exif Start - 2 bytes

```
\255\255
```

#### TIFF Header - 8 bytes

```
ETX\177Exif
```

`ETX` is 1 byte, `\177` is 1 byte and `Exif` is 4 bytes, `\NUL\NUL` is 2 bytes = 8 bytes

#### Byte Order - 2 bytes

`II`

Byte order is either `II` little endian (Least Significant Byte i.e. LSB) or `MM` big endian (Most Significant Byte i.e. MSB) and represents
the order in which to read bytes - right to left or left to right. This concept is especially important for networking, computer systems, binary manipulation, encoding and others.

#### Magic Number - 1 byte

`*`

The star a.k.a asterisk is assigned to ascii integer 42 and is often called the magic number. This is probably used as a check.

#### Offset to IFD - 4 bytes

Offset in bytes from the beginning of the TIFF header.

```
\NUL\b\NUL\NUL\  
```

\NUL is 1 byte with a decimal value of 0, \b is the backspace character and is 1 byte with a decimal of 8. In little-endian format, the bytes are read from right to left. In order to read in little-endian we convert to 32-bit integer:

hmmmm TODO: understand what the go is for how to get that number. Below goes into converting into binary and reading little-endian:

`00000000 00000100 00000000 00000000`

Here is the decimal form
'       0      8          0        0'

If we read this from right to left (little-endian), we get:

`00000000 00000000 00000100 00000000`

This is the binary representation of the decimal number 256. Lets convert this binary number to decimal and identify the offset value from the start of the file:

`0 * 2^31 + 0 * 2^30 + 0 * 2^29 + 0 * 2^28 + 0 * 2^27 + 0 * 2^26 + 0 * 2^25 + 0 * 2^24 + 0 * 2^23 + 0 * 2^22 + 0 * 2^21 + 0 * 2^20 + 0 * 2^19 + 0 * 2^18 + 0 * 2^17 + 0 * 2^16 + 0 * 2^15 + 0 * 2^14 + 0 * 2^13 + 0 * 2^12 + 0 * 2^11 + 0 * 2^10 + 0 * 2^9 + 0 * 2^8 + 1 * 2^7 + 0 * 2^6 + 0 * 2^5 + 0 * 2^4 + 0 * 2^3 + 0 * 2^2 + 0 * 2^1 + 0 * 2^0 = 256`

hint: This is the only binary sum
```
2^8 + 1 = 256
```

We already have the 8 byte offset... ?

#### Number of Entries

Number of entries (12 bytes per entry) within the Image File Directory (IFD)

`\r`

The \r character is 1 byte, also known as the carriage return character and is represented in decimal as 13. In hexadecimal, it is represented as 0D or 0x0D.
We have 13 entries in the IFD.

#### Entries
The novel purpose is to find the date the photo was created. There are a few different types but I've chosen:

* DateTimeOriginal: tag number 36867
* DateTimeDigitized: tag number 36898
* DateTime: tag 306

Each entry contains a tag number at the first 2 bytes of each entry. These are identifier codes for various metadata including datetimes.

## python

### Quickstart

```python
poetry install
poetry run python -m src.main
```

output:
```bash
Date taken: 2022:01:06 15:22:30
```

### run tests:

```python
poetry install --with dev
poetry run python -m pytest
```

TODO: include description of python

## haskell

TODO: include description of haskell and learnings

## java

TODO: include description of java and learnings

## rust

TODO: include description of rust and learnings
