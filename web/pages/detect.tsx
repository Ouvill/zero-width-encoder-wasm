import { NextPage } from "next";
import React, { useMemo } from "react";
import { detectSteganography, parse } from "../lib/steganography";
import styles from "../styles/Home.module.css";

const Detect: NextPage = () => {
  const [value, setValue] = React.useState("");
  const detected = useMemo(() => detectSteganography(value), [value]);
  const data = useMemo(() => parse(detected), [detected]);

  return (
    <div className={styles.container}>
      <h1>Detect</h1>
      <textarea
        value={value}
        onChange={(e) => {
          setValue(e.target.value);
        }}
      />
      {data.map((item) => {
        return (
          <div key={item.original}>
            <p>copy from: {item.href}</p>
            <p>original text: {item.original}</p>
            <p>date: {item.date}</p>
          </div>
        );
      })}
    </div>
  );
};

export default Detect;
