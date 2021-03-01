import           Data.Array.IO

main = do
  n <- readLn :: IO Int
  let as = flip
        map
        [(x, y, z)
        | x <- [1 .. 100]
        , y <- [1 .. 100]
        , z <- [1 .. 100]
        , x ^ 2 + y ^ 2 + z ^ 2 + x * y + y * z + z * x <= 10 ^ 4]
        $ \(x, y, z) -> x ^ 2 + y ^ 2 + z ^ 2 + x * y + y * z + z * x
  ar <- newArray (1, n) 0 :: IO (IOUArray Int Int)
  flip mapM_ as $ write n ar
  flip mapM_ [1 .. n] $ \i -> print =<< readArray ar i
  where
    write n ar i
      | i <= n = writeArray ar i =<< (+ 1) <$> (readArray ar i)
      | otherwise = return ()
