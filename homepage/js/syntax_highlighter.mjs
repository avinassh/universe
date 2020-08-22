import { escapeHtml } from './utils.mjs';

class LanguageModel {
  constructor() {
    this.line = "";
    this.index = 0;
    this.keywords = new Set([]);
  }

  next() {
    const ch = this.peek();
    this.index++;
    return ch;
  }

  peek() {
    if(this.index >= this.line.length) {
      return '';
    } else {
      return this.line[this.index];
    }
  }

  isStringDelimiter(ch) {
    return ch == '\'' || ch == '"' || ch == '`'
  }

  isCommentCharacter(ch) {
    return false;
  }

  isInterminableComment(chs) {
    return false;
  }

  isAlphanumeric(ch) {
    const code = ch.charCodeAt(0);
    if(code >= 48 && code <= 57) return true;
    if(code >= 97 && code <= 122) return true;
    if(code >= 65 && code <= 90) return true;

    return false;
  }

  takeUntil(delimiter) {
    let acc = "";
    let ch = this.next();
    let escaped = false;
    let loops = 0;
    while(ch != '' && loops < 100) {
      loops++;
      if(ch == delimiter && !escaped) {
        break;
      }

      escaped = ch == '\'';
      acc += ch;

      ch = this.next();
    }

    return acc;
  }

  extractSyntax(line) {
    this.line = line;
    this.index = 0;

    let output = "";
    let ch = this.next();
    let loops = 0;
    let acc = "";
    let commentAcc = "";
    while(ch != '' && loops < 100) {
      loops++;
      if(this.isStringDelimiter(ch)) {
        const str = this.takeUntil(ch);
        output += `<span class='str'>${escapeHtml(ch + str + ch)}</span>`;
        ch = this.next();
        continue;
      }

      if(this.isCommentCharacter(ch)) {
        commentAcc += ch;
        if(this.isInterminableComment(commentAcc)) {
          let remainder = "";
          ch = this.next();
          while(ch != '') {
            ch = this.next();
            remainder += ch;
          }
          
          output += `<span class='comment'>${escapeHtml(commentAcc + remainder)}</span>`;
          return output;
        }
        continue;
      } else if(commentAcc.length > 0) {
        output += commentAcc;
        commentAcc = "";
      }

      if(this.isAlphanumeric(ch)) {
        acc += ch;
      } else if(acc.length > 0) {
        if (this.keywords.has(acc)) {
          output += `<span class='keyword'>${escapeHtml(acc)}</span>${escapeHtml(ch)}`;
        } else {
          output += escapeHtml(acc + ch);
        }
        acc = "";
      } else {
        output += escapeHtml(ch);
      }

      ch = this.next();
    }

    return output + escapeHtml(acc);
  }
}

class RustLanguageModel extends LanguageModel {
  constructor() {
    super();

    this.keywords = new Set([
      'as', 'break', 'const', 'continue', 'crate', 'else', 'enum',
      'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let',
      'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
      'self', 'Self', 'static', 'struct', 'super', 'trait', 'true',
      'type', 'unsafe', 'use', 'where', 'while', 'async', 'await',
      'dyn', 'u8', 'u16', 'u32', 'u64', 'u128', 'i8', 'i16', 'i32',
      'i64', 'i128', 'f16', 'f32', 'f64', 'f128'
    ]);
  }

  isCommentCharacter(ch) {
    return ch == '/' || ch == '*'
  }

  isInterminableComment(chs) {
    return chs == '//'
  }
}

class JavascriptLanguageModel extends LanguageModel {
  constructor() {
    super();

    this.keywords = new Set([
      'abstract','arguments','await','boolean',
      'break','byte','case','catch',
      'char', 'class', 'const', 'continue',
      'debugger','default', 'delete','do',
      'double','else','enum','eval',
      'export','extends','false', 'final',
      'finally', 'float', 'for', 'function', 
      'goto', 'if', 'implements', 'import', 
      'in', 'instanceof', 'int', 'interface', 
      'let', 'long', 'native', 'new', 
      'null', 'package', 'private', 'protected', 
      'public', 'return', 'short', 'static', 
      'super', 'switch', 'synchronized', 'this', 
      'throw', 'throws', 'transient', 'true', 
      'try', 'typeof', 'var', 'void', 
      'volatile', 'while', 'with', 'yield'
    ])
  }

  isCommentCharacter(ch) {
    return ch == '/' || ch == '*'
  }

  isInterminableComment(chs) {
    return chs == '//'
  }
}

class BazelLanguageModel extends LanguageModel {
  isCommentCharacter(ch) {
    return ch == '#'
  }

  isInterminableComment(chs) {
    return chs == '#'
  }
}

export default function getLanguageModel(lang) {
  const models = {
     'bazel': BazelLanguageModel,
     'rust': RustLanguageModel,
     'javascript': JavascriptLanguageModel,
  };

  if(models[lang]) {
    return new models[lang]();
  }
  return new LanguageModel();
};
