main = do
  n <- readLn :: IO Int
  as <- map read . words <$> getLine :: IO [Int]
  print $ f as 0 1000

f :: Integral p => [p] -> p -> p -> p
f (a : []) stock money = money + stock * a
f (a : b : xs) stock money
  | a > b =
    let m = money + stock * a
        (s, m') = m `divMod` b
     in f (b : xs) s m'
  | otherwise =
    let (s, m') = money `divMod` a
     in f (b : xs) (stock + s) m'

