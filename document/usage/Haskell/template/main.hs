solve :: String
solve =
  "Your solved code"

readLineToInt :: IO Int
readLineToInt = read <$> getLine

readLineToIntArray :: IO [Int]
readLineToIntArray = map read . words <$> getLine

main :: IO ()
main = do
  n <- readLineToInt
  print n

-- putStrLn $ solve
