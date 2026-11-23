//! Ordered working matrices for Core pattern compilation.
//!
//! Each row contains the remaining pattern vector and its statement body.
//! Row order preserves first-match priority.

use std::rc::Rc;

use crate::syntax::{
    AnyPattern, Chirality, Cns, Identifier, NestedClause, PatternVec, Prd, Statement, Term, Ty,
    TypingContext, XVar, arguments::Argument,
};

/// One working row of the backtracking algorithm.
///
/// Its pattern vector has one entry per current subject.
/// The body may be updated when pattern binders are substituted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// Patterns of this row that need to be checked.
    pub patterns: PatternVec,
    /// The statement selected when all patterns in this row match.
    pub body: Rc<Statement>,
}

/// Working rows in their original relative clause order.
pub type Matrix = Vec<Row>;

/// A typed Core atom inspected during pattern compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject {
    Producer(XVar<Prd>),
    Consumer(XVar<Cns>),
}

/// The ordered subject vector A, aligned with every row's pattern vector.
pub type SubjectVec = Vec<Subject>;

impl Subject {
    /// Returns the chirality of the subject atom.
    pub fn chirality(&self) -> Chirality {
        match self {
            Subject::Producer(_) => Chirality::Prd,
            Subject::Consumer(_) => Chirality::Cns,
        }
    }

    /// Returns the stored type annotation of the subject atom.
    pub fn ty(&self) -> &Ty {
        match self {
            Subject::Producer(p) => &p.ty,
            Subject::Consumer(c) => &c.ty,
        }
    }

    /// Returns the identifier of the subject atom.
    pub fn identifier(&self) -> &Identifier {
        match self {
            Subject::Producer(p) => &p.var,
            Subject::Consumer(c) => &c.var,
        }
    }

    /// Represents the subject atom as a Core argument.
    ///
    /// Preserves its identifier, chirality and type.
    pub fn to_argument(&self) -> Argument {
        match self {
            Subject::Producer(p) => {
                let term = Term::XVar(p.clone());

                Argument::Producer(term)
            }
            Subject::Consumer(c) => {
                let term = Term::XVar(c.clone());

                Argument::Consumer(term)
            }
        }
    }
}

/// Converts context bindings into subject atoms in context order.
///
/// Reuses existing identifiers without generating fresh binders.
pub fn atoms(context: &TypingContext) -> SubjectVec {
    let mut res = vec![];

    for binding in &context.bindings {
        match binding.chi {
            Chirality::Prd => res.push(Subject::Producer(XVar {
                prdcns: Prd,
                var: binding.var.clone(),
                ty: binding.ty.clone(),
            })),
            Chirality::Cns => res.push(Subject::Consumer(XVar {
                prdcns: Cns,
                var: binding.var.clone(),
                ty: binding.ty.clone(),
            })),
        }
    }

    res
}

/// Creates the initial one-column working matrix M from nested clauses.
///
/// Preserves clause order and shares the existing statement bodies.
pub fn initial_matrix(clauses: &[NestedClause]) -> Matrix {
    let mut res = vec![];

    for clause in clauses {
        let row = Row {
            patterns: vec![clause.pattern.clone()],
            body: clause.body.clone(),
        };

        res.push(row);
    }

    res
}

/// Classifies a non-empty first pattern column for rule selection.
///
/// Empty matrices and exhausted subject vectors are handled separately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnKind {
    Xtor,
    Irrefutable,
    Mixed,
}

/// Classifies the first pattern column.
///
/// Requires a non-empty matrix with at least one pattern in every row.
pub fn classify_first_column(matrix: &Matrix) -> ColumnKind {
    let mut has_xtor = false;
    let mut has_irrefutable = false;

    for row in matrix {
        let row_irrefutable = match &row.patterns[0] {
            AnyPattern::Producer(p) => p.is_irrefutable(),
            AnyPattern::Consumer(c) => c.is_irrefutable(),
        };
        if row_irrefutable {
            has_irrefutable = true;
        } else {
            has_xtor = true;
        }
    }

    if has_irrefutable && !has_xtor {
        ColumnKind::Irrefutable
    } else if !has_irrefutable && has_xtor {
        ColumnKind::Xtor
    } else {
        ColumnKind::Mixed
    }
}

