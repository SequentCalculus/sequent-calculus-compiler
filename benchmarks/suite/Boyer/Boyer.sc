data List[A] { Nil, Cons(a: A, as: List[A]) }

data Pair[A, B] { Tup(a: A, b: B) }

data Bool { True, False }

data Unit { Unit }

codata Fun[A, B] { apply(a: A): B }

data Id {
    A,
    B,
    C,
    D,
    X,
    Y,
    Z,
    U,
    W,
    ADD1,
    AND,
    APPEND,
    CONS,
    DIFFERENCE,
    EQUAL,
    EVEN,
    EXP,
    F,
    FALSE,
    FOUR,
    IF,
    IMPLIES,
    LENGTH,
    LESSP,
    MEMBER,
    NIL,
    NOT,
    ONE,
    OR,
    PLUS,
    QUOTIENT,
    REMAINDER,
    REVERSE,
    TIMES,
    TRUE,
    TWO,
    ZERO,
    ZEROP
}

data Term { Var(i: Id), Func(i: Id, t: List[Term], l: Fun[Unit, List[Pair[Term, Term]]]), ERROR }

def id_eq(i1: Id, i2: Id): Bool {
    i1.case {
        A =>
            i2.case {
                A => True,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        B =>
            i2.case {
                A => False,
                B => True,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        C =>
            i2.case {
                A => False,
                B => False,
                C => True,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        D =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => True,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        X =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => True,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        Y =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => True,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        Z =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => True,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        U =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => True,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        W =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => True,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        ADD1 =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => True,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        AND =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => True,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        APPEND =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => True,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        CONS =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => True,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        DIFFERENCE =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => True,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        EQUAL =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => True,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        EVEN =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => True,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        EXP =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => True,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        F =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => True,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        FALSE =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => True,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        FOUR =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => True,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        IF =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => True,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        IMPLIES =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => True,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        LENGTH =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => True,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        LESSP =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => True,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        MEMBER =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => True,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        NIL =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => True,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        NOT =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => True,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        ONE =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => True,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        OR =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => True,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        PLUS =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => True,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        QUOTIENT =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => True,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        REMAINDER =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => True,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        REVERSE =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => True,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        TIMES =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => True,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        TRUE =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => True,
                TWO => False,
                ZERO => False,
                ZEROP => False
            },
        TWO =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => True,
                ZERO => False,
                ZEROP => False
            },
        ZERO =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => True,
                ZEROP => False
            },
        ZEROP =>
            i2.case {
                A => False,
                B => False,
                C => False,
                D => False,
                X => False,
                Y => False,
                Z => False,
                U => False,
                W => False,
                ADD1 => False,
                AND => False,
                APPEND => False,
                CONS => False,
                DIFFERENCE => False,
                EQUAL => False,
                EVEN => False,
                EXP => False,
                F => False,
                FALSE => False,
                FOUR => False,
                IF => False,
                IMPLIES => False,
                LENGTH => False,
                LESSP => False,
                MEMBER => False,
                NIL => False,
                NOT => False,
                ONE => False,
                OR => False,
                PLUS => False,
                QUOTIENT => False,
                REMAINDER => False,
                REVERSE => False,
                TIMES => False,
                TRUE => False,
                TWO => False,
                ZERO => False,
                ZEROP => True
            }
    }
}

def term_ls_eq(h1t1: List[Term], h2t2: List[Term]): Bool {
    h1t1.case[Term] {
        Nil => True,
        Cons(h1, t1) =>
            h2t2.case[Term] {
                Nil => False,
                Cons(h2, t2) =>
                    term_eq(h1, h2).case {
                        True => term_ls_eq(t1, t2),
                        False => False
                    }
            }
    }
}

