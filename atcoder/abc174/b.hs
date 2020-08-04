import Control.Monad

main = do
  [n, d] <- map read . words <$> getLine :: IO [Int]
  xys <- replicateM n $ map read . words <$> getLine :: IO [[Double]]
  print . length . filter (d >=) $ [(ceiling . sqrt) (x ^ 2 + y ^ 2) | (x : y : []) <- xys]

