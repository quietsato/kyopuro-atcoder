import Control.Monad (forM, forM_, replicateM)
import Data.Array.IO
  ( IOUArray,
    MArray (newArray),
    readArray,
    writeArray,
  )

main :: IO ()
main = do
  n <- readLn :: IO Int
  abcs <- replicateM n $ map read . words <$> getLine :: IO [[Int]]
  memo <- newArray ((0, 0), (n, 2)) 0 :: IO (IOUArray (Int, Int) Int)
  forM_ (zip [1 ..] abcs) $ \(i, abc) -> do
    forM_ actions $ \j -> do
      newValue <- maximum <$> forM (filter (/= j) actions) (\jj -> readArray memo (i - 1, jj) :: IO Int)
      writeArray memo (i, j) $ newValue + abc !! j
  print . maximum =<< forM actions (\j -> readArray memo (n, j) :: IO Int)
  where
    actions = [0 .. 2]

{-
  C - Vacation
  https://atcoder.jp/contests/dp/tasks/dp_c

  dp[0][j] = 0 -- 初期条件

 -- 前日と同じ行動は取れない
  dp[i][0] = a[i] + max(dp[i-1][1],dp[i-1][2])
  dp[i][1] = b[i] + max(dp[i-1][0],dp[i-1][2])
  dp[i][2] = c[i] + max(dp[i-1][1],dp[i-1][1])

  これに従って 1 <= i <= N, 0 <= j <= 2 で昇順にDPテーブルを構築する
  解は max(dp[N][0], dp[N][1], dp[N][2]) となる
-}
