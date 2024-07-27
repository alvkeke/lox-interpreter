mod parser;

use parser::LoxParser;


fn main() {

    let mut lox = LoxParser::new();
    test(&mut lox);
    if let Err(msg) = lox.repl() {
        eprintln!("failed to read line: {}", msg);
    }

}



fn test(lox: &mut LoxParser) {
    let code = String::from("

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

    var n = 3 * (4 + 5 * 9) - 10; print n;


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

    ");

    println!("{:#?}", lox.exec_line(&code));

    lox.env_clear();
}

