import Data.Array

main = do
  n <- readLn
  cs <- getLine
  let ar = listArray (1, n) cs
      f l r i
        | r <= l = i
        | ar ! l == 'R' = f (l + 1) r i
        | ar ! r == 'W' = f l (r - 1) i
        | otherwise = f (l + 1) (r - 1) (i + 1)
  print $ f 1 n 0

