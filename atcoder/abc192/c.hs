import Control.Applicative
import Control.Monad
import Data.Array.ST
import qualified Data.Char as Char
import Data.Functor
import Data.List
import Data.Maybe
import Data.STRef
import qualified Data.Vector as V

-- IO
getLineAsList :: (Read a) => IO [a]
getLineAsList = map read . words <$> getLine

f :: Int -> Int
f i =
  if null l then 0 else read l
 where
  l = dropWhile (== '0') . sort . show $ i

f' :: Int -> Int
f' = read . reverse . sort . show

g :: Int -> Int -> Int
g x 0 = x
g x k = g (f' x - f x) (k - 1)

-- main
main :: IO ()
main = do
  (n : k : _) <- getLineAsList :: IO [Int]
  print $ g n k

