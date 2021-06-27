import qualified Data.Char as Char
main =
  putStrLn . f . and . zipWith (\f c -> f c) (cycle [Char.isLower, Char.isUpper]) =<< getLine
 where
  f True = "Yes"
  f False = "No"

