import           Control.Monad (replicateM)

main = do
  [n, m, k] <- map read . words <$> getLine
  [as, bs] <- replicateM 2
    $ zip [0 ..] . scanl (+) 0 . takeWhile (<= k) . map read . words
    <$> getLine
  let f (a:as) (b:bs)
        | (sum . map snd) [a, b] <= k = [(sum . map fst) [a, b]] ++ f as (b:bs)
        | otherwise = f (a:as) bs
      f [] _ = []
      f _ [] = []
      ans = maximum $ f as $ reverse bs
  print ans

-- ソートされた配列を両側から走査するアルゴリズム
-- 配列のそれぞれの要素を1回だけ見ればよいので高速
