app [main] { pf: platform "platform.roc" }

import Store

main = |name|
    greeting = "Ahoy"
    """
    ${greeting} there ${name}, you have;
        - ${Num.to_str Store.apples} apples, and
        - ${Num.to_str Store.bananas} bananas.
    """
