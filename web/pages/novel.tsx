import { NextPage } from "next";
import Link from "next/link";
import {
  detectSteganography,
  parse,
  setZeroWidthSteganographyOnCopy,
  SteganographyData,
} from "../lib/steganography";
import styles from "../styles/Home.module.css";
import Merosu from "../components/merosu.md";
import React, { useCallback, useMemo } from "react";
import {
  Button,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  TextField,
  Typography,
} from "@mui/material";

const Detect = () => {
  const [value, setValue] = React.useState("");
  const [result, setResult] = React.useState<SteganographyData[]>([]);
  const detect = useCallback(() => {
    const detected = detectSteganography(value);
    const parsed = parse(detected);
    setResult(parsed);
  }, [value]);
  return (
    <div>
      <div>
        <Typography variant={"h6"}>検出器</Typography>
        <Typography variant={"caption"}>
          上の文章をコピーしてみてください。
        </Typography>
      </div>
      <TextField
        value={value}
        onChange={(e) => {
          setValue(e.target.value);
        }}
        multiline={true}
        rows={6}
        fullWidth={true}
      />
      <Button onClick={detect} variant={"contained"}>
        検出
      </Button>

      {result.length > 0 && (
        <div>
          <Typography>コピペを検出しました。</Typography>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>項目</TableCell>
                  <TableCell>値</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                <TableRow>
                  <TableCell>日付</TableCell>
                  <TableCell>{result[0].date}</TableCell>
                </TableRow>
                <TableRow>
                  <TableCell>オリジナル文章</TableCell>
                  <TableCell>{result[0].original}</TableCell>
                </TableRow>
                <TableRow>
                  <TableCell>URL</TableCell>
                  <TableCell>{result[0].href}</TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </TableContainer>
        </div>
      )}
    </div>
  );
};

const Novel: NextPage = () => {
  return (
    <div className={styles.container}>
      <main>
        <article onCopy={setZeroWidthSteganographyOnCopy}>
          <h1>コピペ検出</h1>
          <Typography variant={"caption"}>
            コピーしたときにクリップボードに情報を埋め込む
          </Typography>
          <Merosu />
        </article>
        <Detect />
      </main>
    </div>
  );
};

export default Novel;
