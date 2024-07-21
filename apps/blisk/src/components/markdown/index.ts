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
import TableHead from "./TableHead.svelte";
import TableBody from "./TableBody.svelte";
import TableRow from "./TableRow.svelte";
import TableCell from "./TableCell.svelte";
import List from "./List.svelte";
import ListItem from "./ListItem.svelte";
import Hr from "./Hr.svelte";
import Html from "./Html.svelte";
import Blockquote from "./Blockquote.svelte";
import Code from "./Code.svelte";
import Br from "./Br.svelte";

export const renderers = {
  heading: Heading,
  paragraph: Paragraph,
  text: Text,
  image: Image,
  link: Link,
  em: Em,
  strong: Strong,
  codespan: Codespan,
  del: Del,
  table: Table,
  tablehead: TableHead,
  tablebody: TableBody,
  tablerow: TableRow,
  tablecell: TableCell,
  list: List,
  orderedlistitem: null,
  unorderedlistitem: null,
  listitem: ListItem,
  hr: Hr,
  html: Html,
  blockquote: Blockquote,
  code: Code,
  br: Br,
} as const;