def term_eq(t1: Term, t2: Term): Bool {
    t1.case {
        Var(i1) =>
            t2.case {
                Var(i2) => id_eq(i1, i2),
                Func(i, t, l) => False,
                ERROR => False
            },
        Func(f1, ts1, l1) =>
            t2.case {
                Var(i2) => False,
                Func(f2, ts2, l2) =>
                    id_eq(f1, f2).case {
                        True => term_ls_eq(ts1, ts2),
                        False => False
                    },
                ERROR => False
            },
        ERROR => False
    }
}

def term_in_list(term: Term, ht: List[Term]): Bool {
    ht.case[Term] {
        Nil => False,
        Cons(h, t) =>
            term_eq(term, h).case {
                True => True,
                False => term_in_list(term, t)
            }
    }
}

def all_term(f: Fun[Term, Bool], ls: List[Term]): Bool {
    ls.case[Term] {
        Nil => True,
        Cons(t, ts) =>
            f.apply[Term, Bool](t)
                .case {
                    True => all_term(f, ts),
                    False => False
                }
    }
}

def replicate_term(n: i64, t: Term): List[Term] {
    if n == 0 { Nil } else { Cons(t, replicate_term(n - 1, t)) }
}

def find(vid: Id, ls: List[Pair[Id, Term]]): Pair[Bool, Term] {
    ls.case[Pair[Id, Term]] {
        Nil => Tup(False, ERROR),
        Cons(b, bs) =>
            b.case[Id, Term] {
                Tup(vid2, val2) =>
                    id_eq(vid, vid2).case {
                        True => Tup(True, val2),
                        False => find(vid, bs)
                    }
            }
    }
}

def map(f: Fun[Term, Term], l: List[Term]): List[Term] {
    l.case[Term] {
        Nil => Nil,
        Cons(x, xs) => Cons(f.apply[Term, Term](x), map(f, xs))
    }
}

def boyer_add1(t: Term): Term {
    Func(ADD1, Cons(t, Nil), new { apply(u) => Nil })
}

def boyer_zero(): Term {
    Func(ZERO, Nil, new { apply(u) => Nil })
}

def boyer_zerop(a: Term): Term {
    Func(
        ZEROP,
        Cons(a, Nil),
        new {
            apply(u) => Cons(Tup(boyer_zerop(boyer_x()), boyer_equal(boyer_x(), boyer_zero())), Nil)
        }
    )
}

def boyer_one(): Term {
    Func(ONE, Nil, new { apply(u) => Cons(Tup(boyer_one(), boyer_add1(boyer_zero())), Nil) })
}

def boyer_two(): Term {
    Func(TWO, Nil, new { apply(u) => Cons(Tup(boyer_two(), boyer_add1(boyer_one())), Nil) })
}

def boyer_four(): Term {
    Func(
        FOUR,
        Nil,
        new { apply(u) => Cons(Tup(boyer_four(), boyer_add1(boyer_add1(boyer_two()))), Nil) }
    )
}

def boyer_if_(a: Term, b: Term, c: Term): Term {
    Func(
        IF,
        Cons(a, Cons(b, Cons(c, Nil))),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_if_(boyer_if_(boyer_x(), boyer_y(), boyer_z()), boyer_u(), boyer_w()),
                        boyer_if_(
                            boyer_x(),
                            boyer_if_(boyer_y(), boyer_u(), boyer_w()),
                            boyer_if_(boyer_z(), boyer_u(), boyer_w())
                        )
                    ),
                    Nil
                )
        }
    )
}

def boyer_not_(a: Term): Term {
    Func(
        NOT,
        Cons(a, Nil),
        new {
            apply(u) =>
                Cons(
                    Tup(boyer_not_(boyer_x()), boyer_if_(boyer_x(), boyer_false(), boyer_true())),
                    Nil
                )
        }
    )
}

def boyer_and_(a: Term, b: Term): Term {
    Func(
        AND,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_and_(boyer_x(), boyer_y()),
                        boyer_if_(
                            boyer_x(),
                            boyer_if_(boyer_y(), boyer_true(), boyer_false()),
                            boyer_false()
                        )
                    ),
                    Nil
                )
        }
    )
}

