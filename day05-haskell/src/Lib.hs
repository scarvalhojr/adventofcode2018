module Lib
    ( part1
    , part2
    ) where

import Data.Char (toLower)
import Data.List (nub)

part1 :: String -> Int
part1 = length . reduce

reduce :: String -> String
reduce = foldl react ""
    where react [] c = [c]
          react (x:xs) c
            | x /= c && toLower x == toLower c  = xs
            | otherwise                         = c : x : xs

part2 :: String -> Int
part2 p = minimum $ map (flip evaluate p) units
    where units = nub (map toLower p)

evaluate :: Char -> String -> Int
evaluate c = length . reduce . filter ((/=) c . toLower)
