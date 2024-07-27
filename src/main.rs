mod parser;

use parser::LoxParser;


fn main() {

    let mut lox = LoxParser::new();
    test(&mut lox);
    // if let Err(msg) = lox.repl() {
    //     eprintln!("failed to read line: {}", msg);
    // }

}



fn test(lox: &mut LoxParser) {
    println!("{:#?}", lox.exec_line(&CODE_CALC.to_string()));
    println!("{:#?}", lox.exec_line(&CODE_SCOPE.to_string()));
    println!("{:#?}", lox.exec_line(&CODE_IF.to_string()));
    println!("{:#?}", lox.exec_line(&CODE_WHILE.to_string()));

    dbg!(&lox);
    lox.env_clear();
    dbg!(lox);
}


const CODE_WHILE: &str = "
    var n = 0;
    while (n<5) {
        print n;
        n = n+1;
    }
";

const CODE_IF: &str = "
    if (true) {
        print 1;
    } else {
        print 2;
    }
    if (false) {
        print 1;
    } else {
        print 2;
    }

    var n = 5;
    if (n==3) {
        print 1;
    } else if (n==4) {
        print 2;
    } else if (n==5)
        print 3;
    if (true)
        if (true)
            print 123;
    var n=5;
    if (n == 4 or n==5)
        print 111;
";

const CODE_SCOPE: &str = "
{{{
    {
        var n=\"n1\";
        print n;
        {
            var n=\"n2\";
            print n;
        }
        print n;
    }
    var a = \"global a\";
    var b = \"global b\";
    var c = \"global c\";
    {
        var a = \"outer a\";
        var b = \"outer b\";
        {
            var a = \"inner a\";
            print a;   print b;
            print c;
        }
        print a;
        print b;
        print c;
    }
    print a;
    print b;
    print c;
}}}
";

const CODE_CALC: &str = "
var n = 3 * (4 + 5 * 9) - 10;
print n;
";