def boyer_equal(a: Term, b: Term): Term {
    Func(
        EQUAL,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_equal(boyer_plus(boyer_x(), boyer_y()), boyer_zero()),
                        boyer_and_(boyer_zerop(boyer_x()), boyer_zerop(boyer_y()))
                    ),
                    Cons(
                        Tup(
                            boyer_equal(
                                boyer_plus(boyer_x(), boyer_y()),
                                boyer_plus(boyer_x(), boyer_z())
                            ),
                            boyer_equal(boyer_y(), boyer_z())
                        ),
                        Cons(
                            Tup(
                                boyer_equal(boyer_zero(), boyer_difference(boyer_x(), boyer_y())),
                                boyer_not_(boyer_lessp(boyer_y(), boyer_x()))
                            ),
                            Cons(
                                Tup(
                                    boyer_equal(boyer_x(), boyer_difference(boyer_x(), boyer_y())),
                                    boyer_or_(
                                        boyer_equal(boyer_x(), boyer_zero()),
                                        boyer_zerop(boyer_y())
                                    )
                                ),
                                Cons(
                                    Tup(
                                        boyer_equal(
                                            boyer_times(boyer_x(), boyer_y()),
                                            boyer_zero()
                                        ),
                                        boyer_or_(boyer_zerop(boyer_x()), boyer_zerop(boyer_y()))
                                    ),
                                    Cons(
                                        Tup(
                                            boyer_equal(
                                                boyer_append_(boyer_x(), boyer_y()),
                                                boyer_append_(boyer_x(), boyer_z())
                                            ),
                                            boyer_equal(boyer_y(), boyer_z())
                                        ),
                                        Cons(
                                            Tup(
                                                boyer_equal(
                                                    boyer_y(),
                                                    boyer_times(boyer_x(), boyer_y())
                                                ),
                                                boyer_or_(
                                                    boyer_equal(boyer_y(), boyer_zero()),
                                                    boyer_equal(boyer_x(), boyer_one())
                                                )
                                            ),
                                            Cons(
                                                Tup(
                                                    boyer_equal(
                                                        boyer_x(),
                                                        boyer_times(boyer_x(), boyer_y())
                                                    ),
                                                    boyer_or_(
                                                        boyer_equal(boyer_x(), boyer_zero()),
                                                        boyer_equal(boyer_y(), boyer_one())
                                                    )
                                                ),
                                                Cons(
                                                    Tup(
                                                        boyer_equal(
                                                            boyer_times(boyer_x(), boyer_y()),
                                                            boyer_one()
                                                        ),
                                                        boyer_and_(
                                                            boyer_equal(boyer_x(), boyer_one()),
                                                            boyer_equal(boyer_y(), boyer_one())
                                                        )
                                                    ),
                                                    Cons(
                                                        Tup(
                                                            boyer_equal(
                                                                boyer_difference(
                                                                    boyer_x(),
                                                                    boyer_y()
                                                                ),
                                                                boyer_difference(
                                                                    boyer_z(),
                                                                    boyer_y()
                                                                )
                                                            ),
                                                            boyer_if_(
                                                                boyer_lessp(boyer_x(), boyer_y()),
                                                                boyer_not_(
                                                                    boyer_lessp(
                                                                        boyer_y(),
                                                                        boyer_z()
                                                                    )
                                                                ),
                                                                boyer_if_(
                                                                    boyer_lessp(
                                                                        boyer_z(),
                                                                        boyer_y()
                                                                    ),
                                                                    boyer_not_(
                                                                        boyer_lessp(
                                                                            boyer_y(),
                                                                            boyer_x()
                                                                        )
                                                                    ),
                                                                    boyer_equal(
                                                                        boyer_x(),
                                                                        boyer_z()
                                                                    )
                                                                )
                                                            )
                                                        ),
                                                        Cons(
                                                            Tup(
                                                                boyer_equal(
                                                                    boyer_lessp(
                                                                        boyer_x(),
                                                                        boyer_y()
                                                                    ),
                                                                    boyer_z()
                                                                ),
                                                                boyer_if_(
                                                                    boyer_lessp(
                                                                        boyer_x(),
                                                                        boyer_y()
                                                                    ),
                                                                    boyer_equal(
                                                                        boyer_true(),
                                                                        boyer_z()
                                                                    ),
                                                                    boyer_equal(
                                                                        boyer_false(),
                                                                        boyer_z()
                                                                    )
                                                                )
                                                            ),
                                                            Nil
                                                        )
                                                    )
                                                )
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
        }
    )
}

def boyer_append_(a: Term, b: Term): Term {
    Func(
        APPEND,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_append_(boyer_append_(boyer_x(), boyer_y()), boyer_z()),
                        boyer_append_(boyer_x(), boyer_append_(boyer_y(), boyer_z()))
                    ),
                    Nil
                )
        }
    )
}

def boyer_x(): Term {
    Var(X)
}

def boyer_y(): Term {
    Var(Y)
}

def boyer_z(): Term {
    Var(Z)
}

def boyer_u(): Term {
    Var(U)
}

def boyer_w(): Term {
    Var(W)
}

def boyer_a(): Term {
    Var(A)
}

def boyer_b(): Term {
    Var(B)
}

def boyer_c(): Term {
    Var(C)
}

def boyer_d(): Term {
    Var(D)
}

def boyer_false(): Term {
    Func(FALSE, Nil, new { apply(u) => Nil })
}

def boyer_true(): Term {
    Func(TRUE, Nil, new { apply(u) => Nil })
}

def boyer_or_(a: Term, b: Term): Term {
    Func(
        OR,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_or_(boyer_x(), boyer_y()),
                        boyer_if_(
                            boyer_x(),
                            boyer_true(),
                            boyer_if_(boyer_y(), boyer_true(), boyer_false())
                        )
                    ),
                    Nil
                )
        }
    )
}

