import { BuiltinSpec } from './types';

export const KEYWORDS = new Set([
    'var', 'const', 'struct', 'impl', 'trait', 'enum', 'if', 'else', 'while', 'for', 'in',
    'break', 'continue', 'return', 'import', 'extern', 'match', 'case', 'as', 'null',
    'true', 'false', 'void', 'int', 'float', 'bool', 'string',
    // vec / Result 类型关键字（lexer.rs 中的独立 token）
    'vec', 'Ok', 'Err'
]);

export const BUILTIN_SPECS: Record<string, BuiltinSpec> = {
    print: {
        signatureLabel: 'print(msg)',
        markdown: '`void print(string msg)`\n\n将字符串打印到标准输出。',
        parameters: ['msg']
    },
    read_file: {
        signatureLabel: 'read_file(path)',
        markdown: '`string! read_file(string path)`\n\n读取文件全部内容。返回可空结果。',
        parameters: ['path']
    },
    write_file: {
        signatureLabel: 'write_file(path, content)',
        markdown: '`void! write_file(string path, string content)`\n\n将内容写入指定文件。',
        parameters: ['path', 'content']
    },
    len: {
        signatureLabel: 'len(s)',
        markdown: '`int len(string s)`\n\n返回字符串长度。',
        parameters: ['s']
    },
    trim: {
        signatureLabel: 'trim(s)',
        markdown: '`string trim(string s)`\n\n去除字符串首尾空白。',
        parameters: ['s']
    },
    split: {
        signatureLabel: 'split(s, sep)',
        markdown: '`vec<string> split(string s, string sep)`\n\n用分隔符拆分字符串。',
        parameters: ['s', 'sep']
    },
    join: {
        signatureLabel: 'join(parts, sep)',
        markdown: '`string join(vec<string> parts, string sep)`\n\n用分隔符连接字符串数组。',
        parameters: ['parts', 'sep']
    },
    substr: {
        signatureLabel: 'substr(s, start, length)',
        markdown: '`string substr(string s, int start, int length)`\n\n提取子字符串。',
        parameters: ['s', 'start', 'length']
    },
    char_to_string: {
        signatureLabel: 'char_to_string(code)',
        markdown: '`string char_to_string(int code)`\n\n将 Unicode 码点转换为字符串（如 65 -> "A"）。',
        parameters: ['code']
    },
    panic: {
        signatureLabel: 'panic(msg)',
        markdown: '`void panic(string msg)`\n\n抛出运行时错误并终止程序。',
        parameters: ['msg']
    },
    format: {
        signatureLabel: 'format(template, args)',
        markdown: '`string format(string template, vec<string> args)`\n\n将占位符 `{}` 依次替换为 args 中的元素。',
        parameters: ['template', 'args']
    }
};

export const BUILTIN_DOCS: Record<string, string> = Object.fromEntries(
    Object.entries(BUILTIN_SPECS).map(([name, spec]) => [name, spec.markdown])
);
