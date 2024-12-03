import type { MarkedToken } from "marked";
import { marked } from "marked";
import type { Component } from "svelte";
import Heading from "./Heading.svelte";
import Paragraph from "./Paragraph.svelte";
import Text from "./Text.svelte";
import Image from "./Image.svelte";
import Link from "./Link.svelte";
import Em from "./Em.svelte";
import Del from "./Del.svelte";
import Codespan from "./Codespan.svelte";
import Strong from "./Strong.svelte";
import Table from "./Table.svelte";
import List from "./List.svelte";
import ListItem from "./ListItem.svelte";
import Hr from "./Hr.svelte";
import Html from "./Html.svelte";
import Blockquote from "./Blockquote.svelte";
import Code from "./Code.svelte";
import Br from "./Br.svelte";
import type { Tokens } from "./types";

export const renderers = {
  heading: Heading,
  paragraph: Paragraph,
  escape: Text,
  text: Text,
  image: Image,
  link: Link,
  em: Em,
  strong: Strong,
  codespan: Codespan,
  del: Del,
  table: Table,
  list: List,
  list_item: ListItem,
  hr: Hr,
  html: Html,
  blockquote: Blockquote,
  code: Code,
  br: Br,
} satisfies Partial<Record<MarkedToken["type"], Component<any>>>;

marked.use({
  tokenizer: {
    codespan(src: string): Tokens.Codespan | undefined {
      const cap = this.rules.inline.code.exec(src);
      if (cap) {
        let text = cap[2].replace(/\n/g, " ");
        const hasNonSpaceChars = /[^ ]/.test(text);
        const hasSpaceCharsOnBothEnds = /^ /.test(text) && / $/.test(text);
        if (hasNonSpaceChars && hasSpaceCharsOnBothEnds) {
          text = text.substring(1, text.length - 1);
        }
        return {
          type: "codespan",
          raw: cap[0],
          text,
        };
      }
    },
    inlineText(src: string): Tokens.Text | undefined {
      const cap = this.rules.inline.text.exec(src);
      if (cap) {
        return {
          type: "text",
          raw: cap[0],
          text: cap[0],
        };
      }
    },
  },
});

export { marked as markdown };