def boyer_lessp(a: Term, b: Term): Term {
    Func(
        LESSP,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_lessp(boyer_remainder(boyer_x(), boyer_y()), boyer_y()),
                        boyer_not_(boyer_zerop(boyer_y()))
                    ),
                    Cons(
                        Tup(
                            boyer_lessp(boyer_quotient(boyer_x(), boyer_y()), boyer_x()),
                            boyer_and_(
                                boyer_not_(boyer_zerop(boyer_x())),
                                boyer_lessp(boyer_one(), boyer_y())
                            )
                        ),
                        Cons(
                            Tup(
                                boyer_lessp(
                                    boyer_plus(boyer_x(), boyer_y()),
                                    boyer_plus(boyer_x(), boyer_z())
                                ),
                                boyer_lessp(boyer_y(), boyer_z())
                            ),
                            Cons(
                                Tup(
                                    boyer_lessp(
                                        boyer_times(boyer_x(), boyer_z()),
                                        boyer_times(boyer_y(), boyer_z())
                                    ),
                                    boyer_and_(
                                        boyer_not_(boyer_zerop(boyer_z())),
                                        boyer_lessp(boyer_x(), boyer_y())
                                    )
                                ),
                                Cons(
                                    Tup(
                                        boyer_lessp(boyer_y(), boyer_plus(boyer_x(), boyer_y())),
                                        boyer_not_(boyer_zerop(boyer_x()))
                                    ),
                                    Nil
                                )
                            )
                        )
                    )
                )
        }
    )
}

def boyer_cons(a: Term, b: Term): Term {
    Func(CONS, Cons(a, Cons(b, Nil)), new { apply(u) => Nil })
}

def boyer_remainder(a: Term, b: Term): Term {
    Func(
        REMAINDER,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(boyer_remainder(boyer_x(), boyer_one()), boyer_zero()),
                    Cons(
                        Tup(boyer_remainder(boyer_x(), boyer_x()), boyer_zero()),
                        Cons(
                            Tup(
                                boyer_remainder(boyer_times(boyer_x(), boyer_y()), boyer_x()),
                                boyer_zero()
                            ),
                            Cons(
                                Tup(
                                    boyer_remainder(boyer_times(boyer_x(), boyer_y()), boyer_y()),
                                    boyer_zero()
                                ),
                                Nil
                            )
                        )
                    )
                )
        }
    )
}

