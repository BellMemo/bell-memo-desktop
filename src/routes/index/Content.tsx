import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Card } from "./Card";
import styles from './Content.module.less';

export const Content = () => {
  const [records, setRecords] = useState([]);

  const fetchInfo = async () => {
    const resp = await invoke("select_memo_data", {
      params: { content: "" },
    });
    console.log(resp);
    setRecords(resp);
  };

  useEffect(() => {
    fetchInfo();
  }, []);

  return (
    <div className={styles.content}>
      {records.map((i) => {
        return <Card record={i} />;
      })}
    </div>
  );
};
