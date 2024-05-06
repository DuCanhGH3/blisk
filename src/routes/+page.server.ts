import { parallel } from "$lib/parallel";
import { fail, type Actions } from "@sveltejs/kit";
import * as pdfjs from "pdfjs-dist/legacy/build/pdf.mjs";

export const actions: Actions = {
  async default({ request }) {
    const formData = await request.formData();

    const title = formData.get("title");

    if (!title || typeof title !== "string") {
      return fail(400, { titleError: "Please provide a valid title!" });
    }

    const file = formData.get("file");

    if (!file || typeof file === "string") {
      return fail(400, { fileError: "You must upload a valid file!" });
    }

    const pdfjsFile = await pdfjs.getDocument(await file.arrayBuffer()).promise;

    const data = await parallel(
      20,
      Array.from({ length: pdfjsFile.numPages }, (_, idx) => idx + 1),
      async (i) => {
        const pdfjsPage = await pdfjsFile.getPage(i);

        const pdfjsPageText = await pdfjsPage.getTextContent();

        let finalString = "";

        for (const item of pdfjsPageText.items) {
          if ("str" in item) {
            finalString += item.str + "\n";
          }
        }

        return finalString;
      }
    );

    return { data };
  },
};
