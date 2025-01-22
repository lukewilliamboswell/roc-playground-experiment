platform ""
    requires {} { main : Str -> Str }
    exposes []
    packages {}
    imports []
    provides [main_for_host, main_for_host_two]

main_for_host : {} -> Str
main_for_host = |{}|
    main "luke"

main_for_host_two : I64 -> Str
main_for_host_two = |_|
    main "romona"
