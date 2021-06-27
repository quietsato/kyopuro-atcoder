import Control.Monad

main = do
  k <- readLn :: IO Int
  print . f $ k

f :: Int -> Int
f k
  | gcd k 10 /= 1 = -1
  | otherwise = g l 10 1
  where
    l
      | k `mod` 7 == 0 = 9 * k `div` 7
      | otherwise = 9 * k

g :: Int -> Int -> Int -> Int
g l a i
  | m == 1 = i
  | otherwise = g l (10 * m) (i + 1)
  where
    m = a `mod` l

