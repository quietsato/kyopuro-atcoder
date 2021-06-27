main = do
  (l:r:d:_) <- map read . words <$> getLine :: IO [Int]
  print . length $ [x | x <- [l .. r], x `mod` d == 0]
