main = do
  x <- readLn :: IO Int
  putStrLn $ if x >= 30 then "Yes" else "No"

