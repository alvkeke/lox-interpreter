
extern crate lox_lib;

use lox_lib::LoxParser;

#[test]
fn test_calc() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_CALC.to_string());
    assert_eq!(lox.console_take(), "137\n");
}

#[test]
fn test_scope() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_SCOPE.to_string());
    assert_eq!(lox.console_take(), "n1\nn2\nn1\ninner a\nouter b\nglobal c\nouter a\nouter b\nglobal c\nglobal a\nglobal b\nglobal c\n");
}

#[test]
fn test_if() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_IF.to_string());
    assert_eq!(lox.console_take(), "1\n2\n3\n123\n111\n");
}

#[test]
fn test_while() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_WHILE.to_string());
    assert_eq!(lox.console_take(), "0\n1\n2\n3\n4\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
}

#[test]
fn test_for() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_FOR.to_string());
    assert_eq!(lox.console_take(), "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144\n233\n377\n610\n987\n1597\n2584\n4181\n6765\n=========================\n0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144\n233\n377\n610\n987\n1597\n2584\n4181\n6765\n=========================\n0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144\n233\n377\n610\n987\n1597\n2584\n4181\n6765\n");
}

#[test]
fn test_function() {
    let mut lox = LoxParser::new_test();
    lox.exec_code(&CODE_FUNCTION.to_string());
    assert_eq!(lox.console_take(), "entered fn1\n1\nentered fn2\n1\nentered fn3\n1\n1\nentered fn4\n1\n2\n3\n4\nentered fn4\n5\n6\n7\n8\n9\nentered fn4\n10\n11\n12\n13\n14\nentered outside\nentered inside\nentered inside\n0\nentered inside\n0\n1\n");
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

fun fn4(arg1, arg2) {
    print \"entered fn4\";
    for (var i=arg1; i<arg2; i=i+1) {
        print i;
    }
}

fn4(1, 5);
fn4(5, 10);
fn4(10, 15);

fun inside(arg) {
    print \"entered inside\";
    for (var i=0; i<arg; i=i+1) {
        print i;
    }
}

fun outside(arg) {
    print \"entered outside\";
    for (var i=0; i<arg; i=i+1) {
        inside(i);
    }
}

outside(3);
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

