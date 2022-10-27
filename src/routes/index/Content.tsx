import { useAppSelector } from "@src/states";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Card } from "./Card";
import styles from "./Content.module.less";

export const Content = () => {
  const [records, setRecords] = useState([]);
  const { reloadState } = useAppSelector((store) => store.record);

  const fetchInfo = async () => {
    const resp = await invoke("select_memo_data", {
      params: { content: "" },
    });
    setRecords(resp);
  };

  useEffect(() => {
    fetchInfo();
  }, [reloadState]);

  return (
    <div className={styles.content}>
      {records.map((i, index) => {
        return <Card record={i} key={i.id || String(index)} />;
      })}
    </div>
  );
};
