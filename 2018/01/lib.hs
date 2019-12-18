import Data.IntSet (IntSet)
import Data.String.Utils

stripPlus str = last (split "+" str)
asInt str = read str :: Int

-- main :: IO()
main = do
    input <- lines <$> readFile "input"
    -- inpuat_ <- map(asInt) input
    print input
    map(asInt . stripPlus) input
    print "done"
-- import System.IO (readFile)
-- main = do
--     lines <- readFile "input"
--     putStr lines

-- freq = []
--
-- main = do
--     handle <- openFile "input" ReadMode
--     changeFreq <- hGetContents handle :: Int
--     ++ freq + changeFreq
--     hClose handle
