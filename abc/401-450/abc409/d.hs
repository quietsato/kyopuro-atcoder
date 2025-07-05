import           Control.Monad
import           Data.List     (find)

main :: IO ()
main = do
    t <- readLn :: IO Int
    forM_ [1..t] $ const solve

solve :: IO ()
solve = do
    _ <- readLn :: IO Int
    s <- getLine
    let
        l = solveL s
        r = solveR s l
    -- print $ show l ++ " " ++ show r
    let
        seg1 = take l s
        seg2 = take (r-l) (drop (l+1) s)
        seg3 = [s !! l]
        seg4 = drop (r+1) s
    -- forM_ [seg1, seg2, seg3, seg4] $ \s -> do
    --     putStr $ "[" ++ s ++ "]"
    putStrLn $ join [seg1, seg2, seg3, seg4]

solveL :: String -> Int
solveL s = case match of
    Just (i, _, _) -> i
    Nothing        -> 0
    where
        s1 = drop 1 s
        -- m[o][n]ey -> m[n]...[o]...
        match = find (\(_, c1, c2) -> c1 > c2) $ zip3 [0..] s s1

solveR :: String -> Int -> Int
solveR s l = case match of
        Just (i, _) -> i - 1
        Nothing     -> length s - 1
    where
        s1 = drop (l+1) $ zip [0..] s
        cl = s !! l
        -- m[n]...[o]... -> m[n]e[o]y
        match = find ((> cl) . snd) s1

