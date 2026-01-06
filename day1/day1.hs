main = do

    content <- readFile "input.txt"
    let fileLines = lines content
    let d = calc fileLines
    let digits = 50 : d
    let res = getRes (reverse digits)
    print (sum digits)
    print res


getNumber :: String -> Int
getNumber (c:l)
    | c == 'R' =  read l :: Int
    | c == 'L' = 100 - (read l :: Int)




calc :: [String] -> [Int]
calc [] = []
calc (x:(xs)) = getNumber x : calc xs




getRes :: [Int] -> Int
getRes [] = 0
getRes x
    | sum x `mod` 100 == 0 = 1 + getRes (tail x)
    | otherwise = getRes (tail x) 



