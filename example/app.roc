app [main] { pf: platform "platform.roc" }

greeting : Str
greeting = "hello"

main : Str -> Str
main = |name|
    "${greeting} ${name}!"
