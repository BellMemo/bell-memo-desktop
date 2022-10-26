import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";

export const Content = () => {
  const fetchInfo = async () => {
    const resp = await invoke("select_memo_data", {
      params: { content: "" },
    });
    console.log(resp);
  };

  useEffect(() => {
    fetchInfo();
  },[])

  return <div><Button onClick={fetchInfo}>更新</Button></div>;
};
