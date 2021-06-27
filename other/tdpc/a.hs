import Control.Monad (forM, forM_, when, guard)
import Data.Array.IO
  ( IOUArray,
    MArray (newArray),
    readArray,
    writeArray,
  )

main :: IO ()
main = do
  n <- readLn :: IO Int
  ps <- map read . words <$> getLine :: IO [Int]
  let m = sum ps
  memo <- newArray ((0, 0), (n, m)) False :: IO (IOUArray (Int, Int) Bool)
  forM_ [0 .. n] $ \i -> writeArray memo (i, 0) True
  forM_ (zip [1 .. n] ps) $ \(i, v) ->
    forM_ [0 .. m] $ \j -> do
      case1 <- readArray memo (i - 1, j) -- i番目を使わなくてもすでにできているか
      case2 <- if v <= j then readArray memo (i - 1, j - v) else return False -- i番目を使うことで作成できるか
      when (case1 || case2) $ writeArray memo (i, j) True
  print . length . filter (== True) =<< forM [0 .. m] (\x -> readArray memo (n, x) :: IO Bool)

{-
  A - コンテスト
  https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

  dp[i][0] = 0 (初期条件，0はどのように選んでも作れる)

  dp[i][j] = 1 (if dp[i-1][j] = 1)
  dp[i][j] = 1 (if dp[i-1][j-a[i]] = 1 and j >= a[i])
  dp[i][j] = 0 (otherwise)
-}