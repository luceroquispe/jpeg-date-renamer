import Data.ByteString qualified as B

-- Bytesting is a type which implements the type class show (smiilar to trait implementating another class)
-- This is why we can pass to print and it'll know how to convert to string in stdout
--

readJpeg :: FilePath -> IO B.ByteString
readJpeg filePath = do
  B.readFile filePath

main :: IO ()
main = do
  content <- readJpeg "../20220113_1040_troopy.jpg"
  print content
