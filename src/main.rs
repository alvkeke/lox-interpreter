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
    println!("{:#?}", lox.exec_line(&CODE_FOR.to_string()));
    println!("{:#?}", lox.exec_line(&CODE_FUNCTION.to_string()));

    dbg!(&lox);
    lox.clear();
    dbg!(lox);
}

const CODE_FUNCTION: &str = "

fun fn1() {
    print \"entered fn1\";
    print 1;
}

fun fn2(arg1) {
    print \"entered fn2\";
    print arg1;
}

fun fn3(arg1, arg2) {
    print \"entered fn3\";
    print arg1;
    print arg2;
}

fn1();

var arg1 = 1;

fn2(arg1);
fn3(arg1, arg1);
fn3(1, 1+2, nn==4);

";

const CODE_FOR: &str = "
var temp;
var a = 0;
for (var b=1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
print \"=========================\";
var a = 0;
var b = 1;
for (b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
print \"=========================\";
var a = 0;
var b = 1;
for (;a < 10000;) {
    print a;
    temp = a;
    a = b;
    b = temp + b;
}
";


const CODE_WHILE: &str = "
var n = 0;
while (n<5) {
    print n;
    n = n+1;
}

{
  var i = 0;
  while (i < 10) {
    print i;
    i = i + 1;
  }
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

