import Data.Array.IO
import Control.Monad

main = do
  [n,k] <- map read . words <$> getLine :: IO [Int]
  as <- (newListArray (0,n-1) =<< (map read . words <$> getLine :: IO [Int])):: IO (IOArray Int Int)
  let ixs = [0..n-1]
  flip mapM_ (zip ixs $ drop (k) ixs) $ \(i,j) -> do
      x <- readArray as i
      y <- readArray as j
      putStrLn $ if y > x then "Yes" else "No"
