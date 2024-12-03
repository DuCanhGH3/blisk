import { createHighlighterCore, createJavaScriptRegexEngine } from "shiki";

const langAlias = [
  ["asm", ["assembly"]],
  ["shellscript", ["bash", "sh", "shell", "zsh"]],
  ["typescript", ["ts"]],
  ["javascript", ["js"]],
  ["csharp", ["c#", "cs"]],
  ["python", ["py"]],
  ["ruby", ["rb"]],
  ["rust", ["rs"]],
] as const;

export const permittedLanguages = [
  ...langAlias.flatMap(([key, value]) => [key, ...value]),
  "c",
  "go",
  "java",
  "php",
  "scala",
  "swift",
  "json",
  "tsx",
  "jsx",
  "svelte",
  "html",
  "vue",
] as const;

export const highlighter = await createHighlighterCore({
  themes: [import("./themes/github-dark.js"), import("./themes/github-light.js")],
  langs: [
    import("shiki/langs/asm.mjs"),
    import("shiki/langs/shellscript.mjs"),
    import("shiki/langs/typescript.mjs"),
    import("shiki/langs/javascript.mjs"),
    import("shiki/langs/csharp.mjs"),
    import("shiki/langs/python.mjs"),
    import("shiki/langs/ruby.mjs"),
    import("shiki/langs/rust.mjs"),
    import("shiki/langs/c.mjs"),
    import("shiki/langs/go.mjs"),
    import("shiki/langs/java.mjs"),
    import("shiki/langs/php.mjs"),
    import("shiki/langs/scala.mjs"),
    import("shiki/langs/swift.mjs"),
    import("shiki/langs/json.mjs"),
    import("shiki/langs/tsx.mjs"),
    import("shiki/langs/jsx.mjs"),
    import("shiki/langs/svelte.mjs"),
    import("shiki/langs/html.mjs"),
    import("shiki/langs/vue.mjs"),
  ],
  langAlias: Object.fromEntries(langAlias.flatMap(([original, aliases]) => aliases.map((alias) => [alias, original]))),
  engine: createJavaScriptRegexEngine(),
});