def boyer_quotient(a: Term, b: Term): Term {
    Func(
        QUOTIENT,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_quotient(
                            boyer_plus(boyer_x(), boyer_plus(boyer_x(), boyer_y())),
                            boyer_two()
                        ),
                        boyer_plus(boyer_x(), boyer_quotient(boyer_y(), boyer_two()))
                    ),
                    Cons(
                        Tup(
                            boyer_quotient(boyer_times(boyer_y(), boyer_x()), boyer_y()),
                            boyer_if_(boyer_zerop(boyer_y()), boyer_zero(), boyer_x())
                        ),
                        Nil
                    )
                )
        }
    )
}

def boyer_times(a: Term, b: Term): Term {
    Func(
        TIMES,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_times(boyer_x(), boyer_plus(boyer_y(), boyer_z())),
                        boyer_plus(
                            boyer_times(boyer_x(), boyer_y()),
                            boyer_times(boyer_x(), boyer_z())
                        )
                    ),
                    Cons(
                        Tup(
                            boyer_times(boyer_times(boyer_x(), boyer_y()), boyer_z()),
                            boyer_times(boyer_x(), boyer_times(boyer_y(), boyer_z()))
                        ),
                        Cons(
                            Tup(
                                boyer_times(boyer_x(), boyer_difference(boyer_y(), boyer_z())),
                                boyer_difference(
                                    boyer_times(boyer_y(), boyer_x()),
                                    boyer_times(boyer_z(), boyer_x())
                                )
                            ),
                            Cons(
                                Tup(
                                    boyer_times(boyer_x(), boyer_add1(boyer_y())),
                                    boyer_plus(boyer_x(), boyer_times(boyer_x(), boyer_y()))
                                ),
                                Nil
                            )
                        )
                    )
                )
        }
    )
}

def boyer_difference(a: Term, b: Term): Term {
    Func(
        DIFFERENCE,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(boyer_difference(boyer_x(), boyer_x()), boyer_zero()),
                    Cons(
                        Tup(
                            boyer_difference(boyer_plus(boyer_x(), boyer_y()), boyer_x()),
                            boyer_y()
                        ),
                        Cons(
                            Tup(
                                boyer_difference(boyer_plus(boyer_y(), boyer_x()), boyer_x()),
                                boyer_y()
                            ),
                            Cons(
                                Tup(
                                    boyer_difference(
                                        boyer_plus(boyer_x(), boyer_y()),
                                        boyer_plus(boyer_x(), boyer_z())
                                    ),
                                    boyer_difference(boyer_y(), boyer_z())
                                ),
                                Cons(
                                    Tup(
                                        boyer_difference(
                                            boyer_plus(boyer_y(), boyer_plus(boyer_x(), boyer_z())),
                                            boyer_x()
                                        ),
                                        boyer_plus(boyer_y(), boyer_z())
                                    ),
                                    Cons(
                                        Tup(
                                            boyer_difference(
                                                boyer_add1(boyer_plus(boyer_y(), boyer_z())),
                                                boyer_z()
                                            ),
                                            boyer_add1(boyer_y())
                                        ),
                                        Cons(
                                            Tup(
                                                boyer_difference(
                                                    boyer_add1(boyer_add1(boyer_x())),
                                                    boyer_two()
                                                ),
                                                boyer_x()
                                            ),
                                            Nil
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
        }
    )
}

def boyer_exp_(a: Term, b: Term): Term {
    Func(
        EXP,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_exp_(boyer_x(), boyer_plus(boyer_y(), boyer_z())),
                        boyer_times(
                            boyer_exp_(boyer_x(), boyer_y()),
                            boyer_exp_(boyer_x(), boyer_z())
                        )
                    ),
                    Cons(
                        Tup(
                            boyer_exp_(boyer_x(), boyer_times(boyer_y(), boyer_z())),
                            boyer_exp_(boyer_exp_(boyer_x(), boyer_y()), boyer_z())
                        ),
                        Nil
                    )
                )
        }
    )
}

def boyer_implies(a: Term, b: Term): Term {
    Func(
        IMPLIES,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_implies(boyer_x(), boyer_y()),
                        boyer_if_(
                            boyer_x(),
                            boyer_if_(boyer_y(), boyer_true(), boyer_false()),
                            boyer_true()
                        )
                    ),
                    Nil
                )
        }
    )
}

