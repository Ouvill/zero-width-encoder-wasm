import { ClipboardEventHandler } from "react";
import { decode, encode } from "@ouvill/zero-width-encoder-wasm";
import Ajv, { JSONSchemaType } from "ajv";

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
  const steganography = encode(hidden);
  const center = text.length / 2;
  return text.slice(0, center) + steganography + text.slice(center);
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
  const regex = /[\u200B\u200C]+/g;
  const steganographyData = regex.exec(data);
  let decrypted: string[] = [];
  if (steganographyData) {
    for (let item of steganographyData) {
      try {
        const decoded = decode(item);
        decrypted.push(decoded);
      } catch (e) {
        console.log(e);
      }
    }
  }
  return decrypted;
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
