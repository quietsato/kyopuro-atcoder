import Control.Applicative
import Control.Monad
import Data.Array.ST
import Data.Function
import Data.Functor
import Data.Maybe (fromMaybe)
import Data.STRef
import qualified Data.Vector as V

-- IO
getLineAsList :: (Read a) => IO [a]
getLineAsList = map read . words <$> getLine

main :: IO ()
main = do
    n <- readLn :: IO Int
    let xMin = floor . calcOriginalPrice $ n
        xMax = ceiling . calcOriginalPrice $ n
        candidate = filter ((==) n . floor . (ratio *) . fromInteger) [xMin .. xMax]
    putStrLn $ (guard . null) candidate $> ":(" & (fromMaybe . show . head) candidate
  where
    ratio = 1.08 :: Double
    calcOriginalPrice = (/ ratio) . realToFrac