/// Returns the length of the longest initial group whose first-column
/// patterns are either all irrefutable or all xtor patterns.
///
/// Requires a non-empty matrix with at least one pattern in every row.
pub fn simple_prefix_len(matrix: &Matrix) -> usize {
    let mut res = 1;
    let irrefutable_start = match &matrix[0].patterns[0] {
        AnyPattern::Producer(p) => p.is_irrefutable(),
        AnyPattern::Consumer(c) => c.is_irrefutable(),
    };

    while res < matrix.len() {
        let is_irrefutable = match &matrix[res].patterns[0] {
            AnyPattern::Producer(p) => p.is_irrefutable(),
            AnyPattern::Consumer(c) => c.is_irrefutable(),
        };
        if irrefutable_start == is_irrefutable {
            res += 1;
        } else {
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::*;
    use std::rc::Rc;

    fn id(text: &str) -> Identifier {
        Identifier::new(text.to_string())
    }

    fn nat() -> Ty {
        Ty::Decl(id("Nat"))
    }

    fn stream() -> Ty {
        Ty::Decl(id("Stream"))
    }

    fn body(code: i64) -> Rc<Statement> {
        Rc::new(Statement::Exit(Exit {
            arg: Rc::new(Term::Literal(Literal { lit: code })),
            ty: Ty::I64,
        }))
    }

    fn binder() -> AnyPattern {
        AnyPattern::Producer(Pattern::XVar(XVar {
            prdcns: Prd,
            var: id("x"),
            ty: nat(),
        }))
    }

    fn wildcard() -> AnyPattern {
        AnyPattern::Producer(Pattern::Wildcard {
            chi: Prd,
            ty: nat(),
        })
    }

    fn constructor(name: &str, args: PatternVec) -> AnyPattern {
        AnyPattern::Producer(Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: id(name),
            args,
            ty: nat(),
        }))
    }

    fn zero() -> AnyPattern {
        constructor("Zero", vec![])
    }

    fn succ() -> AnyPattern {
        constructor("Succ", vec![wildcard()])
    }

    fn covariable() -> AnyPattern {
        AnyPattern::Consumer(Pattern::XVar(XVar {
            prdcns: Cns,
            var: id("alpha"),
            ty: stream(),
        }))
    }

    fn cowildcard() -> AnyPattern {
        AnyPattern::Consumer(Pattern::Wildcard {
            chi: Cns,
            ty: stream(),
        })
    }

    fn destructor(name: &str, result_ty: Ty) -> AnyPattern {
        AnyPattern::Consumer(Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: id(name),
            args: vec![AnyPattern::Consumer(Pattern::Wildcard {
                chi: Cns,
                ty: result_ty,
            })],
            ty: stream(),
        }))
    }

    fn head() -> AnyPattern {
        destructor("head", Ty::I64)
    }

    fn tail() -> AnyPattern {
        destructor("tail", stream())
    }

    fn one_column(patterns: Vec<AnyPattern>) -> Matrix {
        patterns
            .into_iter()
            .map(|pattern| Row {
                patterns: vec![pattern],
                body: body(0),
            })
            .collect()
    }

    #[test]
    fn producer_columns_select_rules_and_maximal_prefixes() {
        let cases = vec![
            ("single binder", vec![binder()], ColumnKind::Irrefutable, 1),
            ("single constructor", vec![zero()], ColumnKind::Xtor, 1),
            (
                "binders and wildcards",
                vec![binder(), wildcard()],
                ColumnKind::Irrefutable,
                2,
            ),
            (
                "different constructors",
                vec![zero(), succ(), zero()],
                ColumnKind::Xtor,
                3,
            ),
            (
                "irrefutable prefix",
                vec![binder(), wildcard(), zero(), binder()],
                ColumnKind::Mixed,
                2,
            ),
            (
                "constructor prefix",
                vec![zero(), succ(), binder(), zero()],
                ColumnKind::Mixed,
                2,
            ),
            (
                "immediate change",
                vec![zero(), binder(), zero()],
                ColumnKind::Mixed,
                1,
            ),
        ];

        for (label, patterns, expected_kind, expected_len) in cases {
            let matrix = one_column(patterns);

            assert_eq!(classify_first_column(&matrix), expected_kind, "{label}");
            assert_eq!(simple_prefix_len(&matrix), expected_len, "{label}");
        }
    }

    #[test]
    fn consumer_columns_select_rules_and_maximal_prefixes() {
        let cases = vec![
            ("single destructor", vec![head()], ColumnKind::Xtor, 1),
            (
                "consumer binders",
                vec![covariable(), cowildcard()],
                ColumnKind::Irrefutable,
                2,
            ),
            (
                "different destructors",
                vec![head(), tail()],
                ColumnKind::Xtor,
                2,
            ),
            (
                "consumer prefix",
                vec![covariable(), cowildcard(), head()],
                ColumnKind::Mixed,
                2,
            ),
            (
                "destructor prefix",
                vec![head(), tail(), covariable(), head()],
                ColumnKind::Mixed,
                2,
            ),
        ];

        for (label, patterns, expected_kind, expected_len) in cases {
            let matrix = one_column(patterns);

            assert_eq!(classify_first_column(&matrix), expected_kind, "{label}");
            assert_eq!(simple_prefix_len(&matrix), expected_len, "{label}");
        }
    }

    #[test]
    fn later_columns_do_not_affect_first_column_classification() {
        let matrix = vec![
            Row {
                patterns: vec![binder(), zero()],
                body: body(0),
            },
            Row {
                patterns: vec![wildcard(), succ()],
                body: body(1),
            },
        ];

        assert_eq!(classify_first_column(&matrix), ColumnKind::Irrefutable);
        assert_eq!(simple_prefix_len(&matrix), 2);
    }

    #[test]
    fn initial_matrix_preserves_nested_patterns_order_and_shared_bodies() {
        let clauses = vec![
            NestedClause {
                pattern: constructor("Succ", vec![constructor("Succ", vec![binder()])]),
                body: body(0),
            },
            NestedClause {
                pattern: wildcard(),
                body: body(1),
            },
        ];

        let matrix = initial_matrix(&clauses);

        assert_eq!(matrix.len(), clauses.len());

        for (row, clause) in matrix.iter().zip(&clauses) {
            assert_eq!(row.patterns, vec![clause.pattern.clone()]);
            assert!(Rc::ptr_eq(&row.body, &clause.body));
        }
    }

    #[test]
    fn subject_atoms_preserve_identifiers_and_form_well_typed_call_arguments() {
        let x1 = Identifier {
            name: "x".to_string(),
            id: 7,
        };
        let x2 = Identifier {
            name: "x".to_string(),
            id: 8,
        };
        let alpha = Identifier {
            name: "alpha".to_string(),
            id: 9,
        };

        let context = TypingContext {
            bindings: vec![
                ContextBinding {
                    var: x1.clone(),
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: x2.clone(),
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: alpha.clone(),
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        };

        let subjects = atoms(&context);

        let names: Vec<_> = subjects
            .iter()
            .map(|subject| subject.identifier().clone())
            .collect();

        assert_eq!(names, vec![x1, x2, alpha]);

        let typing: SubjectTyping = subjects
            .iter()
            .map(|subject| SubjectType {
                chi: subject.chirality(),
                ty: subject.ty().clone(),
            })
            .collect();

        assert_eq!(typing, subject_typing(&context));

        let target_context = TypingContext {
            bindings: vec![
                ContextBinding {
                    var: Identifier {
                        name: "a".to_string(),
                        id: 11,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: Identifier {
                        name: "b".to_string(),
                        id: 12,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: Identifier {
                        name: "k".to_string(),
                        id: 13,
                    },
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        };

        let program = Prog {
            defs: vec![
                Def {
                    name: id("main"),
                    context,
                    body: Statement::Call(Call {
                        name: id("target"),
                        args: Arguments {
                            entries: subjects.iter().map(Subject::to_argument).collect(),
                        },
                        ty: Ty::I64,
                    }),
                },
                Def {
                    name: id("target"),
                    context: target_context,
                    body: body(0).as_ref().clone(),
                },
            ],
            data_types: vec![],
            codata_types: vec![],
            max_id: 13,
        };

        assert_eq!(crate::typing::check_program(&program), Ok(()));
    }
}
