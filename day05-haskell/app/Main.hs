module Main where

import Lib

main :: IO ()
main = do
    let inputFile = "input.txt"
    putStrLn $ "Advent of Code - Day 05"
    putStrLn $ "Reading input from: " ++ inputFile
    input <- readFile inputFile
    putStrLn $ "Part 1: " ++ show (part1 input)
    putStrLn $ "Part 1: " ++ show (part2 input)
