import Control.Monad
main = do
  n <- readLn :: IO Int
  apxs <- replicateM n $ map read . words <$> getLine :: IO [[Int]]
  let l = filter (\[_, x] -> x > 0) $ map (\[a, p, x] -> [p, x - a]) apxs
  print $ if null l then -1 else minimum $ map head l