def boyer_length_(a: Term): Term {
    Func(
        LENGTH,
        Cons(a, Nil),
        new {
            apply(u) =>
                Cons(
                    Tup(boyer_length_(boyer_reverse_(boyer_x())), boyer_length_(boyer_x())),
                    Cons(
                        Tup(
                            boyer_length_(
                                boyer_cons(
                                    boyer_x(),
                                    boyer_cons(
                                        boyer_y(),
                                        boyer_cons(boyer_z(), boyer_cons(boyer_u(), boyer_w()))
                                    )
                                )
                            ),
                            boyer_plus(boyer_four(), boyer_length_(boyer_w()))
                        ),
                        Nil
                    )
                )
        }
    )
}

def boyer_reverse_(a: Term): Term {
    Func(
        REVERSE,
        Cons(a, Nil),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_reverse_(boyer_append_(boyer_x(), boyer_y())),
                        boyer_append_(boyer_reverse_(boyer_y()), boyer_reverse_(boyer_x()))
                    ),
                    Nil
                )
        }
    )
}

def boyer_nil(): Term {
    Func(NIL, Nil, new { apply(u) => Nil })
}

def boyer_member(a: Term, b: Term): Term {
    Func(
        MEMBER,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_member(boyer_x(), boyer_append_(boyer_y(), boyer_z())),
                        boyer_or_(
                            boyer_member(boyer_x(), boyer_y()),
                            boyer_member(boyer_x(), boyer_z())
                        )
                    ),
                    Cons(
                        Tup(
                            boyer_member(boyer_x(), boyer_reverse_(boyer_y())),
                            boyer_member(boyer_x(), boyer_y())
                        ),
                        Nil
                    )
                )
        }
    )
}

def boyer_plus(a: Term, b: Term): Term {
    Func(
        PLUS,
        Cons(a, Cons(b, Nil)),
        new {
            apply(u) =>
                Cons(
                    Tup(
                        boyer_plus(boyer_plus(boyer_x(), boyer_y()), boyer_z()),
                        boyer_plus(boyer_x(), boyer_plus(boyer_y(), boyer_z()))
                    ),
                    Cons(
                        Tup(
                            boyer_plus(
                                boyer_remainder(boyer_x(), boyer_y()),
                                boyer_times(boyer_y(), boyer_quotient(boyer_x(), boyer_y()))
                            ),
                            boyer_x()
                        ),
                        Cons(
                            Tup(
                                boyer_plus(boyer_x(), boyer_add1(boyer_y())),
                                boyer_add1(boyer_plus(boyer_x(), boyer_y()))
                            ),
                            Nil
                        )
                    )
                )
        }
    )
}

def boyer_f(a: Term): Term {
    Func(F, Cons(a, Nil), new { apply(u) => Nil })
}

def one_way_unify1(
    term1: Term,
    term2: Term,
    subst: List[Pair[Id, Term]]
): Pair[Bool, List[Pair[Id, Term]]] {
    term2.case {
        Var(vid2) =>
            find(vid2, subst).case[Bool, Term] {
                Tup(found, v2) =>
                    found.case {
                        True => Tup(term_eq(term1, v2), subst),
                        False => Tup(True, Cons(Tup(vid2, term1), subst))
                    }
            },
        Func(f2, as2, l2) =>
            term1.case {
                Var(vid1) => Tup(False, Nil),
                Func(f1, as1, l2) =>
                    id_eq(f1, f2).case {
                        True => one_way_unify1_lst(as1, as2, subst),
                        False => Tup(False, Nil)
                    },
                ERROR => Tup(False, Nil)
            },
        ERROR => Tup(False, Nil)
    }
}

