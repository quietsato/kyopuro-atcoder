main =
  print . (100 -) . (100 *) . uncurry (/) . tuple . reverse . map read . words =<< getLine
  where
    tuple (x:y:_) = (x,y)

