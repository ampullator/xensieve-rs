use std::collections::VecDeque;

use crate::UnsignedInt;

/// Given a Residual string representation, parse it into two integers.
pub(crate) fn residual_to_ints<T>(value: &str) -> Result<(T, T), &'static str>
where
    T: UnsignedInt,
{
    let parts: Vec<&str> = value.split('@').collect();
    if parts.len() != 2 {
        return Err("Input must contain one '@' character separating two numbers.");
    }
    let m = parts[0]
        .parse::<T>()
        .map_err(|_e| "Residual error parsing modulus")?;
    let s = parts[1]
        .parse::<T>()
        .map_err(|_e| "Residual error parsing shift")?;
    Ok((m, s))
}

/// Operator precedence for `infix_to_postfix`.
#[inline(always)]
fn char_to_precedence(op: char) -> i8 {
    match op {
        '!' => 4,
        '&' => 3,
        '^' => 2,
        '|' => 1,
        _ => 0,
    }
}

/// Utility function to reduce repetition in `infix_to_postfix`.
#[inline(always)]
fn collect_operand(post: &mut VecDeque<String>, operand: &mut String) {
    if !operand.is_empty() {
        post.push_back(operand.clone());
        operand.clear();
    }
}

// Implementation of Shunting yard algorithm for Sieve expressions.
pub(crate) fn infix_to_postfix(expr: &str) -> Result<VecDeque<String>, String> {
    let mut post: VecDeque<String> = VecDeque::new();
    let mut operators: Vec<char> = Vec::new();
    let mut operand: String = String::new();

    for c in expr.chars() {
        match c {
            '0'..='9' | '@' => operand.push(c), // operand characters
            '!' => operators.push(c),
            '|' | '&' | '^' => {
                // all binary operators
                collect_operand(&mut post, &mut operand);
                while let Some(&top) = operators.last() {
                    if top == '(' || char_to_precedence(top) < char_to_precedence(c) {
                        break;
                    }
                    post.push_back(operators.pop().unwrap().to_string())
                }
                operators.push(c);
            }
            '(' => operators.push(c),
            ')' => {
                collect_operand(&mut post, &mut operand);
                while let Some(top) = operators.pop() {
                    if top == '(' {
                        break;
                    }
                    post.push_back(top.to_string())
                }
            }
            _ if c.is_whitespace() => {}
            _ => {
                return Err(format!("Found unsupported operator: {}.", c));
            }
        }
    }
    // get any remaining numbers
    if !operand.is_empty() {
        post.push_back(operand);
    }
    // get any remaining operators
    while let Some(op) = operators.pop() {
        post.push_back(op.to_string());
    }
    Ok(post)
}

// to run cargo test and see stdout:
// % cargo test test_infix_to_rpn_a -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_residual_to_ints_a() {
        assert_eq!(residual_to_ints::<u64>("3@1").unwrap(), (3, 1))
    }

    #[test]
    fn test_residual_to_ints_b() {
        assert_eq!(residual_to_ints::<u64>("9@2").unwrap(), (9, 2))
    }

    #[test]
    fn test_residual_to_ints_c() {
        assert_eq!(residual_to_ints::<u64>("0@5").unwrap(), (0, 5))
    }

    #[test]
    fn test_residual_to_ints_d() {
        assert!(residual_to_ints::<u64>("0").is_err());
    }

    #[test]
    fn test_residual_to_ints_e() {
        assert!(residual_to_ints::<u64>("3@wer").is_err());
    }

    #[test]
    fn test_residual_to_ints_f() {
        assert!(residual_to_ints::<u64>("foo@3").is_err());
    }

    #[test]
    fn test_char_to_precedence_a() {
        assert_eq!(char_to_precedence('!'), 4);
        assert_eq!(char_to_precedence('-'), 0);
        assert_eq!(char_to_precedence('&'), 3);
    }

    #[test]
    fn test_infix_to_postfix_a() {
        let e1 = "!3@1 & 6@2 | !(10@0 | 2@0 | 3@0 )";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["3@1", "!", "6@2", "&", "10@0", "2@0", "|", "3@0", "|", "!", "|"]
        );
    }

    #[test]
    fn test_infix_to_postfix_b() {
        let e1 = "10@0 | 2@0 | 3@0";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "2@0", "|", "3@0", "|"]
        );
    }

    #[test]
    fn test_infix_to_postfix_c() {
        let e1 = "!10@0 | !2@0 | !3@0";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "!", "2@0", "!", "|", "3@0", "!", "|"]
        );
    }

    #[test]
    fn test_infix_to_postfix_d() {
        let e1 = "(10@0 & !2@0) | (!3@0 & 4@2)";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "2@0", "!", "&", "3@0", "!", "4@2", "&", "|"]
        );
    }

    #[test]
    fn test_infix_to_postfix_e() {
        let e1 = "10@0 ^ 10@9";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(px1.iter().collect::<Vec<_>>(), vec!["10@0", "10@9", "^"]);
    }

    #[test]
    fn test_infix_to_postfix_f() {
        let e1 = "10@0 ^ -10@9";
        assert!(infix_to_postfix(e1).is_err());
    }

    #[test]
    fn test_infix_to_postfix_g() {
        let e1 = "10@0 + 10@9";
        assert!(infix_to_postfix(e1).is_err());
    }
}
