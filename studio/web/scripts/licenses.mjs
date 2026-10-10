// Decides whether an SPDX license expression is allowed.
//
// `A OR B` is allowed when any side is, `A AND B` when both are; AND binds
// tighter than OR, and parentheses group. A missing, empty or malformed
// expression is refused, and so is `WITH` (an exception is not on the list).

/** Splits an expression into identifiers, `(` and `)`. */
function tokenize(expression) {
  return expression.match(/\(|\)|[^\s()]+/g) ?? [];
}

/**
 * @param {unknown} expression the `license` field of a package
 * @param {Iterable<string>} allowed the allowed SPDX identifiers
 * @returns {boolean}
 */
export function isLicenseAllowed(expression, allowed) {
  if (typeof expression !== 'string') return false;
  const allowedSet = new Set(allowed);
  const tokens = tokenize(expression);
  let position = 0;

  // or := and ("OR" and)*
  // and := atom ("AND" atom)*
  // atom := "(" or ")" | identifier
  const parseOr = () => {
    let result = parseAnd();
    while (tokens[position] === 'OR') {
      position += 1;
      const right = parseAnd();
      result = result || right;
    }
    return result;
  };
  const parseAnd = () => {
    let result = parseAtom();
    while (tokens[position] === 'AND') {
      position += 1;
      const right = parseAtom();
      result = result && right;
    }
    return result;
  };
  const parseAtom = () => {
    const token = tokens[position];
    if (token === undefined) throw new Error('unexpected end');
    position += 1;
    if (token === '(') {
      const inner = parseOr();
      if (tokens[position] !== ')') throw new Error('missing )');
      position += 1;
      return inner;
    }
    if (token === ')' || token === 'AND' || token === 'OR' || token === 'WITH') {
      throw new Error(`unexpected ${token}`);
    }
    return allowedSet.has(token);
  };

  try {
    const result = parseOr();
    return position === tokens.length && result;
  } catch {
    return false;
  }
}