def one_way_unify1_lst(
    tts1: List[Term],
    tts2: List[Term],
    subst: List[Pair[Id, Term]]
): Pair[Bool, List[Pair[Id, Term]]] {
    tts1.case[Term] {
        Nil =>
            tts2.case[Term] {
                Nil => Tup(True, subst),
                Cons(t, ts) => Tup(False, Nil)
            },
        Cons(t1, ts1) =>
            tts2.case[Term] {
                Nil => Tup(False, Nil),
                Cons(t2, ts2) =>
                    one_way_unify1(t1, t2, subst).case[Bool, List[Pair[Id, Term]]] {
                        Tup(hd_ok, subst_) =>
                            one_way_unify1_lst(ts1, ts2, subst_).case[Bool, List[Pair[Id, Term]]] {
                                Tup(tl_ok, subst__) =>
                                    let is_ok: Bool = hd_ok.case {
                                        True => tl_ok,
                                        False => False
                                    };
                                    Tup(is_ok, subst__)
                            }
                    }
            }
    }
}

def one_way_unify(term1: Term, term2: Term): Pair[Bool, List[Pair[Id, Term]]] {
    one_way_unify1(term1, term2, Nil)
}

def rewrite_with_lemmas(term: Term, lss: List[Pair[Term, Term]]): Term {
    lss.case[Pair[Term, Term]] {
        Nil => term,
        Cons(p, ls) =>
            p.case[Term, Term] {
                Tup(lhs, rhs) =>
                    one_way_unify(term, lhs).case[Bool, List[Pair[Id, Term]]] {
                        Tup(unified, subst) =>
                            unified.case {
                                True => rewrite(apply_subst(subst, rhs)),
                                False => rewrite_with_lemmas(term, ls)
                            }
                    }
            }
    }
}

def rewrite(t: Term): Term {
    t.case {
        Var(v) => Var(v),
        Func(f, args, lemmas) =>
            rewrite_with_lemmas(
                Func(f, map(new { apply(x) => rewrite(x) }, args), lemmas),
                lemmas.apply[Unit, List[Pair[Term, Term]]](Unit)
            ),
        ERROR => ERROR
    }
}

def truep(x: Term, l: List[Term]): Bool {
    x.case {
        Var(v) => term_in_list(x, l),
        Func(t, args, lemmas) =>
            id_eq(t, TRUE).case {
                True => True,
                False => term_in_list(x, l)
            },
        ERROR => term_in_list(x, l)
    }
}

def falsep(x: Term, l: List[Term]): Bool {
    x.case {
        Var(v) => term_in_list(x, l),
        Func(f, args, lemmas) =>
            id_eq(f, FALSE).case {
                True => True,
                False => term_in_list(x, l)
            },
        ERROR => term_in_list(x, l)
    }
}

