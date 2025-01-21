platform ""
    requires {} { main! : List Str => Result {} [Exit I32 Str]_ }
    exposes []
    packages {}
    imports []
    provides [main_for_host!]

main_for_host! : {} => I32
main_for_host! = |_|
    when main!(["app", "arg1"]) is
        Ok({}) -> 0
        Err(_) -> 1
