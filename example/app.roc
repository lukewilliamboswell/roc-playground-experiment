app [main] { pf: platform "platform.roc" }

import Counter

apples : U64
apples = 5

main : Str -> Str
main = |name|
    "hello ${name}!, I have ${Num.to_str apples}, count ${Num.to_str Counter.count}"
