main = do
  x <- readLn :: IO Int
  putStrLn $ show $ 8 - div (x-400) 200
  
