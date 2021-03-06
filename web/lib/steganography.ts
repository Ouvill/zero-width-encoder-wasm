import { ClipboardEventHandler } from "react";
import Ajv, { JSONSchemaType } from "ajv";
const { encode, decode, detect, embed } = await import(
  "@ouvill/zero-width-encoder-wasm"
);

const ajv = new Ajv();

export type EmbeddedData = {
  original: string;
  href: string;
  date: string;
  version: "1.0.0";
};

const schema: JSONSchemaType<EmbeddedData> = {
  type: "object",
  properties: {
    original: { type: "string" },
    href: { type: "string" },
    date: { type: "string" },
    version: { type: "string" },
  },
  required: ["original", "href", "date", "version"],
};

const validate = ajv.compile(schema);

export const embedSteganography = (text: string, hidden: string) => {
  return embed(text, hidden);
};

export const setZeroWidthSteganographyOnCopy: ClipboardEventHandler = (
  event
) => {
  const selection = document.getSelection();
  if (event.clipboardData && selection && selection.type === "Range") {
    const text = selection.toString();
    if (text) {
      const data: EmbeddedData = {
        original: text,
        href: location.href,
        date: new Date().toISOString(),
        version: "1.0.0",
      };
      const json = JSON.stringify(data);

      const embedded = embedSteganography(text, json);
      event.clipboardData.setData("text/plain", embedded);
      event.preventDefault();
    }
  }
};

export const detectSteganography = (data: string): string[] => {
  return detect(data);
};

export const parse = (json: string[]): EmbeddedData[] => {
  let data: EmbeddedData[] = [];
  json.forEach((item) => {
    try {
      const obj: EmbeddedData | unknown = JSON.parse(item);
      if (validate(obj)) {
        data.push(obj);
      }
    } catch (e) {
      console.log(e);
    }
  });
  return data;
};