def tautologyp(x: Term, true_lst: List[Term], false_lst: List[Term]): Bool {
    truep(x, true_lst).case {
        True => True,
        False =>
            falsep(x, false_lst).case {
                True => False,
                False =>
                    x.case {
                        Var(v) => False,
                        Func(if_, args, lemmas) =>
                            args.case[Term] {
                                Nil => False,
                                Cons(cond, conds) =>
                                    conds.case[Term] {
                                        Nil => False,
                                        Cons(t, es) =>
                                            es.case[Term] {
                                                Nil => False,
                                                Cons(e, rst) =>
                                                    rst.case[Term] {
                                                        Nil =>
                                                            id_eq(if_, IF).case {
                                                                True =>
                                                                    truep(cond, true_lst).case {
                                                                        True =>
                                                                            tautologyp(
                                                                                t,
                                                                                true_lst,
                                                                                false_lst
                                                                            ),
                                                                        False =>
                                                                            falsep(
                                                                                cond,
                                                                                false_lst
                                                                            ).case {
                                                                                True =>
                                                                                    tautologyp(
                                                                                        e,
                                                                                        true_lst,
                                                                                        false_lst
                                                                                    ),
                                                                                False =>
                                                                                    tautologyp(
                                                                                        t,
                                                                                        Cons(
                                                                                            cond,
                                                                                            true_lst
                                                                                        ),
                                                                                        false_lst
                                                                                    ).case {
                                                                                        True =>
                                                                                            tautologyp(
                                                                                                e,
                                                                                                true_lst,
                                                                                                Cons(
                                                                                                    cond,
                                                                                                    false_lst
                                                                                                )
                                                                                            ),
                                                                                        False =>
                                                                                            False
                                                                                    }
                                                                            }
                                                                    },
                                                                False => False
                                                            },
                                                        Cons(r, rs) => False
                                                    }
                                            }
                                    }
                            },
                        ERROR => False
                    }
            }
    }
}

def tautp(x: Term): Bool {
    tautologyp(rewrite(x), Nil, Nil)
}

def apply_subst(subst: List[Pair[Id, Term]], t: Term): Term {
    t.case {
        Var(vid) =>
            find(vid, subst).case[Bool, Term] {
                Tup(found, value) =>
                    found.case {
                        True => value,
                        False => Var(vid)
                    }
            },
        Func(f, args, ls) => Func(f, map(new { apply(x) => apply_subst(subst, x) }, args), ls),
        ERROR => ERROR
    }
}

def boyer_subst0(): List[Pair[Id, Term]] {
    Cons(
        Tup(
            X,
            boyer_f(
                boyer_plus(boyer_plus(boyer_a(), boyer_b()), boyer_plus(boyer_c(), boyer_zero()))
            )
        ),
        Cons(
            Tup(
                Y,
                boyer_f(
                    boyer_times(boyer_times(boyer_a(), boyer_b()), boyer_plus(boyer_c(), boyer_d()))
                )
            ),
            Cons(
                Tup(
                    Z,
                    boyer_f(
                        boyer_reverse_(
                            boyer_append_(boyer_append_(boyer_a(), boyer_b()), boyer_nil())
                        )
                    )
                ),
                Cons(
                    Tup(
                        U,
                        boyer_equal(
                            boyer_plus(boyer_a(), boyer_b()),
                            boyer_difference(boyer_x(), boyer_y())
                        )
                    ),
                    Cons(
                        Tup(
                            W,
                            boyer_lessp(
                                boyer_remainder(boyer_a(), boyer_b()),
                                boyer_member(boyer_a(), boyer_length_(boyer_b()))
                            )
                        ),
                        Nil
                    )
                )
            )
        )
    )
}

def boyer_theorem(xxxx: Term): Term {
    boyer_implies(
        boyer_and_(
            boyer_implies(xxxx, boyer_y()),
            boyer_and_(
                boyer_implies(boyer_y(), boyer_z()),
                boyer_and_(boyer_implies(boyer_z(), boyer_u()), boyer_implies(boyer_u(), boyer_w()))
            )
        ),
        boyer_implies(boyer_x(), boyer_w())
    )
}

def test0(xxxx: Term): Bool {
    tautp(apply_subst(boyer_subst0(), boyer_theorem(xxxx)))
}

def test_boyer_nofib(n: i64): Bool {
    all_term(new { apply(t) => test0(t) }, replicate_term(n, Var(X)))
}

def main_loop(iters: i64, n: i64): i64 {
    let res: Bool = test_boyer_nofib(n);
    if iters == 1 {
        res.case {
            True =>
                println_i64(1);
                0,
            False =>
                println_i64(0);
                0
        }
    } else {
        main_loop(iters - 1, n)
    }
}

def main(iters: i64, n: i64): i64 {
    main_loop(iters, n)
}