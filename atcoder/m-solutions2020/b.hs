main = do
  [a, b, c] <- map read . words <$> getLine :: IO [Int]
  k <- readLn :: IO Int
  putStrLn $ ans k $ f a b c 0

f :: Int -> Int -> Int -> Int -> Int
f a b c i
  | a >= b = f a (b * 2) c i + 1
  | b >= c = f a b (c * 2) i + 1
  | otherwise = i

ans k i
  | k >= i = "Yes"
  | otherwise = "No"
