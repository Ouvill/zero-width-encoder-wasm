import { NextPage } from "next";
import Link from "next/link";
import { setZeroWidthSteganographyOnCopy } from "../lib/steganography";
import styles from "../styles/Home.module.css";
import Merosu from "../components/merosu.md";

const Novel: NextPage = () => {
  return (
    <div className={styles.container}>
      <main>
        <article onCopy={setZeroWidthSteganographyOnCopy}>
          <h1>Novel</h1>
          <Merosu />
        </article>
        <Link href={"/detect"}>detect</Link>
      </main>
    </div>
  );
};

export default Novel;
