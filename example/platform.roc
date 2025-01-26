platform ""
    requires {} { main : Str -> Str }
    exposes []
    packages {}
    imports []
    provides [main_for_host]

main_for_host : {} -> Str
main_for_host = |{}|
    main "luke"
