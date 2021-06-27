main = do
  n <- readLn :: IO Int
  as <- map read . words <$> getLine :: IO [Int]
  let as' = map snd . filter (odd . fst) $ zip [1 ..] as
  print . length $ filter odd [x | x <- as']
